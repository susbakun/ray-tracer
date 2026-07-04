# Rust Ray Tracer

A path tracer written in Rust, following the ideas from the
[*Ray Tracing in One Weekend*](https://raytracing.github.io/) book series by
Peter Shirley, Steve Marschner, and others.

The renderer writes ASCII PPM images to stdout and supports recursive path
tracing, antialiasing, depth of field, BVH acceleration, texture mapping,
emissive lights, participating media, OBJ mesh loading, geometric
transformations, and row-parallel rendering with Rayon.

## Reference

This project is a Rust implementation inspired by the free online books:

**[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)**

It also includes later-book features such as quadrilaterals, boxes, lights,
BVH, Perlin noise, image textures, and constant-density volumes, plus
extensions beyond the book series: triangle meshes, Wavefront OBJ models, and
translate / rotate / scale transformations.

## Features

- **Primitives** — stationary and moving spheres, quadrilaterals, boxes,
  triangles, and triangle meshes
- **Models** — Wavefront `.obj` loading via `tobj`, with per-vertex normals
  and UVs, triangulated and wrapped in a BVH
- **Materials**
  - `Lambertian` — diffuse reflection
  - `Metal` — specular reflection with optional fuzz
  - `Dielectric` — refraction, reflection, and Fresnel effects (glass)
  - `DiffuseLight` — emissive surfaces
  - `Isotropic` — scattering inside constant media
- **Textures** — solid colors, checker textures, image textures, and Perlin noise
- **Transformations** — uniform scale, translation, and rotation wrappers around
  any `Hittable`
- **Acceleration** — bounding boxes and BVH traversal
- **Camera** — adjustable field of view, look-at orientation, antialiasing,
  motion blur, and depth of field
- **Parallel rendering** — image rows are rendered in parallel with Rayon, then
  written in image order; a progress bar tracks row completion
- **Scenes** — bouncing spheres, checker spheres, Earth texture, Perlin
  spheres, quads, simple lights, Cornell box, Cornell smoke, the final scene,
  a single-triangle test scene, and the Utah teapot

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

## Build & Run

```bash
cargo build --release
cargo run --release > image.ppm
```

The active scene is selected in `src/main.rs`:

```rust
let scene = 11;
```

| Scene | Description |
|-------|-------------|
| 1 | Bouncing spheres |
| 2 | Checker spheres |
| 3 | Earth texture |
| 4 | Perlin noise spheres |
| 5 | Quadrilaterals |
| 6 | Simple light |
| 7 | Cornell box |
| 8 | Cornell smoke volume |
| 9 | Final scene (800×450, 10 000 spp) |
| 10 | Single triangle |
| 11 | Utah teapot (default) |

For quicker previews, lower `image_width` and `samples_per_pixel` in the
selected scene setup inside `src/scenes.rs`.

View the output with any PPM viewer, or convert it with ImageMagick:

```bash
magick image.ppm image.png
```

## Assets

Static files live under `assets/`:

```
assets/
├── models/     # Wavefront OBJ files (e.g. utah_teapot.obj)
└── textures/   # Image textures referenced by scene setups
```

Models are loaded from `assets/models/`; image textures from
`assets/textures/`. Run the binary from the project root so these relative
paths resolve correctly.

## Project Structure

```
src/
├── main.rs              # Scene selection and render entry point
├── scenes.rs            # All scene setups
├── camera.rs            # Camera, ray generation, path tracing, and parallel render loop
├── bvh.rs               # Bounding volume hierarchy
├── aabb.rs              # Axis-aligned bounding boxes
├── color.rs             # Color type and PPM output (with gamma correction)
├── hittable.rs          # Hit record and Hittable trait
├── hittable_list.rs     # Scene object list
├── material.rs          # Surface and volume scattering materials
├── texture.rs           # Solid, checker, image, and noise textures
├── perlin.rs            # Perlin noise
├── ray.rs               # Ray type
├── vector.rs            # Vec3, Point3, and vector math
├── matrix.rs            # 4×4 transformation matrices
├── interval.rs          # Min/max interval utility
├── rtw_image.rs         # Image loading helper
├── model.rs             # OBJ model loader (triangles + BVH)
├── mesh.rs              # Simple triangle mesh container
├── shape/               # Geometric primitives
│   ├── sphere.rs
│   ├── quad.rs
│   ├── triangle.rs
│   └── constant_medium.rs
├── transformation/      # Hittable wrappers
│   ├── translate.rs
│   ├── rotate.rs
│   └── scale.rs
└── prelude/             # Shared types, constants, and random sampling helpers
```

## Parallel Rendering

Rendering is parallelized across image rows in `camera.rs` with Rayon. Each row
is rendered into an in-memory image buffer using a local random number
generator, then the completed buffer is written to stdout in deterministic
image order.

This keeps stdout writes out of worker threads and avoids sharing mutable camera
state during rendering. An `indicatif` progress bar reports elapsed time and ETA
while rows are being traced.

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
