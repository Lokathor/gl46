#![no_std]
#![allow(bad_style)]

//! Makes the OpenGL 4.6 Core API (plus some extensions) available for use.
//!
//! The crate's interface is provided as a "struct" style loader. Construct a
//! [`GlFns`] using an appropriate "gl_get_proc_address" function and then call
//! its methods.
//!
//! ## Extensions
//!
//! * [GL_ARB_texture_filter_anisotropic](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_texture_filter_anisotropic.txt)
//! * [GL_ARB_bindless_texture](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_bindless_texture.txt)
//! * [GL_ARB_sparse_texture](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_sparse_texture.txt)
//! * [GL_ARB_pipeline_statistics_query](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_pipeline_statistics_query.txt)
//!
//! ## Cargo Features
//!
//! * `track_caller`: Enables the [track_caller][track_caller_reference]
//!   attribute on any function that can panic. Specifically, the extension
//!   functions of the struct loader might not be loaded, and if you call them
//!   when they're not loaded you'll get a panic.
//!
//! [track_caller_reference]:
//! https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute
//!
//! ## `gl_get_proc_address`
//!
//! GL must generally be dynamically loaded at runtime. This is done via a
//! function which we'll call "gl_get_proc_address". The expected operation of
//! this function is very simple: The caller passes in the pointer to a
//! null-terminated string with the name of a GL function, and
//! `gl_get_proc_address` returns the pointer to that function.
//!
//! The way that you get an appropriate `gl_get_proc_address` function is
//! platform dependent.
//! * With Win32 you'd use [wglGetProcAddress][wglGetProcAddress] for any
//!   function from *after* OpenGL 1.1, and [GetProcAddress][GetProcAddress] on
//!   an open `HMODULE` to "OpenGL.dll" for any OpenGL function from either 1.1
//!   or 1.0. That sounds silly, but it's true.
//! * With SDL2 you'd call [SDL_GL_GetProcAddress][SDL_GL_GetProcAddress], or
//!   the equivalent function within your SDL2 bindings.
//! * With [glutin][glutin] you'd call [Context::get_proc_address][glutin-gpa]
//!
//! The function to create a `GLFns` takes a `&dyn Fn(*const u8) -> *const
//! c_void`. Note the `&dyn` part on the front. In addition to passing a closure
//! to the constructor (`|x| { ... }`), you generally need to prefix your
//! closure with a `&` to make it be a `&dyn` value. Like this:
//! ```no_run
//! use gl46::*;
//! # let SDL_GL_GetProcAddress: fn(*const u8) -> *mut core::ffi::c_void = unimplemented!();
//! let gl = unsafe { GlFns::load_from(&|u8_ptr| SDL_GL_GetProcAddress(u8_ptr.cast())).unwrap() };
//! ```
//! It might seem a little silly, but it genuinely helps keep re-compile times
//! down, and it's just one extra `&` to write.
//!
//! [wglGetProcAddress]:
//! https://docs.microsoft.com/en-us/windows/win32/api/wingdi/nf-wingdi-wglgetprocaddress
//!
//! [GetProcAddress]:
//! https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress
//!
//! [SDL_GL_GetProcAddress]: https://wiki.libsdl.org/SDL_GL_GetProcAddress
//!
//! [glutin]: https://docs.rs/glutin/0.26.0/glutin
//!
//! [glutin-gpa]:
//! https://docs.rs/glutin/0.26.0/glutin/struct.Context.html#method.get_proc_address
//!
//! ## Inlining
//!
//! This crate does **not** use the `#[inline]` attribute. If you want full
//! inlining just turn on Link-Time Optimization in your cargo profile:
//!
//! ```toml
//! [profile.release]
//! lto = "thin"
//! ```

use chlorine::*;

// TODO: make the gl_command_types module pub(crate), but it's a breaking
// change.
pub mod gl_command_types;
pub(crate) use gl_command_types::*;

pub mod gl_core_types;
pub use gl_core_types::*;

pub mod gl_enumerations;
pub use gl_enumerations::*;

pub mod gl_groups;
pub use gl_groups::*;

pub(crate) mod struct_loader;
pub use struct_loader::*;
