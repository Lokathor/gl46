cargo install phosphorus
mkdir target
phosphorus ../gl.xml gl 4 6 core GL_EXT_texture_filter_anisotropic >target/lib.rs
rustfmt target/lib.rs
