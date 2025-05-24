/// An instant in sample time. A sample index.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SampleInstant {
    /// The sample index.
    pub index: usize,
}
