use crate::sync::ArcCell;
use crate::view::{Alignment, OnClick, Quotated, ToText as _};
use crate::{ToArcStr as _, UserInterface, View};
use closure::closure;
use std::fs::DirEntry;
use std::path::Path;
use std::sync::Arc;

// TODO: make pretty
// TODO: add functionality
//  - hide hidden files (dot files)
//  - type in path
//  - go back (..)
//  - keyboard navigation
//  - mkdir
/// Constructs a new file selector.
pub fn file_selector<Ui: UserInterface>(selected_file: &Arc<ArcCell<Path>>) -> View {
    View::Generator(Box::new(generator::<Ui>(selected_file)))
}

fn generator<Ui: UserInterface>(
    selected_file: &Arc<ArcCell<Path>>,
) -> impl Fn() -> View + Send + Sync + 'static {
    closure!([clone selected_file] move || {
        list::<Ui>(&selected_file)
            .bordered()
            .hard_titled::<Ui>(selected_file.get().display().to_arc_str())
    })
}

fn list<Ui: UserInterface>(selected_file: &Arc<ArcCell<Path>>) -> View {
    let path = selected_file.get();

    let directory = if path.is_dir() {
        &path
    } else {
        path.parent().unwrap_or(&path)
    };

    let Ok(directory) = directory.read_dir() else {
        return View::Empty;
    };

    View::y_stack(
        directory
            .filter_map(Result::ok)
            .map(|entry| view_entry::<Ui>(entry, Arc::clone(selected_file))),
    )
}

fn view_entry<Ui: UserInterface>(entry: DirEntry, selected_file: Arc<ArcCell<Path>>) -> Quotated {
    // TODO: add icon
    // TODO: add selection status
    entry
        .file_name()
        .to_string_lossy()
        .to_arc_str()
        .aligned_to(Alignment::Left)
        .on_click(OnClick::new(move |_, _, _| {
            selected_file.set(Arc::from(entry.path()));
        }))
        .quotated_minimally::<Ui>()
}
