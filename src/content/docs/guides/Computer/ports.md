---
title: Ports
description: Computer ports and protocols.
---

## Application Ports

### Difference between Ports

- **Hardware port**: Refers to the physical interface where a cable must be connected.
- **Software port**: A point of entry for communication using a specific protocol.

The software port is also called an **application port** because it matches a 16-bit coded number (ranging from 1 to 65536) with an application.

### Common Network Ports

- Port `0` to `1023` are standardized

- 20 TCP – FTP Data: File transfer
- 21 TCP – FTP Control: File transfer commands
- **22 TCP** – SSH: Secure remote login
- 23 TCP – Telnet: Unencrypted remote login
- 25 TCP – SMTP: Sending email
- **53 TCP/UDP** – DNS: Domain Name System
- 67 UDP – DHCP Server: Assign IP addresses
- 68 UDP – DHCP Client: Request IP addresses
- 69 UDP – TFTP: Trivial File Transfer Protocol
- **80 TCP/UDP** – HTTP: Web traffic
- 110 TCP – POP3: Email retrieval
- 123 UDP – NTP: Network Time Protocol (clock sync)
- 139 TCP – NetBIOS Session Service: Windows file sharing
- 143 TCP – IMAP: Email retrieval
- 389 TCP/UDP – LDAP: Directory services
- **443 TCP/UDP** – HTTPS: Secure web traffic
- 587 TCP – SMTP Submission
- 993 TCP – IMAPS: IMAP over SSL/TLS
- 995 TCP – POP3S: POP3 over SSL/TLS
- 3306 TCP – MySQL / MariaDB
- 3389 TCP – RDP: Remote Desktop Protocol
- 5432 TCP – PostgreSQL
- 5900 TCP – VNC: Remote desktop protocol
- 6000–6007 TCP – X11 Display Server (graphical interface)
- 6379 TCP – Redis
- 8080 TCP – HTTP Alternate / Proxy
- 27017 TCP – MongoDB
