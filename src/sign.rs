use std::fmt::{Display, Formatter, Write};
use strum::VariantArray;

pub const SHARP: char = '\u{266F}';
pub const FLAT: char = '\u{266D}';

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, VariantArray)]
pub enum Sign {
    #[default]
    Sharp,
    Flat,
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Sharp => f.write_char(SHARP),
            Sign::Flat => f.write_char(FLAT),
        }
    }
}
