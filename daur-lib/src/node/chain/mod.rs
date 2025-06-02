mod instance;

use crate::audio::sample;
pub(crate) use instance::Instance;

#[cfg_attr(doc, doc(hidden))]
pub(crate) struct Chain {
    // TODO: add a Dag<Node, NodeConnection>
    _nodes: (),
}

impl Chain {
    #[expect(clippy::unused_self, reason = "todo")]
    pub fn instantiate(&self, sample_rate: sample::Rate) -> Instance {
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
