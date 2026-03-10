use crate::{Output, input::Vip, output::Arc};

pub fn validate(vips: &[Vip], output: &Output) -> Result<(), Error> {
    'vip: for vip in vips.iter() {
        let vip_arcs = vip
            .vip_arcs
            .iter()
            .map(|(a, b)| (*a.as_ref() as usize, *b.as_ref() as usize))
            .collect::<Vec<_>>();

        for route in output.global_routes.iter() {
            let belongs_to_route = |arc: &Arc| {
                let reverse = &(arc.1, arc.0);
                route.contains(arc) || route.contains(reverse)
            };
            if vip_arcs.iter().all(belongs_to_route) {
                continue 'vip;
            }
        }
        return Err(Error::VipMissing(vip.id));
    }
    Ok(())
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("Vip {0} is not a subset of any global route.")]
    VipMissing(u32),
}
