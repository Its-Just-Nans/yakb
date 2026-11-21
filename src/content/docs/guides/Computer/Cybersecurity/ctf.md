---
title: CTF
---

CTF stands for Capture the Flag, a cybersecurity competition where participants solve security challenges to find hidden pieces of data (called "flags") to earn points.

## Cheatsheet

- Check if there are `.git` folder
- Check the `/proc/${PID}`
- Check the metadata of the file (Exif)
- Check for non-deleted files `~/.local/share/Trash/`

## Windows

- <https://github.com/volatilityfoundation/volatility3>

```sh
vol -f memory.dmp windows.netscan
# 10.0.2.15
vol -f memory.dmp windows.pstree
# C:\Users\user\AppData
vol -f memory.dmp windows.registry.hivelist
# 0xb88510266000    \REGISTRY\MACHINE\SYSTEM
vol -f memory.dmp windows.registry.printkey
vol -f memory.dmp windows.session
# DESKTOP-JV996VQ

vol -f memory.dmp windows.filescan
# addr 0xa50a2a42abe0
mkdir export
vol -f memory.dmp -o export windows.dumpfiles --virtaddr 0xa50a2a42abe0
```

## Links

- <https://sadservers.com> - DevOps
- <https://beacondb.net/> - wireless geolocation database
- <https://crackmes.one/> - reverse
- <https://www.newbiecontest.org/>
- <https://github.com/iggredible/Learn-Vim> - learn vim
- <https://vim.rtorr.com/> - vim cheat sheet
- <https://www.root-me.org/>
- <https://pwn.college/>
- <https://microcorruption.com/> - pwn / iot
- <https://ctf101.org/> - documentation
- <https://www.hackthebox.com/> - pentest
- <https://cryptohack.org/>
- <https://www.newbiecontest.org/>
- <https://www.france-ioi.org/>
- <https://fotoforensics.com/>
- <https://france-cybersecurity-challenge.fr/>
- <https://www.nops.re/>
- <https://ph0wn.org/> - smart devices
- <https://insomnihack.ch/>
- <https://owasp.org/>
- <https://www.hackerone.com/>
- <https://hackathon.anfr.fr/>
- <https://www.breizhctf.com/>
- <https://midnightflag.fr/>
- <https://hackropole.fr/>
- <https://fcsc.fr/>
- <https://insomnihack.ch/contest/>
- <https://angr.io/>
- <https://fr.wikipedia.org/wiki/Cicada_3301>

