use crate::key::Key;
use crate::musical_time::{Changing, NonZeroInstant, Signature};
use crate::real_time::Tempo;
use getset::CloneGetters;
use std::cmp::min;
use std::iter::from_fn;
use std::sync::Arc;

/// Settings for a project.
#[derive(Clone, Eq, PartialEq, Debug, Default, CloneGetters)]
pub struct Settings {
    // TODO: continuous change
    /// The tempo of the project
    #[get_clone = "pub"]
    pub tempo: Arc<Changing<Tempo>>,
    /// The time signature of the project.
    #[get_clone = "pub"]
    pub time_signature: Arc<Changing<Signature>>,
    /// The key of the project.
    #[get_clone = "pub"]
    pub key: Arc<Changing<Key>>,
}

impl Settings {
    pub(crate) fn time_changes(&self) -> impl Iterator<Item = (NonZeroInstant, Tempo, Signature)> {
        let mut tempo_iter = self.tempo.changes.iter();
        let mut time_signature_iter = self.time_signature.changes.iter();

        let mut current_tempo = self.tempo.start;
        let mut current_time_signature = self.time_signature.start;

        let mut next_tempo = tempo_iter.next();
        let mut next_time_signature = time_signature_iter.next();

        from_fn(move || match (next_tempo, next_time_signature) {
            (Some((tempo_change, tempo)), Some((time_signature_change, time_signature))) => {
                let change = min(tempo_change, time_signature_change);

                if change == tempo_change {
                    next_tempo = tempo_iter.next();
                    current_tempo = *tempo;
                }
                if change == time_signature_change {
                    next_time_signature = time_signature_iter.next();
                    current_time_signature = *time_signature;
                }

                Some((*change, current_tempo, current_time_signature))
            }
            (Some((change, tempo)), None) => {
                next_tempo = tempo_iter.next();
                current_tempo = *tempo;
                Some((*change, current_tempo, current_time_signature))
            }
            (None, Some((change, time_signature))) => {
                next_time_signature = time_signature_iter.next();
                current_time_signature = *time_signature;
                Some((*change, current_tempo, current_time_signature))
            }
            (None, None) => None,
        })
    }
}
