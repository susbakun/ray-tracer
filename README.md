# Rust Ray Tracer

A path tracer written in Rust, following the ideas from the
[*Ray Tracing in One Weekend*](https://raytracing.github.io/) book series by
Peter Shirley, Steve Marschner, and others.

The renderer writes ASCII PPM images to stdout and supports recursive path
tracing, antialiasing, depth of field, BVH acceleration, texture mapping,
emissive lights, participating media, and row-parallel rendering with Rayon.

## Reference

This project is a Rust implementation inspired by the free online books:

**[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)**

It also includes later-book features such as quadrilaterals, boxes, lights,
BVH, Perlin noise, image textures, and constant-density volumes.

## Features

- **Primitives** — stationary and moving spheres, quadrilaterals, and boxes
- **Materials**
  - `Lambertian` — diffuse reflection
  - `Metal` — specular reflection with optional fuzz
  - `Dielectric` — refraction, reflection, and Fresnel effects (glass)
  - `DiffuseLight` — emissive surfaces
  - `Isotropic` — scattering inside constant media
- **Textures** — solid colors, checker textures, image textures, and Perlin noise
- **Acceleration** — bounding boxes and BVH traversal
- **Camera** — adjustable field of view, look-at orientation, antialiasing, motion blur, and depth of field
- **Parallel rendering** — image rows are rendered in parallel with Rayon, then written in image order
- **Scenes** — bouncing spheres, checker spheres, Earth texture, Perlin spheres, quads, simple lights, Cornell box, Cornell smoke, and the final scene

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

## Build & Run

```bash
cargo build --release
cargo run --release > image.ppm
```

The active scene is selected in `src/main.rs`:

```rust
let scene = 9;
```

Scene `7` renders the Cornell box, and scene `8` renders the Cornell smoke
volume. For quicker previews, lower `image_width` and `samples_per_pixel` in
the selected scene setup.

View the output with any PPM viewer, or convert it with ImageMagick:

```bash
magick image.ppm image.png
```

## Project Structure

```
src/
├── main.rs          # Scene setup and render entry point
├── camera.rs        # Camera, ray generation, path tracing, and parallel render loop
├── bvh.rs           # Bounding volume hierarchy
├── aabb.rs          # Axis-aligned bounding boxes
├── color.rs         # Color type and PPM output (with gamma correction)
├── constant_medium.rs # Constant-density participating media
├── hittable.rs      # Hit record and Hittable trait
├── hittable_list.rs # Scene object list
├── quad.rs          # Quad primitive and box construction
├── sphere.rs        # Sphere primitive
├── material.rs      # Surface and volume scattering materials
├── texture.rs       # Solid, checker, image, and noise textures
├── perlin.rs        # Perlin noise
├── ray.rs           # Ray type
├── vector.rs        # Vec3, Point3, and vector math
├── interval.rs      # Min/max interval utility
├── rtw_image.rs     # Image loading helper
└── prelude/         # Random sampling helpers and traits
```

## Parallel Rendering

Rendering is parallelized across image rows in `camera.rs` with Rayon. Each row
is rendered into an in-memory image buffer using a local random number
generator, then the completed buffer is written to stdout in deterministic
image order.

This keeps stdout writes out of worker threads and avoids sharing mutable camera
state during rendering.

## Output Format

Images are written as ASCII PPM (`P3`) to stdout:

```
P3
<width> <height>
255
<r> <g> <b>
...
```

Redirect stdout to a file to save the image (`> image.ppm`).

## License

This is a learning project. The original book and code are available under the [CC0 license](https://creativecommons.org/publicdomain/zero/1.0/) on [raytracing.github.io](https://raytracing.github.io/).
