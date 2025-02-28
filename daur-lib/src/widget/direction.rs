/// A direction in which items can be laid out
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Direction {
    /// From the bottom to the top
    Up,
    /// From the right to the left
    Left,
    /// From the top to the bottom
    Down,
    /// From the left to the right
    Right,
}
