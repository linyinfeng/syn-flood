use quick_error::quick_error;
use std::{
    net::{Ipv4Addr, Ipv6Addr},
    time::SystemTimeError,
};

quick_error! {
    #[derive(Debug)]
    pub enum SynFloodError {
        ToSocketAddrs(from: String, e: std::io::Error) {
            cause(e)
            display("failed to convert \"{}\" to socket addresses: {}", from, e)
        }
        NoSocketAddr {
            display("no socket addresses resolved satisfies options provided")
        }
        TransportChannel(e: std::io::Error) {
            cause(e)
            display("failed to open transport channel: {}", e)
        }
        NewIpv4Packet {
            display("failed to construct IPv4 packet")
        }
        NewTcpPacket {
            display("failed to construct tcp packet")
        }
        SendPacket(e: std::io::Error) {
            display("failed to send packet: {}", e)
        }
        Ipv4DestinationWithIpv6Source(destination: Ipv4Addr, source: Ipv6Addr) {
            display("specified IPv6 source {} for IPv4 destination {}", source, destination)
        }
        SystemTime(e: SystemTimeError) {
            cause(e)
            display("failed to get system time: {}", e)
        }
    }
}
