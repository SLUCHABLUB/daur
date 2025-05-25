mod non_zero;

pub use non_zero::NonZeroInstant;

use crate::metre::Duration;
use crate::ui::{Grid, Length};
use crate::{project, time};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// An instant in musical time.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Instant {
    /// The duration since the starting point.
    pub since_start: Duration,
}

impl Instant {
    /// The starting point.
    pub const START: Instant = Instant {
        since_start: Duration::ZERO,
    };

    pub(crate) fn to_x_offset(self, project_settings: &project::Settings, grid: Grid) -> Length {
        let mut remaining = self.since_start;
        let mut offset = Length::ZERO;

        let mut bar = project_settings.time_signature.first_bar();

        loop {
            if remaining < bar.duration() {
                let cell_count = remaining / grid.cell_duration;
                offset += grid.cell_width.get() * cell_count;

                break;
            }

            remaining -= bar.duration();
            offset += bar.width(grid);

            bar = bar.next(project_settings);
        }

        offset
    }

    /// Constructs a new instant from an x offset from the left of the overview.
    #[must_use]
    pub fn from_x_offset(
        mut offset: Length,
        project_settings: &project::Settings,
        grid: Grid,
    ) -> Instant {
        let mut instant = Instant::START;

        let mut bar = project_settings.time_signature.first_bar();

        loop {
            let bar_width = bar.width(grid);

            if offset < bar_width {
                let cell_count = offset / grid.cell_width;
                instant += grid.cell_duration.get() * cell_count;

                break;
            }

            offset -= bar_width;
            instant += bar.duration();

            bar = bar.next(project_settings);
        }

        instant
    }

    // TODO: round middle down
    /// Like [`Instant::from_x_offset`] but the instant is quantised to the [grid](Grid).
    #[must_use]
    pub fn quantised_from_x_offset(
        mut offset: Length,
        project_settings: &project::Settings,
        grid: Grid,
    ) -> Instant {
        let mut instant = Instant::START;

        let mut bar = project_settings.time_signature.first_bar();

        loop {
            let bar_width = bar.width(grid);

            if offset < bar_width {
                let cell_count = offset / grid.cell_width;
                instant += grid.cell_duration.get() * cell_count.rounded();

                break;
            }

            offset -= bar_width;
            instant += bar.duration();

            bar = bar.next(project_settings);
        }

        instant
    }

    /// Converts the instant to real time.
    #[must_use]
    pub fn to_real_time(self, project_settings: &project::Settings) -> time::Instant {
        let mut instant = time::Instant::START;

        let mut change = Instant::START;
        let mut tempo = project_settings.tempo.start;
        let mut time_signature = project_settings.time_signature.start;

        for (next_change, next_tempo, next_time_signature) in project_settings.time_changes() {
            if self < next_change.get() {
                break;
            }

            let duration = next_change.get() - change;
            instant += tempo.beat_duration().get() * (duration / time_signature.beat_duration());

            change = next_change.get();
            tempo = next_tempo;
            time_signature = next_time_signature;
        }

        let remaining = self - change;
        instant += tempo.beat_duration().get() * (remaining / time_signature.beat_duration());

        instant
    }
}

// TODO: derive
impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(mut self, rhs: Duration) -> Instant {
        self += rhs;
        self
    }
}

// TODO: derive
impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.since_start += rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    fn sub(mut self, rhs: Duration) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.since_start -= rhs;
    }
}

// TODO: derive
impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.since_start - rhs.since_start
    }
}
