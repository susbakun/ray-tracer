# Ray Tracer

A path tracer written in Rust, following the concepts and scene setup from [*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html) by Peter Shirley, Steve Marschner, and others.

The renderer outputs PPM images to stdout and supports diffuse, metal, and dielectric materials with antialiasing, depth of field, and recursive ray bouncing.

## Reference

This project is a Rust implementation inspired by the free online book:

**[Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)**

If you are learning along with the book, the code structure maps closely to its chapters: rays, hit detection, materials, and a configurable camera.

## Features

- **Primitives** — spheres with ray–sphere intersection
- **Materials**
  - `Lambertian` — diffuse reflection
  - `Metal` — specular reflection with optional fuzz
  - `Dielectric` — refraction, reflection, and Fresnel effects (glass)
- **Camera** — adjustable field of view, look-at orientation, antialiasing, and depth of field
- **Scene** — the final image from the book: random small spheres on a ground plane plus three large focal objects

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

## Build & Run

```bash
cargo build --release
cargo run --release > image.ppm
```

The default scene renders at 1200 px wide with 500 samples per pixel, which can take a while. For a quicker preview, lower `image_width` and `samples_per_pixel` in `src/main.rs`.

View the output with any PPM viewer, or convert it with ImageMagick:

```bash
magick image.ppm image.png
```

## Project Structure

```
src/
├── main.rs          # Scene setup and render entry point
├── camera.rs        # Camera, ray generation, and path tracing loop
├── color.rs         # Color type and PPM output (with gamma correction)
├── hittable.rs      # Hit record and Hittable trait
├── hittable_list.rs # Scene object list
├── sphere.rs        # Sphere primitive
├── material.rs      # Lambertian, Metal, Dielectric
├── ray.rs           # Ray type
├── vector.rs        # Vec3, Point3, and vector math
├── interval.rs      # Min/max interval utility
└── prelude/         # Random sampling helpers and traits
```

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
