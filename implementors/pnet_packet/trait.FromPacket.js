(function() {var implementors = {};
implementors["pnet_packet"] = [{"text":"impl&lt;'p&gt; FromPacket for ArpPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableArpPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for EthernetPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableEthernetPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for GrePacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableGrePacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for U16BEPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableU16BEPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for U32BEPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableU32BEPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for IcmpPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableIcmpPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for EchoReplyPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableEchoReplyPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for EchoRequestPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableEchoRequestPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for DestinationUnreachablePacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableDestinationUnreachablePacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for TimeExceededPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableTimeExceededPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for Icmpv6Packet&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableIcmpv6Packet&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for NdpOptionPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableNdpOptionPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for RouterSolicitPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableRouterSolicitPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for RouterAdvertPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableRouterAdvertPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for NeighborSolicitPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableNeighborSolicitPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for NeighborAdvertPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableNeighborAdvertPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for RedirectPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableRedirectPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for Ipv4Packet&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableIpv4Packet&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for Ipv4OptionPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableIpv4OptionPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for Ipv6Packet&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableIpv6Packet&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for ExtensionPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableExtensionPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for RoutingPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableRoutingPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for FragmentPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableFragmentPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for TcpPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableTcpPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for TcpOptionPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableTcpOptionPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for UdpPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableUdpPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for VlanPacket&lt;'p&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'p&gt; FromPacket for MutableVlanPacket&lt;'p&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()