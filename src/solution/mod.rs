use std::io::{self, Write};

use ordered_float::NotNan;
use petgraph::graph::{EdgeIndex, NodeIndex};
use rand::{Rng, seq::SliceRandom};

use crate::{
    Instance,
    data_mining::Patterns,
    graph::{Graph, vertex::Vertex, vip::InsertedVip},
    input::Zone,
    params::Alpha,
    shared::Coverage,
};

pub mod local_search;
pub use local_search::local_search;

mod route;
pub use route::path_builder;
use route::{Route, to_text};

mod constructor;
use constructor::{build_global, build_vip, build_zone, random_fixed_unit_insertion};

pub use constructor::FixedUnitStrategy;
use constructor::RouteParams;

mod contribution;
pub(crate) use contribution::compute_score;

use self::constructor::{
    build_global_with_pattern, build_vip_with_pattern, build_zone_with_pattern,
    greedy_fixed_unit_insertion,
};

#[derive(Debug, Clone, Copy)]
pub struct Params {
    pub alpha: Alpha,
    pub fixed_unit_strategy: FixedUnitStrategy,
}

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    vips: Vec<(&'a InsertedVip, Route<'a>)>,
    zones: Vec<(Zone, Vec<Route<'a>>)>,
    globals: Vec<Route<'a>>,
    fixed_units: Vec<NodeIndex>,
    graph: &'a Graph,
    score: NotNan<f64>,
    coverage: Coverage<EdgeIndex>,
    pub instance: &'a Instance,
}

impl<'a> Solution<'a> {
    /// Construct a new Solution
    pub fn new(instance: &'a Instance, params: &Params, rng: &mut impl Rng) -> Option<Self> {
        let mut coverage = Coverage::new(instance.arcs());
        let graph = instance.graph();

        let routes = instance
            .inserted_vips()
            .iter()
            .map(|vip| {
                build_vip(graph, vip, instance.global_distance(), rng)
                    .map(|route| update_coverage(route, &mut coverage))
            })
            .collect::<Result<Vec<_>, ()>>()
            .ok()?;

        let vips_routes: Vec<(&InsertedVip, Route)> =
            instance.inserted_vips().iter().zip(routes).collect();

        let zones: Vec<(Zone, Vec<Route>)> = instance
            .zones()
            .map(|zone| {
                let routes = (0..instance.routes_per_zone())
                    .flat_map(|_| {
                        let zone_params = RouteParams {
                            max_distance: instance.zone_distance(),
                            graph,
                            coverage: &coverage,
                            alpha: params.alpha,
                        };

                        build_zone(&zone_params, zone, rng)
                            .map(|route| update_coverage(route, &mut coverage))
                    })
                    .collect::<Vec<_>>();
                (zone, routes)
            })
            .collect();

        let globals = (0..instance.global_routes())
            .flat_map(|_| {
                let global_params = RouteParams {
                    max_distance: instance.global_distance(),
                    graph,
                    coverage: &coverage,
                    alpha: params.alpha,
                };

                build_global(&global_params, rng).map(|route| update_coverage(route, &mut coverage))
            })
            .collect::<Vec<_>>();

        let fixed_units = (0..instance.fixed_units())
            .map(|_| match params.fixed_unit_strategy {
                FixedUnitStrategy::Random => random_fixed_unit_insertion(graph, &mut coverage, rng),
                FixedUnitStrategy::Greedy => greedy_fixed_unit_insertion(graph, &mut coverage),
            })
            .collect();

        let score = compute_score(graph, &coverage);

        Some(Self {
            vips: vips_routes,
            zones,
            globals,
            fixed_units,
            graph,
            score,
            coverage,
            instance,
        })
    }

    pub fn new_with_patterns(
        instance: &'a Instance,
        params: &Params,
        patterns: &Patterns,
        rng: &mut impl Rng,
    ) -> Option<Self> {
        let mut coverage = Coverage::new(instance.arcs());
        let graph = instance.graph();

        let routes = instance
            .inserted_vips()
            .iter()
            .map(|vip| {
                build_vip_with_pattern(
                    graph,
                    vip,
                    instance.global_distance(),
                    patterns
                        .vips
                        .iter()
                        .find(|p| p.0 == vip.id)
                        .expect("vip pattern for each vip")
                        .1
                        .choose(rng),
                    rng,
                )
                .map(|route| update_coverage(route, &mut coverage))
            })
            .collect::<Result<Vec<_>, ()>>()
            .ok()?;

        let vips_routes: Vec<(&InsertedVip, Route)> =
            instance.inserted_vips().iter().zip(routes).collect();

        let zones: Vec<(Zone, Vec<Route>)> = instance
            .zones()
            .map(|zone| {
                let routes = (0..instance.routes_per_zone())
                    .flat_map(|_| {
                        let zone_params = RouteParams {
                            max_distance: instance.zone_distance(),
                            graph,
                            coverage: &coverage,
                            alpha: params.alpha,
                        };

                        build_zone_with_pattern(
                            &zone_params,
                            zone,
                            patterns
                                .zones
                                .iter()
                                .find(|z| z.0 == zone)
                                .expect("zone pattern for each zone")
                                .1
                                .choose(rng),
                            rng,
                        )
                        .map(|route| update_coverage(route, &mut coverage))
                    })
                    .collect::<Vec<_>>();
                (zone, routes)
            })
            .collect();

        let globals = (0..instance.global_routes())
            .flat_map(|_| {
                let global_params = RouteParams {
                    max_distance: instance.global_distance(),
                    graph,
                    coverage: &coverage,
                    alpha: params.alpha,
                };

                build_global_with_pattern(&global_params, patterns.globals.choose(rng), rng)
                    .map(|route| update_coverage(route, &mut coverage))
            })
            .collect::<Vec<_>>();

        let fixed_units = (0..instance.fixed_units())
            .map(|_| random_fixed_unit_insertion(graph, &mut coverage, rng))
            .collect();

        let score = compute_score(graph, &coverage);

        Some(Self {
            vips: vips_routes,
            zones,
            globals,
            fixed_units,
            graph,
            score,
            coverage,
            instance,
        })
    }

    /// The routes which are VIP
    pub fn vip_routes(&self) -> &[(&InsertedVip, Route<'_>)] {
        &self.vips
    }

    /// The routes which are global and not VIP
    pub fn global_routes(&self) -> &[Route<'_>] {
        &self.globals
    }

    /// Routes Belonging to an specific Zone
    pub fn zone_routes(&self) -> &[(Zone, Vec<Route<'_>>)] {
        &self.zones
    }

    pub fn fixed_units(&self) -> &[NodeIndex] {
        &self.fixed_units
    }

    /// The score is the sum of the crime factors of all routes and fixed units.
    pub fn score(&self) -> NotNan<f64> {
        self.score
    }

    pub fn refresh_score(&mut self) -> NotNan<f64> {
        let new_score = compute_score(self.graph, &self.coverage);
        self.score = new_score;
        new_score
    }

    /// Export the solution in the custom output structure defined in the project.
    pub fn export(&self, w: &mut impl Write) -> io::Result<()> {
        writeln!(w, "S {:.2}", self.score())?;

        let fixed_units = self
            .fixed_units()
            .iter()
            .map(|node| self.graph[*node].weight.id().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(w, "F {}", fixed_units)?;

        self.vip_routes()
            .iter()
            .map(|(_, route)| route)
            .chain(self.global_routes().iter())
            .try_for_each(|route| writeln!(w, "G {}", to_text(&route.edges, self.graph)))?;

        for (zone, routes) in self.zone_routes() {
            routes.iter().try_for_each(|route| {
                writeln!(
                    w,
                    "Z{} {}",
                    zone.as_ref(),
                    to_text(&route.edges, self.graph)
                )
            })?;
        }

        Ok(())
    }
}

/// Helper method to update `coverage` in the routes iterators, in [Solution::new].
///
/// It just updates the coverage and forwards `route` without mutating it.
/// We take ownership just to chain it easily.
fn update_coverage<'a>(route: Route<'a>, coverage: &mut Coverage<EdgeIndex>) -> Route<'a> {
    coverage.merge(&route.coverage);
    route
}
