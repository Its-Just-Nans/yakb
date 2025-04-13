---
title: "Modulations"
sidebar:
    order: 3
---

To transmit the signal, we modulate it using various modulation techniques.

If we want to reduce the frequency bandwidth occupied by the signal, we use raised cosine or Gaussian pulses instead of square pulses.

We measure the efficiency (which we aim to maximize) of the modulation using the following formula:

$$$
p = \frac{R_b}{B}
$$$

> Caption:
>
> - `p`: efficiency
> - `R_b`: source bitrate
> - `B`: bandwidth

---

## ASK

_**A**mplitude-**S**hift **K**eying_.

Modulation is done by varying the **amplitude**.

|                                                                                                        Operation                                                                                                         |
| :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: |
| ![ASK Example](https://www.researchgate.net/profile/Ahmad_Fauzi_Abas/publication/221907745/figure/fig1/AS:670028466765837@1536758643863/Example-of-ASK-modulation-foramt-a-binary-signal-and-b-ASK-modulated-signal.png) |

---

## FSK

_**F**requency-**S**hift **K**eying_ (also called **FSK** - Frequency Shift Keying in French too).

Modulation is done by varying the **frequency**.

|                              Operation                              |
| :-----------------------------------------------------------------: |
| ![FSK](https://upload.wikimedia.org/wikipedia/commons/3/39/Fsk.svg) |

---

## MSK

**MSK** (Minimum Shift Keying), used in GSM, is a variant of FSK where the signal is coherently demodulated.

---

## GMSK

**GMSK** (Gaussian Minimum Shift Keying) is a variant of MSK that uses a Gaussian filter to reduce bandwidth.

---

## PSK

_**P**hase-**S**hift **K**eying_.

Modulation is done by varying the **phase**.

There are several types of PSK modulations based on how many phases are used:

---

### BPSK

_**B**inary **P**hase-**S**hift **K**eying_.  
A PSK modulation with only **two phases**.

|                                        Operation                                        |                                     Constellation Diagram                                      |
| :-------------------------------------------------------------------------------------: | :--------------------------------------------------------------------------------------------: |
| ![BPSK](https://i.pcmag.com/imagery/encyclopedia-terms/psk-_psk.fit_lim.size_1050x.gif) | ![BPSK constellation](https://upload.wikimedia.org/wikipedia/commons/4/41/BPSK_Gray_Coded.svg) |

---

### DPSK

**DPSK** is a phase modulation that uses the **previous symbol’s phase** to encode the current symbol.

---

### QPSK

_**Q**uadrature **P**hase-**S**hift **K**eying_ (also called **4-PSK**).

A PSK modulation with **four different phases**.

|                                        Operation                                        |                                     Constellation Diagram                                      |
| :-------------------------------------------------------------------------------------: | :--------------------------------------------------------------------------------------------: |
| ![QPSK](https://i.pcmag.com/imagery/encyclopedia-terms/psk-_qpsk.fit_lim.size_800x.gif) | ![QPSK constellation](https://upload.wikimedia.org/wikipedia/commons/8/8f/QPSK_Gray_Coded.svg) |

---

## QAM

_**Q**uadrature **A**mplitude **M**odulation_ (in French: MAQ - modulation d'amplitude en quadrature).

QAM combines **amplitude modulation** and **phase modulation** to transmit more bits per symbol. It is widely used in modern communication systems like Wi-Fi, 4G/5G, and digital TV.
