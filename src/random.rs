use crate::option::Opt;
use rand::distributions::Uniform;
use std::net::Ipv4Addr;

pub fn random_global_ipv4_addr<R>(_opt: &Opt, rng: &mut R) -> Ipv4Addr
where
    R: rand::Rng,
{
    let distribution = Uniform::new_inclusive(0, 254);
    loop {
        let addr = Ipv4Addr::new(
            rng.sample(distribution),
            rng.sample(distribution),
            rng.sample(distribution),
            rng.sample(distribution),
        );
        if addr.is_global() {
            return addr
        }
    }
}

pub fn random_source_port<R>(opt: &Opt, rng: &mut R) -> u16
where
    R: rand::Rng,
{
    let distribution = Uniform::new_inclusive(opt.min_port, opt.max_port);
    rng.sample(distribution)
}
