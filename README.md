# gl46
Bindings to OpenGL 4.6

## Stability

The `gl46` crate presents OpenGL 4.6 bindings for Rust, described by [gl.xml](https://github.com/KhronosGroup/OpenGL-Registry/blob/master/xml/gl.xml).

As often as `gl.xml` updates I will also attempt to issue updates for this crate.

Because `gl46` follows the current content of `gl.xml` as closely as possible,
it's *possible* (though highly unlikely) that there could be a `gl.xml` update
that would somehow cause a breaking change. This is most likely to occur if an
argument's type changes between signed and unsigned, which is not a big
difference in C but it would cause a type mismatch in Rust (you'd need to add an
`as _` to make it cast the value).

### Extensions

The crate is generated with support for the `GL_EXT_texture_filter_anisotropic` extension. This adds two additional `const` values, but no additional GL commands.
