# voxel-preview
Faux 3D LED Cube in Rust

# About
This is a test client which renders an [8x8x8 LED cube display](https://www.aliexpress.com/item/DIY-3D8-multicolor-mini-LED-light-display-Excellent-animation-3D-8-8x8x8-Electronic-Kits-Junior/32700909987.html). The cube listens over UDP for OSC packets and updates voxels accordingly.

# Usage
    
```
j@mes:~$ cargo run 127.0.0.1:1234
j@mes:~$ send_osc 1234 /fill/solid/grad ,ffffffffiiiiii 1. 0. 1. 1. 0. 1. 0.  1. 0 0 0 7 7 7           
j@mes:~$ send_osc 1234 /dsc/shell/grad ,iiiiffffffffiiiiii 8 8 8 8 1. 0. 0.  1. 1. 1. 0. 1. 0 0 8 8 8 0
j@mes:~$ send_osc 1234 /dsc/cuboid ,iiiiiiiiiiiiffff 0 0 0 1 0 0 0 2 0 0 0 3 0. 0. 1. 1.
```

# Dependencies
  - [`kiss3d`](http://kiss3d.org/) for 3d rendering
  - [`rosc`](https://github.com/klingtnet/rosc) for osc protocol
  - `nalgebra`

# Example Render
![render](render.png)

# Schema 

*NB*: All color values must be floating points between 0 and 1. All other values
must be integers. Types are denoted inline.

```
                 point color
                 i i i f f f f
/dsc/voxel       x y z r g b a            

                 point vector color
                 i i i i i i  f f f f
/dsc/line        x y z i j k  r g b a

                 point vector color       color       point    vector
                 i i i i i i  f  f  f  f  f  f  f  f  i  i  i  i  i  i
/dsc/line/grad   x y z i j k  r1 g1 b1 a1 r2 g2 b2 a2 cx cy cz ci cj ck

                 point vector   vector   color
                 i i i i  i  i  i  i  i  f f f f
/dsc/plane       x y z u1 v1 w1 u2 v2 w2 r g b a 

                 point vector   vector   color       color       point    vector
                 i i i i  i  i  i  i  i  f  f  f  f  f  f  f  f  i  i  i  i  i  i
/dsc/plane/grad  x y z u1 v1 w1 u2 v2 w2 r1 g1 b1 a1 r2 g2 b2 a2 cx cy cz ci cj ck

                 point vector   vector   color
                 i i i i  i  i  i  i  i  f f f f
/dsc/frame       x y z u1 v1 w1 u2 v2 w2 r g b a 

                 point vector   vector   color       color       point    vector
                 i i i i  i  i  i  i  i  f  f  f  f  f  f  f  f  i  i  i  i  i  i
/dsc/frame/grad  x y z u1 v1 w1 u2 v2 w2 r1 g1 b1 a1 r2 g2 b2 a2 cx cy cz ci cj ck

                 point vector   vector   vector   color
                 i i i i  i  i  i  i  i  i  i  i  f f f f
/dsc/cuboid      x y z u1 v1 w1 u2 v2 w2 u3 v3 w3 r g b a

                 point vector   vector   vector   color       color       point    vector
                 i i i i  i  i  i  i  i  i  i  i  f  f  f  f  f  f  f  f  i  i  i  i  i  i
/dsc/cuboid/grad x y z u1 v1 w1 u2 v2 w2 u3 v3 w3 r1 g1 b1 a1 r2 g2 b2 a2 cx cy cz ci cj ck

                 point radius color
                 i i i i      f f f f
/dsc/sphere      x y z p      r g b a        

                 point radius color       color       point    vector
                 i i i i      f  f  f  f  f  f  f  f  i  i  i  i  i  i
/dsc/sphere/grad x y z p      r1 g1 b1 a1 r2 g2 b2 a2 cx cy cz ci cj ck

                 point radius color
                 i i i i      f f f f
/dsc/shell       x y z p      r g b a        

                 point radius color       color       point    vector
                 i i i i      f  f  f  f  f  f  f  f  i  i  i  i  i  i
/dsc/shell       x y z p      r1 g1 b1 a1 r2 g2 b2 a2 cx cy cz ci cj ck

                 color
                 f f f f
/fill/solid      r g b a

                 color       color       point    vector
                 f  f  f  f  f  f  f  f  i  i  i  i  i  i
/fill/solid/grad r1 g1 b1 a1 r2 g2 b2 a2 cx cy cz ci cj ck
```

