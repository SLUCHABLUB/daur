#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ScreenLength(pub u16);

impl ScreenLength {
    pub fn get(self) -> u16 {
        self.0
    }
}
