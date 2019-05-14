initSidebarItems({"constant":[["ACK","ACK – indicates that the Acknowledgment field is significant. All packets after the initial SYN packet sent by the client should have this flag set."],["CWR","CWR – Congestion Window Reduced (CWR) flag is set by the sending host to indicate that it received a TCP segment with the ECE flag set and had responded in congestion control mechanism (added to header by RFC 3168)."],["ECE","ECE – ECN-Echo has a dual role, depending on the value of the SYN flag. It indicates: If the SYN flag is set (1), that the TCP peer is ECN capable. If the SYN flag is clear (0), that a packet with Congestion Experienced flag set (ECN=11) in IP header received during normal transmission (added to header by RFC 3168)."],["FIN","FIN – No more data from sender"],["NS","NS – ECN-nonce concealment protection (experimental: see RFC 3540)."],["PSH","PSH – Push function. Asks to push the buffered data to the receiving application."],["RST","RST – Reset the connection"],["SYN","SYN – Synchronize sequence numbers. Only the first packet sent from each end should have this flag set."],["URG","URG – indicates that the Urgent pointer field is significant"]]});