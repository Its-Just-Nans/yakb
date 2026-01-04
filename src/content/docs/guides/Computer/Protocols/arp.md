---
title: ARP
---

## ARP (Address Resolution Protocol)

ARP (Address Resolution Protocol) is a fundamental networking protocol used to map a **logical IP address (Layer 3)** to a **physical MAC address (Layer 2)** within a local network.

Even though ARP is closely related to IP, its packets are encapsulated directly inside **Ethernet frames**, which is why ARP is often described as operating at **Layer 2.5**—a layer between the Data Link layer and the Network layer.

When a device wants to communicate with another device on the same local network, it must know the destination’s MAC address.

### How it works

- If this address is unknown, the device sends an **ARP request** as a broadcast frame asking, “Who has this IP address?”.
- The device owning that IP responds with an **ARP reply** containing its MAC address.

This information is then stored temporarily in an **ARP cache** to reduce future requests. ARP is essential for local communication but does not provide security or reliability features, making it vulnerable to attacks such as ARP spoofing.
