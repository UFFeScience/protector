// TODO remove this if this module will not be used
#![allow(dead_code)]

use std::{cmp::Ordering, str::FromStr};

use rand::Rng;
use thiserror::Error;

pub use super::selection_control::SelectionControl;

#[derive(Debug)]
pub struct GraspSelection<'a, T, R> {
    elements: Vec<T>,
    selection: SelectionControl,
    rng: &'a mut R,
}

impl<'a, T, R: Rng> GraspSelection<'a, T, R> {
    /// Creates a GraspSelection.
    ///
    /// It takes Alpha, a slice of the elements and a Greedy Criteria, which is used to compare the elements
    /// and discover which one is better for the greedy approach.
    pub fn new<I, F>(alpha: Alpha, elements: I, greedy_criteria: F, rng: &'a mut R) -> Self
    where
        I: Iterator<Item = T>,
        F: Fn(&T) -> f64,
    {
        let mut elements = elements.collect::<Vec<_>>();
        elements.sort_by(|a, b| Self::cmp_f64(greedy_criteria(a), greedy_criteria(b)).reverse());

        let worst_cost = greedy_criteria(elements.last().unwrap());
        let best_cost = greedy_criteria(elements.first().unwrap());
        let threshold = worst_cost + alpha.value() * (best_cost - worst_cost);

        let elements = elements
            .into_iter()
            .take_while(|el| greedy_criteria(el) >= threshold)
            .collect::<Vec<_>>();

        Self {
            selection: SelectionControl::new(elements.len()),
            elements,
            rng,
        }
    }

    fn cmp_f64(a: f64, b: f64) -> Ordering {
        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    pub fn select(&mut self) -> Option<&T> {
        Some(&self.elements[self.selection.next(self.rng)?])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Alpha(f64);

impl Alpha {
    pub fn new(value: f64) -> Result<Self, AlphaError> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(AlphaError::OutOfBoundariesError)
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    /// The size of the RCL list
    pub fn rcl_size(&self, total: usize) -> usize {
        match (total as f64 * self.value()).ceil() as usize {
            0 => 1,
            x => x,
        }
    }
}

impl FromStr for Alpha {
    type Err = AlphaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Alpha::new(s.parse()?)
    }
}

#[derive(Debug, Error)]
pub enum AlphaError {
    #[error("Alfa value should be between 0 and 1")]
    OutOfBoundariesError,
    #[error(transparent)]
    NotAFloat(#[from] std::num::ParseFloatError),
}

#[cfg(test)]
mod tests {
    use super::Alpha;

    #[test]
    fn test_alpha() {
        assert!(Alpha::new(0.6).is_ok());
        assert!(Alpha::new(0.0).is_ok());
        assert!(Alpha::new(1.0).is_ok());
        assert!(Alpha::new(7.5).is_err());
        assert!(Alpha::new(-0.6).is_err());
    }

    #[test]
    fn rcl_size_works() {
        // alfa, list size, expected answer
        let rcl_size_tests = [
            (0.5, 10, 5),
            (1.0, 10, 10),
            (0.0, 10, 1),
            (0.9, 10, 9),
            (0.8, 10, 8),
        ];

        for test in rcl_size_tests {
            assert_eq!(
                alpha(test.0).rcl_size(test.1),
                test.2,
                "test case: {:?}",
                test
            );
        }
    }

    fn alpha(f: f64) -> Alpha {
        Alpha::new(f).unwrap()
    }
}
