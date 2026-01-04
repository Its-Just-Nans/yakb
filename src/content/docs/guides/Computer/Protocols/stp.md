---
title: STP
---

## Spanning Tree Protocol (STP)

The **Spanning Tree Protocol (STP)** is a network protocol designed to prevent **loops** in Ethernet networks that contain redundant paths.

While redundancy improves fault tolerance, it can cause serious issues such as broadcast storms, multiple frame copies, and MAC address table instability. STP solves this by logically transforming the physical network into a **loop-free tree structure**, known as a *spanning tree*, which is a connected subgraph of the original network without cycles.

### How it works

The operation of STP follows a clear process.

- First, a **root switch (root bridge)** is elected; this is the switch with the **lowest bridge identifier**, which combines priority and MAC address.
- Next, each switch calculates the **shortest path to the root switch**, where the path cost depends on link bandwidth (higher bandwidth means lower cost).
- Based on these calculations, STP determines which ports should be active (forwarding) and which should be blocked to eliminate loops.

Although some physical links are disabled, they can be reactivated automatically if a failure occurs, ensuring both **network stability and resilience**.
