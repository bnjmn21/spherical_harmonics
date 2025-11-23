A script to generate meshes (.stl) of (complex) [spherical harmonics](https://en.wikipedia.org/wiki/Spherical_harmonics) for 3d printing.

You may want to use the pregenerated meshes at 

## Usage

1. Create a sphere in a CAD program, with your desired size and resolution.
1. Export it as an stl file to `sphere.stl`.
1. Select the functions to generate by changing the `L_M_PAIRS` constant.
1. Select a radius for the support ball in the center by changing the `SUPPORT_BALL_SIZE`.
1. `cargo run` it.
1. The results will be output to the `results` folder. 

Note that the generated models will be a bit smaller than input original sphere.

Additionally this repo contains a python script to optimize the meshes.

## Credits

- [sphrs](https://crates.io/crates/sphrs) for computing spherical harmonics
- [stl_io](https://docs.rs/stl_io/latest/stl_io/) for reading/writing stl files