use bytemuck::cast;
use ratatui::layout::{Position, Rect};

pub fn pack_rect(rect: Rect) -> u64 {
    cast([rect.x, rect.y, rect.width, rect.height])
}

pub fn unpack_rect(packed: u64) -> Rect {
    let [x, y, width, height]: [u16; 4] = cast(packed);
    Rect {
        x,
        y,
        width,
        height,
    }
}

pub fn pack_position(position: Position) -> u32 {
    cast([position.x, position.y])
}

pub fn unpack_position(packed: u32) -> Position {
    let [x, y]: [u16; 2] = cast(packed);
    Position { x, y }
}
