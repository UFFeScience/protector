use rand::{Rng, distributions::Standard, prelude::Distribution};

/// The iterator built by this function randomly chooses a starting point and a direction
/// to iterate over the elements It cycles once, in order to go through all elements once,
/// if necessary.
pub fn iterate_randomly(
    total: usize,
    removal: usize,
    rng: &mut impl Rng,
) -> Box<dyn Iterator<Item = usize>> {
    let possible = total - removal + 1;
    let range = 0..possible;
    let start = rng.gen_range(range.clone());
    let direction = rng.r#gen::<Direction>();

    match direction {
        Direction::Forward => Box::new(range.cycle().skip(start).take(possible)),
        Direction::Reverse => Box::new(range.rev().cycle().skip(start).take(possible)),
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use rand_pcg::Pcg64;

    use super::iterate_randomly;

    #[test]
    // The expectations on this test were checked manually.
    fn iterate_randomly_works() {
        assert_eq!(vec_from_seed(5), vec![2, 3, 0, 1]);
        assert_eq!(vec_from_seed(6), vec![0, 3, 2, 1]);
    }

    fn vec_from_seed(seed: u64) -> Vec<usize> {
        iterate_randomly(5, 2, &mut Pcg64::seed_from_u64(seed)).collect()
    }
}

#[derive(Debug)]
enum Direction {
    Forward,
    Reverse,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        if rng.r#gen() {
            Direction::Forward
        } else {
            Direction::Reverse
        }
    }
}
