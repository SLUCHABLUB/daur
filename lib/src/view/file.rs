//! Functions for creating file related views.

use crate::Id;
use crate::Popup;
use crate::View;
use crate::app::Action;
use crate::string::ToArcStr as _;
use crate::sync::ArcCell;
use crate::view::Alignment;
use crate::view::Axis;
use crate::view::CANCEL;
use crate::view::CONFIRM;
use crate::view::OnClick;
use crate::view::Quoted;
use crate::view::ToText as _;
use std::env::current_dir;
use std::fs::DirEntry;
use std::path::Path;
use std::sync::Arc;

/// Constructs a new file picker.
pub fn picker<Confirm>(confirm: Confirm, cancel: OnClick) -> View
where
    Confirm: Fn(Arc<Path>) -> Action + Send + Sync + 'static,
{
    let selected_file = Arc::new(ArcCell::new(Arc::from(current_dir().unwrap_or_default())));

    let file = Arc::clone(&selected_file);

    let confirm = View::standard_button(CONFIRM, OnClick::action(move || confirm(file.get())));
    let cancel = CANCEL.centred().bordered().on_click(cancel);

    let buttons = View::minimal_stack(Axis::X, vec![cancel, confirm]);

    View::y_stack([
        selector(selected_file).fill_remaining(),
        buttons.quoted_minimally(),
    ])
}

/// Constructs a new file picker that closes a popup when "cancel" is pressed.
pub fn picker_in_popup<Confirm>(confirm: Confirm, popup: Id<Popup>) -> View
where
    Confirm: Fn(Arc<Path>) -> Action + Send + Sync + 'static,
{
    picker(confirm, OnClick::action(move || Action::ClosePopup(popup)))
}

// TODO: make pretty
// TODO: add functionality
//  - hide hidden files (dot files)
//  - type in path
//  - go back (..)
//  - keyboard navigation
//  - mkdir
/// Constructs a new file selector.
pub fn selector(selected_file: Arc<ArcCell<Path>>) -> View {
    View::reactive(move |_| {
        list(&selected_file).bordered_with_title(selected_file.get().display().to_arc_str())
    })
}

/// Constructs a file-list view.
///
/// If the selected file is a directory, this directory's files are used.
/// Otherwise, the parent of said file is.
fn list(selected_file: &Arc<ArcCell<Path>>) -> View {
    let path = selected_file.get();

    let directory = if path.is_dir() {
        &path
    } else {
        path.parent().unwrap_or(&path)
    };

    let Ok(directory) = directory.read_dir() else {
        return View::Empty;
    };

    // TODO: Should we collect rather than filter out errors?
    View::y_stack(
        directory
            .filter_map(Result::ok)
            .map(|entry| view_entry(entry, Arc::clone(selected_file))),
    )
}

/// Constructs a view of a [directory entry](DirEntry).
fn view_entry(entry: DirEntry, selected_file: Arc<ArcCell<Path>>) -> Quoted {
    // TODO: add icon
    // TODO: add selection status
    entry
        .file_name()
        .to_string_lossy()
        .to_arc_str()
        .aligned_to(Alignment::Left)
        .on_click(OnClick::new(move |_, _| {
            selected_file.set(Arc::from(entry.path()));
        }))
        .quoted_minimally()
}
