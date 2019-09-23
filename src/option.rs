use std::{net::IpAddr, num::ParseFloatError, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Opt {
    /// Activate debug mode.
    ///
    /// Enabling this option will set the log level of the tool to Trace. To set
    /// the log level of other modules used by the tool, please use RUST_LOG
    /// environment variable.
    #[structopt(short, long)]
    pub debug: bool,

    /// Destination address and port.
    ///
    /// Including an address and a port. If a domain is provided, the tool will
    /// resolve it.
    pub destination: String,

    /// Force using IPv4 address.
    ///
    /// This option conflicts with option --ipv6.
    #[structopt(short = "4", long)]
    pub ipv4: bool,

    /// Force using IPv6 address.
    ///
    /// This option conflicts with option --ipv4.
    #[structopt(short = "6", long, conflicts_with = "ipv4")]
    pub ipv6: bool,

    /// Sets buffer size.
    #[structopt(short = "b", long = "buffer-size", default_value = "8192")]
    pub buffer_size: usize,

    /// Sets ttl of IP packets.
    #[structopt(long, default_value = "128")]
    pub ttl: u8,

    /// Sets window size of TCP packets.
    #[structopt(short = "e", long = "window-size", default_value = "64240")]
    pub window_size: u16,

    /// Sets minimal port number.
    #[structopt(long = "min-port", default_value = "49152")]
    pub min_port: u16,

    /// Sets maximal port number.
    #[structopt(long = "max-port", default_value = "65535")]
    pub max_port: u16,

    /// Force using source address.
    #[structopt(long = "source-address")]
    pub source_address: Option<IpAddr>,

    /// Force using source port.
    #[structopt(long = "source-port")]
    pub source_port: Option<u16>,

    /// Sets output interval(seconds).
    #[structopt(
        short,
        long = "output-interval",
        default_value = "1",
        parse(try_from_str = duration_try_from_str)
    )]
    pub output_interval: Duration,

    /// Sets output interval(seconds).
    #[structopt(
        short = "i",
        long = "interval",
        parse(try_from_str = duration_try_from_str)
    )]
    pub interval: Option<Duration>,

    /// Number of packets to be sent.
    #[structopt(short, long)]
    pub number: Option<usize>,

    /// Maximal time of sending packets.
    #[structopt(short, long, parse(try_from_str = duration_try_from_str))]
    pub time: Option<Duration>,
}

fn duration_try_from_str(s: &str) -> Result<Duration, ParseFloatError> {
    Ok(Duration::from_secs_f64(s.parse()?))
}
