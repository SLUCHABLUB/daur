//! Items pertaining to [`Chain`].

mod instance;

use crate::audio::sample;
pub(crate) use instance::Instance;

/// A chain of Nodes
pub(crate) struct Chain {
    // TODO: add a Dag<Node, NodeConnection>
    /// The nodes in the chain.
    _nodes: (),
}

impl Chain {
    /// Create a [instance](Instance) from the chain.
    pub fn instantiate(&self, sample_rate: sample::Rate) -> Instance {
        let _: &Chain = self;

        Instance::new(sample_rate)
    }
}

#[expect(
    clippy::derivable_impls,
    reason = "the real implementation will not be derivable"
)]
impl Default for Chain {
    fn default() -> Chain {
        Chain { _nodes: () }
    }
}
