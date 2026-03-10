use std::time::Duration;

use ordered_float::NotNan;

use crate::{Run, Solution};

pub struct Analysis<'a> {
    score_sum: NotNan<f64>,
    time_sum: Duration,
    run: &'a Run<'a>,
}

impl<'a> Analysis<'a> {
    pub fn new(run: &'a Run) -> Self {
        let score_sum = run.solutions().iter().map(|(_, s, _)| s.score()).sum();
        let time_sum = run.solutions().iter().map(|(_, _, t)| t).sum();

        Self {
            score_sum,
            time_sum,
            run,
        }
    }

    pub fn average_score(&self) -> f64 {
        *self.score_sum / self.run.opts().executions as f64
    }

    pub fn average_time(&self) -> Duration {
        self.time_sum / self.run.opts().executions
    }

    /// Get a reference to the run's best.
    pub fn best(&self) -> &(u32, Solution<'a>, Duration) {
        self.run
            .solutions()
            .iter()
            .max_by_key(|(_, s, _)| s.score())
            .unwrap()
    }

    pub fn gap(&self, optimal: f64) -> f64 {
        if optimal.is_nan() {
            f64::NAN
        } else {
            *(self.best().1.score() - optimal) / optimal * 100.0
        }
    }
}
