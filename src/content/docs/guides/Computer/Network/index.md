---
title: Computer Networks
---

- [Videos in french](https://www.youtube.com/@guillaumeurvoy-keller3951/videos)
- [Computer Networking: A Top-Down Approach](https://gaia.cs.umass.edu/kurose_ross/online_lectures.htm)

## OSI Model

The OSI model is a seven-layer conceptual framework that standardizes and describes the distinct functions of network communication - from physical bit transmission to application-level services - to guide interoperability, design, and troubleshooting.

| No. | OSI Layer        | Responsibilities                                    |
| --- | :--------------- | --------------------------------------------------- |
| 7   | **Application**  | Application-level services, end-user protocols      |
| 6   | **Presentation** | Data representation, encryption, compression        |
| 5   | **Session**      | Session and connection management                   |
| 4   | **Transport**    | End-to-end communication, reliability, flow control |
| 3   | **Network**      | Logical addressing and routing                      |
| 2   | **Data Link**    | Framing, MAC addressing, error detection/correction |
| 1   | **Physical**     | Physical medium, bit transmission, connectors       |

## TCP/IP Model

As the OSI model is only conceptual, the TCP/IP is the most used for communication on network

| TCP/IP Layer    | Name             | Responsibilities                                                                                                | Examples             |
| --------------- | ---------------- | --------------------------------------------------------------------------------------------------------------- | -------------------- |
| **Application** | Application data | Application protocols used by programs. <br/> The application layer may live on top of a TLS (encryption) layer | HTTP, FTP, SMTP, DNS |
| **Transport**   | Segment          | Host-to-host communication, ports, reliability                                                                  | TCP, UDP             |
| **Internet**    | Datagram/Packet  | Logical addressing and routing (IP)                                                                             | IPv4, IPv6, ICMP     |
| **Link**        | Frame            | Physical addressing and local delivery                                                                          | Ethernet, Wi‑Fi      |
| (part of Link)  | Bits             | /                                                                                                               | Cables, fiber        |

## Acronyms for Area Network

- Personal Area Network (PAN)
- Wireless LAN (WLAN)
- Local Area Network (LAN)
- Metropolitan Area Network (MAN)
- Wide Area Network (WAN)

## Access to the medium

Access to the medium defines the mechanisms that allow multiple devices to share and coordinate the use of a communication link.

### Deterministic Access Methods

- FDMA (**Frequency Division Multiple Access**): Each user is assigned a distinct frequency band for communication.
- TDMA (**Time Division Multiple Access**): Users share the same frequency band but transmit in different time slots.
- CDMA (**Code Division Multiple Access**): Each user is assigned a unique code, allowing multiple users to transmit simultaneously over the same frequency band.
- OFDMA (**Orthogonal Frequency Division Multiple Access**): A variation of FDMA in which the subcarrier frequencies are selected to be orthogonal in the Fourier domain, improving spectral efficiency and reducing interference.

### Random Multiple Access Methods

- **ALOHA**: When a collision occurs, the transmitter waits for a randomly chosen delay before retransmitting.

- **Slotted ALOHA**: An enhanced version of ALOHA where time is divided into slots, with `T_slot ≥ T_propagation`, reducing the probability of collisions.

- **CSMA (Carrier Sense Multiple Access)**: Stations listen to the channel and transmit only if it is free. If a collision occurs, the signal is corrupted for all stations, and the transmitter waits for a random delay before retrying.

- **CSMA/CD (Carrier Sense Multiple Access with Collision Detection)**: Transmission and collision detection occur simultaneously. The retransmission delay follows a **Binary Exponential Backoff (BEB)** algorithm, where the contention window starts at size 2 and doubles after each collision.

- **CSMA/CA (Carrier Sense Multiple Access with Collision Avoidance)**: An extension of CSMA/CD in which the station continues to sense the channel during the BEB delay to reduce the likelihood of collisions.
