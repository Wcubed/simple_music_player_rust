Uses [mpv](https://mpv.io/) as back-end for playing the music.
This needs to be installed to run the program.

# Building

For more info on the mpv part of the build, see the [libmpv-rs](https://github.com/ParadoxSpiral/libmpv-rs) github readme.

## Linux
- Clone the [mpv-build](https://github.com/mpv-player/mpv-build) repo into a folder you can find later.
- Set the `MPV_SOURCE` environment variable to the directory containing the mpv-build repo.
- Run `cargo build` in the directory containing `Cargo.toml`.