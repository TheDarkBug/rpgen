# Rust Pattern Generator
As the name suggests, this is a **Rust Pattern GENerator**.
`rpgen` generates some patterns based on the position of a pixel in an array of pixels. The generated pixel values get saved in the [ppm format](https://it.wikipedia.org/wiki/Netpbm).

## Building
```console
$ cargo build --release
```
## Usage:
```console
$ ./target/release/rpgen [options]
```

## Examples
Render all the patterns in default-sized images (512*512):
```console
$ ./target/release/rpgen -r
```
Render the patterns 4 to 16 in default-sized images (512*512):
```console
$ ./target/release/rpgen -r -f 4:16
```
Render the pattern with id 7, in a 1900x4500 image and show it:
```console
$ ./target/release/rpgen -crsh 1900 -w 4500 -i 7
```
Other options:
```console
$ ./target/release/rpgen --help
```

## License
This program is provided under the [GPL v3 license](https://github.com/TheDarkBug/rpgen/blob/main/LICENSE).