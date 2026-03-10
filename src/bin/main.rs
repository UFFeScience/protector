#![allow(dead_code)]

mod setup;

use std::{fs::File, time::Duration};

use anyhow::{Context, Result};
use clap::Parser;
use cover_crime::{Analysis, Input, Instance, Run, output::Validate, params::*};
use log::{debug, info, trace};

use setup::{cli::Opts, log::log_level};

fn main() -> Result<()> {
    let mut opt = Opts::parse();

    // makes the program be quiet if the irace option is active.
    opt.quiet = opt.quiet || opt.irace;

    if !opt.quiet {
        setup::log::init(log_level(opt.verbose)).with_context(|| "Couldn't start logging")?;
    }

    trace!("Application started.");
    debug!("{:#?}", opt);

    info!("Parsing input file...");
    let input = Input::from_file(&opt.instance).with_context(|| "Couldn't get input from file")?;
    info!("Input parsed successfully.");

    debug!("Summary of Input:\n{:#?}", input);

    info!("Checking Input integrity...");
    cover_crime::check(&input).with_context(|| "Input file rejected on integrity checks.")?;
    info!("Check finished. Everything looks alright.");

    info!("Building Graph...");
    let instance = Instance::new(input)?;
    info!("Graph built successfully.");

    trace!("{:?}", instance.graph());

    let run_opts = RunOpts {
        metaheuristic: opt.metaheuristic,
        grasp: opt.heuristic_params(),
        mining: MiningParams {
            support: opt.mining_support,
        },
        executions: opt.executions,
        base_seed: opt.seed,
        intermediates_folder: opt.intermediates_folder,
        max_time: opt.time.map(Duration::from_secs),
        max_iter: opt.iterations,
    };

    let n = &run_opts.grasp.local_search.neighborhoods;
    info!(
        "Enabled neighborhoods: expand_route={}, add_loop={}, reposition_unit={}",
        n.expand_route, n.add_loop, n.reposition_unit,
    );

    info!("Building solutions...");
    let run = Run::new(&instance, run_opts)?;
    info!("Final solution built!");

    let analysis = Analysis::new(&run);
    let (number, solution, duration) = analysis.best();

    info!("Execution number of the best solution: {}", number);
    info!(
        "Average time: {:?}; score: {:.2}",
        analysis.average_time(),
        analysis.average_score()
    );
    info!(
        "Best solution time: {:?}; score: {:.2}",
        duration,
        solution.score()
    );

    if opt.irace {
        println!("{}", -solution.score());
    }

    if opt.print {
        solution.export(&mut std::io::stdout())?;
    }

    trace!("Instance's solution below\n{:#?}", solution);

    info!("Validating the solution generated...");
    solution.validate()?;
    info!("The solution is valid!");

    if let Some(file_name) = opt.save {
        info!("Exporting solution...");
        let file_name = file_name.unwrap_or_else(|| "heuristic_solution".into());
        let mut file = File::create(
            opt.instance
                .parent()
                .expect("instance should be inside a folder")
                .join(file_name),
        )
        .with_context(|| "when saving the solution")?;
        solution.export(&mut file)?;
        info!("Export finished.");
    }

    trace!("End of application.");
    Ok(())
}
