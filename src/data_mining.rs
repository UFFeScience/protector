use std::fs;

use itertools::Itertools;
use log::debug;
use petgraph::prelude::EdgeIndex;

use crate::{Solution, input::Zone, shared::elite_set::EliteSet};

#[derive(Debug)]
pub struct Patterns {
    pub zones: Vec<(Zone, Vec<RoutePattern>)>,
    pub vips: Vec<(VipId, Vec<RoutePattern>)>,
    pub globals: Vec<RoutePattern>,
}

type VipId = u32;

pub type RoutePattern = Vec<EdgeIndex>;

pub fn mine_elites(elite_set: &EliteSet<Solution>, params: MiningParams) -> Patterns {
    std::fs::write("elite_set.debug", format!("{:#?}", elite_set)).unwrap();

    let patterns = Patterns {
        globals: mine_routes(
            elite_set
                .iter()
                .flat_map(|s| s.global_routes())
                .map(|route| route.edges.as_ref()),
            params,
        ),
        vips: elite_set
            .iter()
            .flat_map(|s| s.vip_routes())
            .map(|(vip, route)| (vip.id, route))
            .sorted_by_key(|(vip, _)| *vip)
            .group_by(|(vip, _)| *vip)
            .into_iter()
            .map(|(vip, routes)| {
                (
                    vip,
                    mine_routes(routes.map(|(_, route)| route.edges.as_ref()), params),
                )
            })
            .collect(),
        zones: elite_set
            .iter()
            .flat_map(|s| s.zone_routes())
            .sorted_by_key(|(zone, _)| *zone)
            .group_by(|(zone, _)| *zone)
            .into_iter()
            .map(|(zone, routes)| {
                (
                    zone,
                    mine_routes(
                        routes
                            .flat_map(|(_, route)| route)
                            .map(|route| route.edges.as_ref()),
                        params,
                    ),
                )
            })
            .collect(),
    };

    std::fs::write("patterns.debug", format!("{:#?}", patterns)).unwrap();
    patterns
}

fn mine_routes<'a>(
    routes: impl IntoIterator<Item = &'a [EdgeIndex]>,
    params: MiningParams,
) -> Vec<RoutePattern> {
    fs::create_dir_all("tmp/data_mining").unwrap();

    let mut lines = routes
        .into_iter()
        .map(|route| route.iter().map(|num| num.index().to_string()).join(" -1 ") + " -1 -2\n")
        .peekable();

    if lines.peek().is_none() {
        debug!("there's no route to mine");
        return Default::default();
    }

    let body = lines.join("");
    let response = execute_miner(body, params);

    let mut patterns = response
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line: Vec<&str> = line.split_whitespace().collect();

            // Remove support information e.g. `#SUP: 2`
            let final_len = line.len().saturating_sub(2);
            line.truncate(final_len);

            line.into_iter()
                // Data mining separators such as -1 and -2 will fail to parse to `u32`
                // and then they'll be ignored by the flat operation
                .flat_map(|piece| piece.parse::<u32>())
                .map(EdgeIndex::from)
                .collect()
        })
        .sorted_by_key(|line: &Vec<_>| -(line.len() as i32))
        .collect::<Vec<_>>();

    patterns.truncate(10);
    patterns
}

#[derive(Debug, Clone, Copy)]
pub struct MiningParams {
    pub support: usize,
}

fn execute_miner(lines: String, MiningParams { support }: MiningParams) -> String {
    let client = reqwest::blocking::Client::new();
    let params = [
        ("input", lines),
        ("minsup", (support as f32 / 100.0).to_string()),
    ];
    client
        .post("http://localhost:8001")
        .form(&params)
        .send()
        .unwrap()
        .text()
        .unwrap()
}
