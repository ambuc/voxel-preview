# voxel-preview
Faux 3D LED Cube in Rust

# About
This is a test client which renders an [8x8x8 LED cube display](https://www.aliexpress.com/item/DIY-3D8-multicolor-mini-LED-light-display-Excellent-animation-3D-8-8x8x8-Electronic-Kits-Junior/32700909987.html). The cube listens over UDP for OSC packets and updates voxels accordingly.

# Dependencies
  - [`kiss3d`](http://kiss3d.org/) for 3d rendering
  - [`rosc`](https://github.com/klingtnet/rosc) for osc protocol
  - `mio` for async udp i/o
  - `nalgebra`

# Example
![render](render.png)

# OSC Schema
```
//       port  x y z r   g   b
send_osc 1234 /0/2/4 0.0 0.5 1.0
```
*NB*: 
 - `x`, `y`, and `z` must be in the range `[0..N)`, where `N` is the static `CUBE_WIDTH`. 
 - `r`, `g`, and `b` must be in the range `[0..1]`, to be valid RGB values.

