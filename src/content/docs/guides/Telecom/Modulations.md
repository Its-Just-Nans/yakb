---
title: "Modulations"
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

## MSK

**MSK** (Minimum Shift Keying), used in GSM, is a variant of FSK where the signal is coherently demodulated.

---

## GMSK

**GMSK** (Gaussian Minimum Shift Keying) is a variant of MSK that uses a Gaussian filter to reduce bandwidth.
