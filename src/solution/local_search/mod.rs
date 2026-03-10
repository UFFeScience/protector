mod loop_move;
mod path;
mod reposition_unit;
mod section_moves;

use clap::ValueEnum;
use petgraph::graph::{EdgeIndex, NodeIndex};
use rand::Rng;

use crate::{Percentage, Solution, graph::vertex::Vertex};

// Local Search moves
use loop_move::add_loop;
use reposition_unit::reposition_unit;
use section_moves::expand_route;

use self::section_moves::RemovalParams;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Neighborhood {
    ExpandRoute,
    AddLoop,
    RepositionUnit,
}

impl std::fmt::Display for Neighborhood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no skipped values")
            .get_name()
            .fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EnabledNeighborhoods {
    pub expand_route: bool,
    pub add_loop: bool,
    pub reposition_unit: bool,
}

impl EnabledNeighborhoods {
    pub fn from_list(neighborhoods: &[Neighborhood]) -> Self {
        Self {
            expand_route: neighborhoods.contains(&Neighborhood::ExpandRoute),
            add_loop: neighborhoods.contains(&Neighborhood::AddLoop),
            reposition_unit: neighborhoods.contains(&Neighborhood::RepositionUnit),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Params {
    pub max_edges_between_loop: Percentage,
    pub max_section_size: Percentage,
    pub neighborhoods: EnabledNeighborhoods,
}

pub fn local_search(solution: &mut Solution<'_>, params: &Params, rng: &mut impl Rng) {
    let no_restriction = |_, _| false;
    let no_special = |_| false;
    let graph = solution.graph;

    loop {
        for (zone, routes) in solution.zones.iter_mut() {
            for route in routes.iter_mut() {
                let same_zone_restriction =
                    |_, node: NodeIndex| graph[node].weight.zone() != *zone;

                if params.neighborhoods.expand_route {
                    let mut removal_params = RemovalParams {
                        solution_coverage: &mut solution.coverage,
                        route,
                        path_restriction: &same_zone_restriction,
                        special_edge: &no_special,
                    };

                    expand_route(&mut removal_params, params.max_section_size, rng);
                }
                if params.neighborhoods.add_loop {
                    add_loop(
                        route,
                        &mut solution.coverage,
                        &same_zone_restriction,
                        params,
                        rng,
                    );
                }
            }
        }

        for route in solution.globals.iter_mut() {
            if params.neighborhoods.expand_route {
                let mut removal_params = RemovalParams {
                    solution_coverage: &mut solution.coverage,
                    route,
                    path_restriction: &no_restriction,
                    special_edge: &no_special,
                };

                expand_route(&mut removal_params, params.max_section_size, rng);
            }
            if params.neighborhoods.add_loop {
                add_loop(route, &mut solution.coverage, &no_restriction, params, rng);
            }
        }

        for (vips, route) in solution.vips.iter_mut() {
            let vip_restriction = |edge: EdgeIndex| {
                vips.edges.contains(&edge)
                    || graph[edge]
                        .weight
                        .reverse
                        .filter(|reverse| vips.edges.contains(reverse))
                        .is_some()
            };

            if params.neighborhoods.expand_route {
                let mut removal_params = RemovalParams {
                    solution_coverage: &mut solution.coverage,
                    route,
                    path_restriction: &no_restriction,
                    special_edge: &vip_restriction,
                };

                expand_route(&mut removal_params, params.max_section_size, rng);
            }
            if params.neighborhoods.add_loop {
                add_loop(route, &mut solution.coverage, &no_restriction, params, rng);
            }
        }

        if params.neighborhoods.reposition_unit {
            for unit in solution.fixed_units.iter_mut() {
                *unit = reposition_unit(solution.graph, &mut solution.coverage, *unit);
            }
        }

        let old_score = solution.score();
        solution.refresh_score();

        if old_score == solution.score() {
            break;
        }
    }
}
