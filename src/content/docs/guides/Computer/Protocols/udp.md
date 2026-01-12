---
title: UDP
---


## UDP (User Datagram Protocol)

UDP (User Datagram Protocol) is a **connectionless** transport-layer protocol that prioritizes **speed and simplicity** over reliability. Unlike TCP, UDP does not establish a connection before sending data, nor does it guarantee delivery, ordering, or duplication prevention. Data packets, called **datagrams**, may arrive out of order, arrive multiple times, or not arrive at all. The only integrity check performed by UDP is a **checksum**, which verifies that the data has not been corrupted during transmission.

Because UDP has very low overhead and minimal delay, it is well suited for **real-time applications** such as video streaming, online gaming, voice over IP (VoIP), and DNS queries. In these cases, receiving data quickly is often more important than receiving it perfectly. Any necessary reliability or ordering is typically handled by the application itself rather than by the protocol.
