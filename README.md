# voxel-preview
test bed for voxel display rendering using kiss3d in rust

# About
This is just a test of [kiss3d](http://kiss3d.org/) for rendering a [3d LED
display](https://www.aliexpress.com/item/DIY-3D8-multicolor-mini-LED-light-display-Excellent-animation-3D-8-8x8x8-Electronic-Kits-Junior/32700909987.html). Ideally I'll put in controls for accepting data over OSC (probably with
[rosc](https://github.com/klingtnet/rosc).

# Roadmap
[x] Render with `kiss3d`
[x] Write `rosc` sender loop
[x] Write `rosc` reciever loop
[ ] Render image continuously with osc input
[ ] Sender loop generates patterns
[ ] Launch both from command  line

## Example
![render](render.png)

## OSC Schema
```
  #        port /px/x/y/z r   g   b
  send_osc 1234 /px/0/2/4 0.0 0.5 1.0
```
