use crate::sync::ArcCell;
use crate::view::{Alignment, OnClick, Quotated, ToText as _};
use crate::{ToArcStr as _, View};
use alloc::sync::Arc;
use closure::closure;
use std::fs::DirEntry;
use std::path::Path;

// TODO: make pretty
// TODO: add functionality
//  - hide hidden files (dot files)
//  - type in path
//  - go back (..)
//  - keyboard navigation
//  - mkdir
/// Constructs a new file selector.
pub fn file_selector(selected_file: &Arc<ArcCell<Path>>) -> View {
    View::Generator(Box::new(generator(selected_file)))
}

fn generator(selected_file: &Arc<ArcCell<Path>>) -> impl Fn() -> View + Send + Sync + 'static {
    closure!([clone selected_file] move || {
        list(&selected_file)
            .bordered()
            .titled_non_cropping(selected_file.get().display().to_arc_str())
    })
}

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

    View::y_stack(
        directory
            .filter_map(Result::ok)
            .map(|entry| view_entry(entry, Arc::clone(selected_file))),
    )
}

fn view_entry(entry: DirEntry, selected_file: Arc<ArcCell<Path>>) -> Quotated {
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
        .quotated_minimally()
}
