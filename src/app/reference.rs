use crate::app::atomic::{pack_position, pack_rect, unpack_position, unpack_rect};
use crate::app::App;
use ratatui::layout::{Position, Rect};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

pub struct AppShare {
    lock: RwLock<App>,
    mouse_position: AtomicU32,
    area: AtomicU64,
    redraw: AtomicBool,
    exit: AtomicBool,
}

impl AppShare {
    pub fn new(app: App) -> Arc<Self> {
        Arc::new(AppShare {
            lock: RwLock::new(app),
            mouse_position: AtomicU32::default(),
            area: AtomicU64::default(),
            redraw: AtomicBool::new(true),
            exit: AtomicBool::new(false),
        })
    }

    pub fn read_lock(&self) -> RwLockReadGuard<App> {
        self.lock.read().expect("App lock should not be poisoned")
    }

    pub fn write_lock(&self) -> RwLockWriteGuard<App> {
        self.lock.write().expect("App lock should not be poisoned")
    }

    pub fn mouse_position(&self) -> Position {
        unpack_position(self.mouse_position.load(Ordering::Relaxed))
    }

    pub fn set_mouse_position(&self, mouse_position: Position) {
        self.mouse_position
            .store(pack_position(mouse_position), Ordering::Relaxed);
    }

    pub fn area(&self) -> Rect {
        unpack_rect(self.area.load(Ordering::Relaxed))
    }

    pub fn set_area(&self, area: Rect) {
        self.area.store(pack_rect(area), Ordering::Relaxed);
    }

    pub fn should_redraw(&self) -> bool {
        self.redraw.load(Ordering::Relaxed)
    }

    pub fn should_exit(&self) -> bool {
        self.exit.load(Ordering::Relaxed)
    }

    pub fn set_exit(&self) {
        self.exit.store(true, Ordering::Relaxed);
    }
}
