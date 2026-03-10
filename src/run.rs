use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
    time::{Duration, Instant},
};

use anyhow::{anyhow, ensure};
use ordered_float::NotNan;
use rand::SeedableRng;

use crate::{
    Instance, Solution,
    data_mining::MiningParams,
    metaheuristic::{self, DmParams},
    shared::stop_criterion::{IterCriterion, StopCriterion, TimeCriterion},
};

#[derive(Debug, Clone)]
pub struct RunOpts {
    pub executions: u32,
    pub base_seed: u64,
    /// Determines the folder where intermediate solutions are saved
    pub intermediates_folder: PathBuf,
    pub metaheuristic: Metaheuristic,
    pub grasp: metaheuristic::Params,
    pub mining: MiningParams,
    pub max_time: Option<Duration>,
    pub max_iter: Option<usize>,
}

#[derive(Debug, Clone, Copy)]
pub enum Metaheuristic {
    Grasp,
    DmGrasp,
    Greedy,
}

impl FromStr for Metaheuristic {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "grasp" => Metaheuristic::Grasp,
            "dm-grasp" => Metaheuristic::DmGrasp,
            "one-greedy-construction" => Metaheuristic::Greedy,
            _ => return Err("failed to parse metaheuristic"),
        })
    }
}

impl Display for Metaheuristic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Metaheuristic::Grasp => f.write_str("grasp"),
            Metaheuristic::DmGrasp => f.write_str("dm-grasp"),
            Metaheuristic::Greedy => f.write_str("one-greedy-construction"),
        }
    }
}

#[derive(Debug)]
pub struct Run<'a> {
    solutions: Vec<(u32, Solution<'a>, Duration)>,
    opts: RunOpts,
}

impl<'a> Run<'a> {
    pub fn new(instance: &'a Instance, opts: RunOpts) -> anyhow::Result<Self> {
        ensure!(
            opts.max_iter.xor(opts.max_time.map(|_| 0)).is_some(),
            "You must set exactly one stop criterion: time or iter."
        );

        let mut best = NotNan::<f64>::default();
        // Preparing the environment for intermediate solutions export.
        let _ = std::fs::remove_dir_all(&opts.intermediates_folder);
        let _ = std::fs::remove_dir_all("tmp");

        let mut export_intermediate = |solution: &Solution| {
            if solution.score() > best {
                best = solution.score();
                export(solution, &opts.intermediates_folder, best)?;
            }
            Some(())
        };

        let solutions = (1..=opts.executions)
            .flat_map(|exec_number| {
                let start = Instant::now();

                let seed = opts.base_seed + exec_number as u64;
                let mut rng = rand_pcg::Pcg64::seed_from_u64(seed);

                let (mut time_criterion, mut iter_criterion);

                let stop_criterion: &mut dyn StopCriterion = if let Some(max_time) = opts.max_time {
                    time_criterion = TimeCriterion::new(max_time);
                    &mut time_criterion
                } else {
                    iter_criterion = IterCriterion::new(opts.max_iter.unwrap());
                    &mut iter_criterion
                };

                let solution = match opts.metaheuristic {
                    Metaheuristic::Grasp => metaheuristic::multistart_grasp(
                        instance,
                        &opts.grasp,
                        stop_criterion,
                        &mut rng,
                        &mut export_intermediate,
                    ),
                    Metaheuristic::DmGrasp => metaheuristic::dm_grasp(
                        instance,
                        DmParams {
                            grasp: opts.grasp,
                            mining: opts.mining,
                        },
                        stop_criterion,
                        &mut rng,
                        &mut export_intermediate,
                    ),
                    Metaheuristic::Greedy => {
                        metaheuristic::one_greedy_construction(instance, &opts.grasp, &mut rng)
                    }
                }?;

                let duration = start.elapsed();

                log::info!(
                    "Execution {} done. Time: {:?}, Score: {:.2}",
                    exec_number,
                    duration,
                    solution.score(),
                );

                Some((exec_number, solution, duration))
            })
            .collect::<Vec<_>>();

        if solutions.is_empty() {
            return Err(anyhow!("Couldn't build any solution"));
        }

        Ok(Self { solutions, opts })
    }

    /// Get a reference to the run's solutions.
    pub fn solutions(&self) -> &[(u32, Solution<'a>, Duration)] {
        &self.solutions
    }

    /// Get a reference to the run's opts.
    pub fn opts(&self) -> &RunOpts {
        &self.opts
    }
}

// Exporting intermediate solution
fn export(solution: &Solution, folder: &Path, best: NotNan<f64>) -> Option<()> {
    let _ = fs::create_dir(folder);
    let file_name = format!("{:.2}.txt", best.as_ref());
    let mut file = fs::File::create(folder.join(file_name)).ok()?;
    solution.export(&mut file).ok()?;
    Some(())
}
