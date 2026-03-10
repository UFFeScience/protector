use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    time::Duration,
};

use clap::Parser;
use cover_crime::{Analysis, Input, Instance, Percentage, Run, output::Validate, params::*};
use rand::{Rng, thread_rng};

#[derive(Debug, Parser)]
struct Opt {
    /// Folder to create the report
    folder: PathBuf,
    /// Number of executions with different seeds
    executions: u32,

    /// Number of iterations of the heuristic
    #[clap(long)]
    iterations: Option<usize>,

    /// How many seconds the heuristic is allowed to run
    #[clap(long)]
    time: Option<u64>,

    #[clap(long, default_value = "0.2")]
    pub alpha: Alpha,

    #[clap(long, default_value = "0.1")]
    pub max_edges_between_loop: Percentage,

    #[clap(long, default_value = "0.1")]
    pub max_section_size: Percentage,

    /// Base seed for the heuristic
    seed: u64,
    /// Only process solutions with known optima
    #[clap(long)]
    only_optima: bool,

    /// The percentage used as support for the mining procedure
    #[clap(long, default_value = "10")]
    pub mining_support: usize,

    /// Metaheuristic used
    #[clap(long)]
    pub metaheuristic: Metaheuristic,

    /// Fixed unit strategy used in construction
    #[clap(long, default_value = "random")]
    pub fixed_unit_strategy: FixedUnitStrategy,

    /// Neighborhoods to use in local search
    #[clap(long, value_delimiter = ',', default_values_t = [Neighborhood::ExpandRoute, Neighborhood::AddLoop, Neighborhood::RepositionUnit])]
    pub neighborhoods: Vec<Neighborhood>,
}

impl Opt {
    pub fn heuristic_params(&self) -> HeuristicParams {
        HeuristicParams {
            construction: ConstructionParams {
                alpha: self.alpha,
                fixed_unit_strategy: self.fixed_unit_strategy,
            },
            local_search: LocalSearchParams {
                max_edges_between_loop: self.max_edges_between_loop,
                max_section_size: self.max_section_size,
                neighborhoods: EnabledNeighborhoods::from_list(&self.neighborhoods),
            },
        }
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    let run_opts = RunOpts {
        metaheuristic: opt.metaheuristic,
        grasp: opt.heuristic_params(),
        mining: MiningParams {
            support: opt.mining_support,
        },
        executions: opt.executions,
        base_seed: opt.seed,
        intermediates_folder: "intermediate_solutions".into(),
        max_time: opt.time.map(Duration::from_secs),
        max_iter: opt.iterations,
    };

    match fs::create_dir("report") {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => Ok(()),
            _ => Err(e),
        },
    }?;
    let folder = {
        let name = opt
            .folder
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        opt.folder
            .with_file_name(name + &format!("-{}", thread_rng().r#gen::<u32>()))
    };

    let folder = PathBuf::from("report").join(folder);

    let _ = fs::remove_dir_all(&folder);
    fs::create_dir(&folder)?;

    store_parameters(&folder, &opt)?;

    let mut known = fs::File::create(folder.join("known_optima.csv"))?;
    writeln!(
        known,
        "instance;average time;average value;best's value;optimal value;GAP (%)"
    )?;

    let mut unknown = fs::File::create(folder.join("unknown_optima.csv"))?;
    writeln!(
        unknown,
        "instance;average time;average value;best's value;BKS value;GAP (%)"
    )?;

    for folder in fs::read_dir("instances")? {
        let folder = folder?;
        if opt.only_optima {
            let mut instance_folder =
                fs::read_dir(folder.path()).expect("only folders on instances");
            if !instance_folder.any(|file| file.unwrap().file_name() == "optimal_solution") {
                continue;
            }
        }

        let name = folder.file_name();
        println!("Running {:?}...", &name);
        let file = {
            let mut path = folder.path();
            path.push("graph");
            path
        };

        let input = Input::from_file(&file)?;
        let instance = Instance::new(input)?;

        let Ok(run) = Run::new(&instance, run_opts.clone()) else {
            println!("instancia {name:?} falhou");
            continue;
        };

        let analysis = Analysis::new(&run);
        analysis.best().1.validate()?;

        let optimal = get_score(&folder, "optimal_solution");

        match optimal {
            Some(optimal) => {
                write_row(&mut known, &name.to_string_lossy(), &analysis, optimal)?;
            }
            None => {
                let bks_score = get_score(&folder, "best_known_solution").unwrap_or(f64::NAN);
                write_row(&mut unknown, &name.to_string_lossy(), &analysis, bks_score)?;
            }
        }
    }

    println!("Report saved on {}", folder.to_string_lossy());
    Ok(())
}

fn store_parameters(folder: &Path, opt: &Opt) -> io::Result<()> {
    fs::File::create(folder.join("parameters.txt"))
        .and_then(|mut file| file.write_all(format!("{opt:#?}").as_bytes()))
}

fn get_score(folder: &fs::DirEntry, file: &str) -> Option<f64> {
    let mut path = folder.path();
    path.push(file);

    let mut first_line = String::new();
    BufReader::new(fs::File::open(path).ok()?)
        .read_line(&mut first_line)
        .ok()?;

    first_line
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.parse().ok())
}

fn write_row(
    file: &mut impl Write,
    name: &str,
    analysis: &Analysis,
    comparison: f64,
) -> io::Result<()> {
    let (_, best, _) = analysis.best();
    writeln!(
        file,
        "{:?};{};{:.2};{:.2};{};{:.2}",
        name,
        analysis.average_time().as_secs_f64(),
        analysis.average_score(),
        best.score(),
        comparison,
        analysis.gap(comparison),
    )
}
