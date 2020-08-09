[![License:Zlib](https://img.shields.io/badge/License-Zlib-brightgreen.svg)](https://opensource.org/licenses/Zlib)
![min-rust-1.34](https://img.shields.io/badge/Min%20Rust-1.34-green.svg)
[![crates.io](https://img.shields.io/crates/v/gl46.svg)](https://crates.io/crates/gl46)
[![docs.rs](https://docs.rs/gl46/badge.svg)](https://docs.rs/gl46/)

![Unsafe-101-Percent](https://img.shields.io/badge/Unsafety-101%25-red.svg)

# gl46
Bindings to OpenGL 4.6

## Stability

The `gl46` crate presents OpenGL 4.6 bindings for Rust, as described by [gl.xml](https://github.com/KhronosGroup/OpenGL-Registry/blob/master/xml/gl.xml).

As often as `gl.xml` updates I will also attempt to issue updates for this crate.

Because `gles30` follows the current content of `gl.xml` as closely as possible,
it's *possible* (though highly unlikely) that there could be a `gl.xml` update
that would somehow cause a breaking change. This is most likely to occur if an
argument's type changes between signed and unsigned, which is not a big
difference in C but it would cause a type mismatch in Rust (you'd need to add an
`as _` to make it cast the value). In this case, the break is considered a
"required bugfix", and you just have to update your code. Sorry.

### Extensions

The crate is generated with support for the `GL_EXT_texture_filter_anisotropic` extension. This adds two additional `const` values, but no additional GL commands.
