//! Items pertaining to [`Node`].

// TODO: clean up this total mess of premature abstractions.

pub(crate) mod chain;
mod process_result;

#[doc(inline)]
pub(crate) use chain::Chain;
pub(crate) use process_result::ProcessResult;

/// A node in a [chain](Chain) of audio processing.
#[expect(unused, reason = "this is needed for the module docs")]
pub(crate) enum Node {}
