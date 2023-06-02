# Glium-Obj

A very basic and simple OBJ viewer, based on the glium teapot example. The
Obj files *MUST* be triangulated before and they *MUST* contain normals. The
majority of the OBJ files on the net are lacking this.

It works with OBJ files imported/triangulated and exported from blender(TM).

# Usage

Provide the file name to be viewed on the command line.

* Use the `QWEASD`-Keys to rotate the object.
* Use + and - to scale the object.
* Use the keypad to change the viewer position and direction.

# Build

```/bin/sh
$ cargo build --release
```
