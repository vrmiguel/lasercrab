# lasercrab (WIP!)

Lasercrab is a KISS implementation of an al-Haytham's model based ray-tracer, written in Rust.

[![Lasercrab example](https://user-images.githubusercontent.com/36349314/99035080-9102d400-255d-11eb-8991-442675396a75.png)](https://youtu.be/zLz4_43jUxo "Lasercrab Animation")
_Click on the image above to watch a small animation made with Lasercrab. Hosted on YouTube._


## Building

Lasercrab has no dependencies by default, but [Rayon](https://github.com/rayon-rs/rayon) is available as an optional dependency, which can parallelize the rendering of frames.
It was developed on rustc 1.47.0, but probably also builds on much older rustc versions.

### Building without Rayon

```
cargo run --release
```

### Building with Rayon

```
cargo run --release --features parallel
```

_Warning_: by default, this will render 599 frames at 1080p, where each one takes 5.9MiB of space.

## Usage

As it stands, Lasercrab works on a [suckless](https://suckless.org/)-like manner.


Spheres, lights and materials are all defined in `main.rs`, alongside other configurations such as the output's image width and height.
Colors, such as the ones used on the checkerboard or the background, may be changed altering the `const Vec3fs` in `shapes.rs`. 

`ffmpeg` is a good companion to Lasercrab when making animations, with an example usage being:
```bash
ffmpeg -framerate 60 -i output_%d.ppm -c:v libx264 -pix_fmt yuv420p output_anim.mp4
```

## TODO

Things I want (or may) implement in the future

* Snell's law based refractions;
* Add command-line arguments using [Clap](https://github.com/clap-rs/clap);
* Add the [image](https://crates.io/crates/image) crate as an optional dependency.



