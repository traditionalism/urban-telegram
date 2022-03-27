# urban-telegram
My second little 'project' within the rust-lang now using bevy-engine!

### Screenshots
Sorry! I'm lazy and have none.

## Rudiments
**Due to the defaulted use of LLD-Linking this program requires you to running the nightly rust toolchain, see the LLD-Linking section to remove it.**

```
cargo install cargo-make
```
### Building Native
```
cargo make run
```

### LLD-Linking
urban-telegram deploys LLD Linker to expedite the compilation process. To get rid of LLD-Linking delete `.cargo/config.toml`
See: [Bevy Fast-Compiles](https://bevyengine.org/learn/book/getting-started/setup/#enable-fast-compiles-optional)

## Credits And Other Notes

### Credit(s)
* [bevy](https://github.com/bevyengine/bevy)
* [Khronos-Group](https://www.khronos.org/gltf/)

### License
* [WTFPL](https://choosealicense.com/licenses/wtfpl/)

### Closing Notes
**Made with 💝 by traditionalism**