use anyhow::{Result, anyhow};
use nom::{
    IResult,
    character::complete::{char, digit1, line_ending, multispace0, multispace1, space0},
    combinator::map_res,
    error::ParseError,
    multi::{many0, separated_list0},
    number::complete::double,
    sequence::{delimited, pair, preceded, separated_pair},
};

use super::{Arc, Node, Route, Zone};
/// Represents a parsed output file
#[derive(Debug)]
pub struct Output {
    /// The sum of the crime factor of the solution
    pub score: f64,
    /// Positioning of fixed units
    pub fixed_units: Vec<Node>,
    /// All routes which go through zones, including vip routes
    pub global_routes: Vec<Route>,
    /// Routes which are inside a specific zone
    pub zone_routes: Vec<(Zone, Route)>,
}

impl Output {
    /// Parses the given `input` to generate an [Output]
    pub fn new(input: &str) -> Result<Self> {
        let (i, score) = ws(score)(input).map_err(|_| anyhow!("Couldn't parse score"))?;
        let (i, fixed_units) =
            ws(fixed_units)(i).map_err(|_| anyhow!("Couldn't parse fixed units"))?;
        let (i, global_routes) =
            get_routes(global_route)(i).map_err(|_| anyhow!("Couldn't parse global routes"))?;
        let (_, zone_routes) =
            get_routes(zone_route)(i).map_err(|_| anyhow!("Couldn't parse zone routes"))?;

        let global_routes = global_routes
            .into_iter()
            .filter(|route| !route.is_empty())
            .collect();

        let zone_routes = zone_routes
            .into_iter()
            .filter(|(_, route)| !route.is_empty())
            .collect();

        Ok(Self {
            score,
            fixed_units,
            global_routes,
            zone_routes,
        })
    }
}

fn score(input: &str) -> IResult<&str, f64> {
    delimited(char('S'), ws(double), line_ending)(input)
}

fn fixed_units(input: &str) -> IResult<&str, Vec<Node>> {
    preceded(
        multispace0,
        delimited(
            ws(char('F')),
            ws(separated_list0(multispace1, number)),
            line_ending,
        ),
    )(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse::<usize>)(input)
}

fn arc(input: &str) -> IResult<&str, Arc> {
    delimited(
        char('('),
        separated_pair(number, ws(char(',')), number),
        char(')'),
    )(input)
}

fn route(input: &str) -> IResult<&str, Route> {
    separated_list0(ws(char(',')), arc)(input)
}

fn zone_route(input: &str) -> IResult<&str, (Zone, Route)> {
    preceded(
        multispace0,
        delimited(char('Z'), pair(number, ws(route)), line_ending),
    )(input)
}

fn global_route(input: &str) -> IResult<&str, Route> {
    preceded(multispace0, delimited(char('G'), ws(route), line_ending))(input)
}

fn get_routes<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    many0(ws(inner))
}

#[cfg(test)]
mod tests;
