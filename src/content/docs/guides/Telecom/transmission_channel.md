---
title: "Transmission Channel"
---

- `BER`: Bit Error Rate, number of bit errors
- `SER`: Symbol Error Rate, number of symbol errors

## 1. Source Coding

**Source coding** is used to compress the data to be transmitted in order to reduce the number of bits that need to be sent.

## 2. Channel Coding

**Channel coding** adds redundancy to the data to be transmitted. This is used to correct potential errors (noise, interference, etc.) during transmission.

## 3. Digital Modulator

The modulator transmits bits in groups called **symbols**. Symbols form an **alphabet**.

Example of a 4-symbol alphabet:
m₁ = 00, m₂ = 01, m₃ = 10, m₄ = 11

The digital modulator is composed of two parts:

- the **encoder** converts the symbol `mₖ` into a set of parameters `(aₖ, φₖ)`
- the **modulator** constructs a signal `Sₖ(t)` of duration `Tₛ` (symbol duration) using the parameters `aₖ` and `φₖ`

## 4. The Channel

The channel acts like a **band-pass filter**. It also adds **noise** and causes **amplitude and phase distortion** to the signal.

The received signal is therefore:

$$$
x(t) = S'ₖ(t) + w(t)
$$$

> Caption:
>
> - `x(t)`: received signal
> - `S'ₖ(t)`: the signal distorted by the band-pass filter
> - `w(t)`: the noise

## 5. Digital Demodulator

The demodulator transforms the signal into a sequence of bits.

The digital demodulator consists of two parts:

- the **detector** retrieves the parameters `(aₖ, φₖ)`
- the **decoder** reconstructs a symbol from the signal `x(t)` and the parameters `(aₖ, φₖ)`. This value may be incorrect if the signal is degraded.

## 6. Channel Decoding

**Channel decoding** uses the added redundancy to correct errors.

## 7. Source Decoding

**Source decoding** decompresses the bit sequence.

The final signal may differ from the original signal due to residual errors.
