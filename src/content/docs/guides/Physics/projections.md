---
title: Projections
---


## Mercator - `EPSG::3395`

```math
x = a \lambda
```
> $a$ – semi-major axis of the ellipsoid (radius at the Equator)
>
> $\lambda$ – longitude (in radians)

```math
y = a \tanh^{-1}{\sin{(\phi)}} - a e \tanh^{-1}{(e \sin{\phi})}
```
> $a$ – semi-major axis of the ellipsoid (radius at the Equator)
>
> $e$ – eccentricity of the ellipsoid (shape of the ellipsoid)
>
> $\phi$ – latitude (in radians)

## Web Mercator - `EPSG::3857`

Earth is a perfect square, cutoff at 85.0511°

```math
x = a \lambda
```
> $a$ – semi-major axis of the ellipsoid (radius at the Equator)
>
> $\lambda$ – longitude (in radians)

```math
y = a \tanh^{-1}{\sin{(\phi)}}
```
> $a$ – semi-major axis of the ellipsoid (radius at the Equator)
>
> $\phi$ – latitude (in radians)

> https://www.sedris.org/wg8home/Documents/WG80605.pdf


## Links

> https://ihatecoordinatesystems.com/
