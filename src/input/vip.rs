use std::iter::Iterator;
use std::str::FromStr;

use anyhow::{Context, Result, anyhow};

use crate::input::Id;

#[derive(Debug)]
pub struct Vip {
    pub id: u32,
    pub vip_arcs: Vec<(Id, Id)>,
}

impl FromStr for Vip {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut segments = s.split_whitespace();
        let id: u32 = segments
            .next()
            .ok_or_else(|| anyhow!("Vip number isn't present"))?
            .parse()?;

        let number_pairs: usize = segments
            .next()
            .ok_or_else(|| anyhow!("number of vip arcs isn't present"))?
            .parse()?;

        let number_ids = number_pairs * 2;

        let segments: Vec<&str> = segments.collect();

        if segments.len() != number_ids {
            return Err(anyhow!("number of vips in Z{} not expected.", id));
        }

        let vip_ids = segments
            .into_iter()
            .map(str::parse)
            .map(|result| result.map_err(|_| anyhow!("failed to parse vertex ids")))
            .collect::<Result<Vec<_>>>()
            .with_context(|| "couldn't import VIP arcs")?;

        let vip_arcs = vip_ids.chunks(2).map(|p| (p[0], p[1])).collect();

        Ok(Self { id, vip_arcs })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Vip;

    #[test]
    fn conversion() {
        let instances = ["1 2 134 151 176 161", "2 1 55 58"];
        for i in instances.iter() {
            assert!(Vip::from_str(i).is_ok())
        }
    }
}
