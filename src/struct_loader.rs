use super::*;

impl GlFns {
  /// Loads all GL functions from the loader given.
  ///
  /// ## Failure
  /// This fails if any non-nullable function does not load.
  /// The error value will be the name of the first non-nullable function that
  /// doesn't load.
  ///
  /// ## Safety
  /// * The "Get Proc Address" function you provide will always be given a
  ///   pointer to the start of a null-terminated string containing the name of
  ///   a GL function to load.
  /// * The "Get Proc Address" function given must always return accurate
  ///   function pointer values, or null on failure.
  pub unsafe fn load_from(f: impl Fn(*const u8) -> *const c_void) -> Result<Self, &'static str> {
    Self::_load_from(&f)
  }
  fn ptr_filter(p: *const c_void) -> Option<core::ptr::NonNull<c_void>> {
    match p as usize {
      // Note(Lokathor): wgl is known to sometimes give phony non-null pointer values.
      0 | 1 | 2 | 3 | usize::MAX => None,
      _ => unsafe { core::mem::transmute(p) },
    }
  }
}
macro_rules! fns {
  (
    $(
      $(#[$m:meta])*
      $(unsafe $(@$unsafe:tt)?)? $(ext $(@$ext:tt)?)? fn $name:ident($($arg:ident : $t:ty),*) $(-> $ret:ty)?;
    )*
  ) => {
    pub struct GlFns {
      $(
        $name: fns!(@field($(true $($ext)?)?) unsafe extern "system" fn($($arg: $t),*) $(-> $ret)*),
      )*
    }
    impl GlFns {
      unsafe fn _load_from(f: &dyn Fn(*const u8) -> *const c_void) -> Result<Self, &'static str> {
        Ok(Self{
          $(
            $name: {
              let s = concat!("gl", stringify!($name), "\0");
              let ptr = Self::ptr_filter(f(s.as_ptr()));
              $($($ext)?
              let ptr = ptr.ok_or(s)?;
              )?
              core::mem::transmute_copy(&ptr)
            },
          )*
        })
      }
    }
    #[doc(hidden)]
    pub mod is_loaded {
      $(
        $($($ext)?
          pub fn $name(fns: &super::GlFns) -> bool {
            fns.$name.is_some()
          }
        )?
      )*
    }
    #[allow(unused_unsafe)]
    impl GlFns {
      $(
        $($($ext)? #[cfg_attr(feature = "track_caller", track_caller)])?
        $(#[$m])*
        pub $(unsafe $($unsafe)?)? fn $name(&self, $($arg : $t),*) $(-> $ret)* {
          let f = self.$name;
          $($($ext)?
            let f = match f {
              Some(f) => f,
              None => panic!(concat!("Function Not Loaded: gl", stringify!($name))),
            };
          )?
          unsafe { (f)($($arg),*) }
        }
      )*
    }
  };
  (@field(true) $t:ty) => {Option<$t>};
  (@field() $t:ty) => {$t};
}
fns! {
  /// glActiveShaderProgram
  /// * `pipeline` class: program pipeline
  /// * `program` class: program
  unsafe fn ActiveShaderProgram(pipeline: GLuint, program: GLuint);
  /// glActiveTexture
  /// * `texture` group: TextureUnit
  unsafe fn ActiveTexture(texture: TextureUnit);
  /// [glAttachShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glAttachShader.xhtml)
  ///
  /// Attaches the given shader object to the given program object. You can
  /// attach more than one shader of the same type to a program.
  ///
  /// * `program` is the program you're attaching the shader object to.
  /// * `shader` is the shader you're attaching.
  fn AttachShader(program: GLuint, shader: GLuint);
  /// glBeginConditionalRender
  /// * `mode` group: ConditionalRenderMode
  unsafe fn BeginConditionalRender(id: GLuint, mode: ConditionalRenderMode);
  /// glBeginQuery
  /// * `target` group: QueryTarget
  /// * `id` class: query
  unsafe fn BeginQuery(target: QueryTarget, id: GLuint);
  /// glBeginQueryIndexed
  /// * `target` group: QueryTarget
  /// * `id` class: query
  unsafe fn BeginQueryIndexed(target: QueryTarget, index: GLuint, id: GLuint);
  /// glBeginTransformFeedback
  /// * `primitiveMode` group: PrimitiveType
  unsafe fn BeginTransformFeedback(primitiveMode: PrimitiveType);
  /// glBindAttribLocation
  /// * `program` class: program
  unsafe fn BindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar);
  /// glBindBuffer
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  unsafe fn BindBuffer(target: BufferTargetARB, buffer: GLuint);
  /// glBindBufferBase
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  unsafe fn BindBufferBase(target: BufferTargetARB, index: GLuint, buffer: GLuint);
  /// glBindBufferRange
  /// * `target` group: BufferTargetARB
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  unsafe fn BindBufferRange(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
  /// glBindBuffersBase
  /// * `target` group: BufferTargetARB
  /// * `buffers` class: buffer
  /// * `buffers` len: count
  unsafe fn BindBuffersBase(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint);
  /// glBindBuffersRange
  /// * `target` group: BufferTargetARB
  /// * `buffers` class: buffer
  /// * `buffers` len: count
  /// * `offsets` len: count
  /// * `sizes` len: count
  unsafe fn BindBuffersRange(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);
  /// glBindFragDataLocation
  /// * `program` class: program
  /// * `name` len: COMPSIZE(name)
  unsafe fn BindFragDataLocation(program: GLuint, color: GLuint, name: *const GLchar);
  /// glBindFragDataLocationIndexed
  /// * `program` class: program
  unsafe fn BindFragDataLocationIndexed(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);
  /// glBindFramebuffer
  /// * `target` group: FramebufferTarget
  /// * `framebuffer` class: framebuffer
  unsafe fn BindFramebuffer(target: FramebufferTarget, framebuffer: GLuint);
  /// glBindImageTexture
  /// * `texture` class: texture
  /// * `layered` group: Boolean
  /// * `access` group: BufferAccessARB
  /// * `format` group: InternalFormat
  unsafe fn BindImageTexture(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: BufferAccessARB, format: InternalFormat);
  /// glBindImageTextures
  /// * `textures` class: texture
  /// * `textures` len: count
  unsafe fn BindImageTextures(first: GLuint, count: GLsizei, textures: *const GLuint);
  /// glBindProgramPipeline
  /// * `pipeline` class: program pipeline
  unsafe fn BindProgramPipeline(pipeline: GLuint);
  /// glBindRenderbuffer
  /// * `target` group: RenderbufferTarget
  /// * `renderbuffer` class: renderbuffer
  unsafe fn BindRenderbuffer(target: RenderbufferTarget, renderbuffer: GLuint);
  /// glBindSampler
  /// * `sampler` class: sampler
  unsafe fn BindSampler(unit: GLuint, sampler: GLuint);
  /// glBindSamplers
  /// * `samplers` class: sampler
  /// * `samplers` len: count
  unsafe fn BindSamplers(first: GLuint, count: GLsizei, samplers: *const GLuint);
  /// glBindTexture
  /// * `target` group: TextureTarget
  /// * `texture` group: Texture
  /// * `texture` class: texture
  unsafe fn BindTexture(target: TextureTarget, texture: GLuint);
  /// glBindTextureUnit
  /// * `texture` class: texture
  unsafe fn BindTextureUnit(unit: GLuint, texture: GLuint);
  /// glBindTextures
  /// * `textures` class: texture
  /// * `textures` len: count
  unsafe fn BindTextures(first: GLuint, count: GLsizei, textures: *const GLuint);
  /// glBindTransformFeedback
  /// * `target` group: BindTransformFeedbackTarget
  /// * `id` class: transform feedback
  unsafe fn BindTransformFeedback(target: BindTransformFeedbackTarget, id: GLuint);
  /// [glBindVertexArray](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glBindVertexArray.xhtml)
  ///
  /// Binds a given vertex array object as the active vertex array object.
  /// Passing 0 will make it so that no vertex array object is bound.
  ///
  /// * `array` names the vertex array object to bind.
  fn BindVertexArray(array: GLuint);
  /// glBindVertexBuffer
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  unsafe fn BindVertexBuffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
  /// glBindVertexBuffers
  /// * `buffers` class: buffer
  /// * `buffers` len: count
  /// * `offsets` len: count
  /// * `strides` len: count
  unsafe fn BindVertexBuffers(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
  /// glBlendColor
  /// * `red` group: ColorF
  /// * `green` group: ColorF
  /// * `blue` group: ColorF
  /// * `alpha` group: ColorF
  unsafe fn BlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
  /// glBlendEquation
  /// * `mode` group: BlendEquationModeEXT
  unsafe fn BlendEquation(mode: BlendEquationModeEXT);
  /// glBlendEquationSeparate
  /// * `modeRGB` group: BlendEquationModeEXT
  /// * `modeAlpha` group: BlendEquationModeEXT
  unsafe fn BlendEquationSeparate(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);
  /// glBlendEquationSeparatei
  /// * `modeRGB` group: BlendEquationModeEXT
  /// * `modeAlpha` group: BlendEquationModeEXT
  unsafe fn BlendEquationSeparatei(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);
  /// glBlendEquationi
  /// * `mode` group: BlendEquationModeEXT
  unsafe fn BlendEquationi(buf: GLuint, mode: BlendEquationModeEXT);
  /// glBlendFunc
  /// * `sfactor` group: BlendingFactor
  /// * `dfactor` group: BlendingFactor
  unsafe fn BlendFunc(sfactor: BlendingFactor, dfactor: BlendingFactor);
  /// glBlendFuncSeparate
  /// * `sfactorRGB` group: BlendingFactor
  /// * `dfactorRGB` group: BlendingFactor
  /// * `sfactorAlpha` group: BlendingFactor
  /// * `dfactorAlpha` group: BlendingFactor
  unsafe fn BlendFuncSeparate(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);
  /// glBlendFuncSeparatei
  /// * `srcRGB` group: BlendingFactor
  /// * `dstRGB` group: BlendingFactor
  /// * `srcAlpha` group: BlendingFactor
  /// * `dstAlpha` group: BlendingFactor
  unsafe fn BlendFuncSeparatei(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);
  /// glBlendFunci
  /// * `src` group: BlendingFactor
  /// * `dst` group: BlendingFactor
  unsafe fn BlendFunci(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);
  /// glBlitFramebuffer
  /// * `mask` group: ClearBufferMask
  /// * `filter` group: BlitFramebufferFilter
  unsafe fn BlitFramebuffer(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);
  /// glBlitNamedFramebuffer
  /// * `readFramebuffer` class: framebuffer
  /// * `drawFramebuffer` class: framebuffer
  /// * `mask` group: ClearBufferMask
  /// * `filter` group: BlitFramebufferFilter
  unsafe fn BlitNamedFramebuffer(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);
  /// glBufferData
  /// * `target` group: BufferTargetARB
  /// * `size` group: BufferSize
  /// * `data` len: size
  /// * `usage` group: BufferUsageARB
  unsafe fn BufferData(target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB);
  /// glBufferStorage
  /// * `target` group: BufferStorageTarget
  /// * `data` len: size
  /// * `flags` group: BufferStorageMask
  unsafe fn BufferStorage(target: BufferStorageTarget, size: GLsizeiptr, data: *const void, flags: GLbitfield);
  /// glBufferSubData
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  /// * `data` len: size
  unsafe fn BufferSubData(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void);
  /// glCheckFramebufferStatus
  /// * `target` group: FramebufferTarget
  unsafe fn CheckFramebufferStatus(target: FramebufferTarget) -> FramebufferStatus;
  /// glCheckNamedFramebufferStatus
  /// * `framebuffer` class: framebuffer
  /// * `target` group: FramebufferTarget
  unsafe fn CheckNamedFramebufferStatus(framebuffer: GLuint, target: FramebufferTarget) -> FramebufferStatus;
  /// glClampColor
  /// * `target` group: ClampColorTargetARB
  /// * `clamp` group: ClampColorModeARB
  unsafe fn ClampColor(target: ClampColorTargetARB, clamp: ClampColorModeARB);
  /// glClear
  /// * `mask` group: ClearBufferMask
  unsafe fn Clear(mask: GLbitfield);
  /// glClearBufferData
  /// * `target` group: BufferStorageTarget
  /// * `internalformat` group: InternalFormat
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: COMPSIZE(format,type)
  unsafe fn ClearBufferData(target: BufferStorageTarget, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);
  /// glClearBufferSubData
  /// * `target` group: BufferTargetARB
  /// * `internalformat` group: InternalFormat
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: COMPSIZE(format,type)
  unsafe fn ClearBufferSubData(target: BufferTargetARB, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);
  /// glClearBufferfi
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  unsafe fn ClearBufferfi(buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
  /// [glClearBufferfv](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glClearBuffer.xhtml)
  ///
  /// Clears a specified buffer of the currently bound draw framebuffer object
  /// to a specified value.
  ///
  /// * If `buffer` is `GL_COLOR`, a particular draw buffer `GL_DRAW_BUFFERi` is
  ///   specified by passing `i` as `drawbuffer` (eg: to affect
  ///   `GL_DRAW_BUFFER0` you'd pass 0). In this case, `value` points to a
  ///   four-element vector specifying the R, G, B, and A color to clear that
  ///   draw buffer to. If the value of `GL_DRAW_BUFFERi` is `GL_NONE`, the
  ///   command has no effect. Otherwise, the value of `GL_DRAW_BUFFERi`
  ///   identifies one or more color buffers, each of which is cleared to the
  ///   same value. Clamping and type conversion for fixed-point color buffers
  ///   are performed in the same fashion as for `glClearColor`.
  /// * If `buffer` is `GL_DEPTH`, `drawbuffer` must be zero, and `value` points
  ///   to a single value to clear the depth buffer to. Clamping and type
  ///   conversion for fixed-point depth buffers are performed in the same
  ///   fashion as for `glClearDepth`.
  ///
  /// ## Errors
  /// * `GL_INVALID_ENUM` is generated if `buffer` is not `GL_COLOR` or
  ///   `GL_DEPTH`.
  unsafe fn ClearBufferfv(buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);
  /// glClearBufferiv
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  /// * `value` len: COMPSIZE(buffer)
  unsafe fn ClearBufferiv(buffer: Buffer, drawbuffer: GLint, value: *const GLint);
  /// glClearBufferuiv
  /// * `buffer` group: Buffer
  /// * `drawbuffer` group: DrawBufferName
  /// * `value` len: COMPSIZE(buffer)
  unsafe fn ClearBufferuiv(buffer: Buffer, drawbuffer: GLint, value: *const GLuint);
  /// glClearColor
  /// * `red` group: ColorF
  /// * `green` group: ColorF
  /// * `blue` group: ColorF
  /// * `alpha` group: ColorF
  unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
  /// glClearDepth
  unsafe fn ClearDepth(depth: GLdouble);
  /// glClearDepthf
  unsafe fn ClearDepthf(d: GLfloat);
  /// glClearNamedBufferData
  /// * `buffer` class: buffer
  /// * `internalformat` group: InternalFormat
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  unsafe fn ClearNamedBufferData(buffer: GLuint, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);
  /// glClearNamedBufferSubData
  /// * `buffer` class: buffer
  /// * `internalformat` group: InternalFormat
  /// * `size` group: BufferSize
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  unsafe fn ClearNamedBufferSubData(buffer: GLuint, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);
  /// glClearNamedFramebufferfi
  /// * `framebuffer` class: framebuffer
  /// * `buffer` group: Buffer
  unsafe fn ClearNamedFramebufferfi(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
  /// glClearNamedFramebufferfv
  /// * `framebuffer` class: framebuffer
  /// * `buffer` group: Buffer
  unsafe fn ClearNamedFramebufferfv(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);
  /// glClearNamedFramebufferiv
  /// * `framebuffer` class: framebuffer
  /// * `buffer` group: Buffer
  unsafe fn ClearNamedFramebufferiv(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLint);
  /// glClearNamedFramebufferuiv
  /// * `framebuffer` class: framebuffer
  /// * `buffer` group: Buffer
  unsafe fn ClearNamedFramebufferuiv(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLuint);
  /// glClearStencil
  /// * `s` group: StencilValue
  unsafe fn ClearStencil(s: GLint);
  /// glClearTexImage
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: COMPSIZE(format,type)
  unsafe fn ClearTexImage(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, data: *const void);
  /// glClearTexSubImage
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: COMPSIZE(format,type)
  unsafe fn ClearTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);
  /// glClientWaitSync
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `flags` group: SyncObjectMask
  unsafe fn ClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus;
  /// glClipControl
  /// * `origin` group: ClipControlOrigin
  /// * `depth` group: ClipControlDepth
  unsafe fn ClipControl(origin: ClipControlOrigin, depth: ClipControlDepth);
  /// glColorMask
  /// * `red` group: Boolean
  /// * `green` group: Boolean
  /// * `blue` group: Boolean
  /// * `alpha` group: Boolean
  unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
  /// glColorMaski
  /// * `r` group: Boolean
  /// * `g` group: Boolean
  /// * `b` group: Boolean
  /// * `a` group: Boolean
  unsafe fn ColorMaski(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);
  /// [glCompileShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCompileShader.xhtml)
  ///
  /// Compiles the source code assigned to the shader. The compilation status is
  /// stored as part of the shader object's state, check it with `glGetShader`
  /// and `glGetShaderInfoLog`.
  ///
  /// * `shader` names the shader to compile.
  fn CompileShader(shader: GLuint);
  /// glCompressedTexImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  unsafe fn CompressedTexImage1D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);
  /// glCompressedTexImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  unsafe fn CompressedTexImage2D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);
  /// glCompressedTexImage3D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  unsafe fn CompressedTexImage3D(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);
  /// glCompressedTexSubImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  unsafe fn CompressedTexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);
  /// glCompressedTexSubImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  unsafe fn CompressedTexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);
  /// glCompressedTexSubImage3D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `zoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `data` group: CompressedTextureARB
  /// * `data` len: imageSize
  unsafe fn CompressedTexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);
  /// glCompressedTextureSubImage1D
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  unsafe fn CompressedTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);
  /// glCompressedTextureSubImage2D
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  unsafe fn CompressedTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);
  /// glCompressedTextureSubImage3D
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  unsafe fn CompressedTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);
  /// glCopyBufferSubData
  /// * `readTarget` group: CopyBufferSubDataTarget
  /// * `writeTarget` group: CopyBufferSubDataTarget
  /// * `readOffset` group: BufferOffset
  /// * `writeOffset` group: BufferOffset
  /// * `size` group: BufferSize
  unsafe fn CopyBufferSubData(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
  /// glCopyImageSubData
  /// * `srcTarget` group: CopyImageSubDataTarget
  /// * `dstTarget` group: CopyImageSubDataTarget
  unsafe fn CopyImageSubData(srcName: GLuint, srcTarget: CopyImageSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyImageSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);
  /// glCopyNamedBufferSubData
  /// * `readBuffer` class: buffer
  /// * `writeBuffer` class: buffer
  /// * `size` group: BufferSize
  unsafe fn CopyNamedBufferSubData(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);
  /// glCopyTexImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `border` group: CheckedInt32
  unsafe fn CopyTexImage1D(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);
  /// glCopyTexImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `border` group: CheckedInt32
  unsafe fn CopyTexImage2D(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);
  /// glCopyTexSubImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  unsafe fn CopyTexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
  /// glCopyTexSubImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  unsafe fn CopyTexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  /// glCopyTexSubImage3D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `zoffset` group: CheckedInt32
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  unsafe fn CopyTexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  /// glCopyTextureSubImage1D
  /// * `texture` class: texture
  unsafe fn CopyTextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);
  /// glCopyTextureSubImage2D
  /// * `texture` class: texture
  unsafe fn CopyTextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  /// glCopyTextureSubImage3D
  /// * `texture` class: texture
  unsafe fn CopyTextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  /// glCreateBuffers
  /// * `buffers` class: buffer
  /// * `buffers` len: n
  unsafe fn CreateBuffers(n: GLsizei, buffers: *mut GLuint);
  /// glCreateFramebuffers
  /// * `framebuffers` class: framebuffer
  /// * `framebuffers` len: n
  unsafe fn CreateFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
  /// [glCreateProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateProgram.xhtml)
  ///
  /// Creates an empty program object, returning its name (a non-zero ID value).
  ///
  /// ## Failure
  /// * If this fails, 0 is returned.
  fn CreateProgram() -> GLuint;
  /// glCreateProgramPipelines
  /// * `pipelines` class: program pipeline
  /// * `pipelines` len: n
  unsafe fn CreateProgramPipelines(n: GLsizei, pipelines: *mut GLuint);
  /// glCreateQueries
  /// * `target` group: QueryTarget
  /// * `ids` class: query
  /// * `ids` len: n
  unsafe fn CreateQueries(target: QueryTarget, n: GLsizei, ids: *mut GLuint);
  /// glCreateRenderbuffers
  /// * `renderbuffers` class: renderbuffer
  /// * `renderbuffers` len: n
  unsafe fn CreateRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
  /// glCreateSamplers
  /// * `samplers` class: sampler
  /// * `samplers` len: n
  unsafe fn CreateSamplers(n: GLsizei, samplers: *mut GLuint);
  /// [glCreateShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateShader.xhtml)
  ///
  /// Creates a new empty shader object of the given type, returning its name (a
  /// non-zero ID value).
  ///
  /// * `type` group: ShaderType
  ///
  /// ## Failure
  /// * If an error occurs the function returns 0.
  fn CreateShader(type_: ShaderType) -> GLuint;
  /// glCreateShaderProgramv
  /// * `type` group: ShaderType
  /// * `strings` len: count
  unsafe fn CreateShaderProgramv(type_: ShaderType, count: GLsizei, strings: *const *const GLchar) -> GLuint;
  /// glCreateTextures
  /// * `target` group: TextureTarget
  /// * `textures` class: texture
  /// * `textures` len: n
  unsafe fn CreateTextures(target: TextureTarget, n: GLsizei, textures: *mut GLuint);
  /// glCreateTransformFeedbacks
  /// * `ids` class: transform feedback
  /// * `ids` len: n
  unsafe fn CreateTransformFeedbacks(n: GLsizei, ids: *mut GLuint);
  /// [glCreateVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glCreateVertexArrays.xhtml)
  ///
  /// Fills a buffer with new vertex array object names (non-zero ID values).
  ///
  /// * `n` the size of the buffer.
  /// * `arrays` pointer to the start of the buffer.
  unsafe fn CreateVertexArrays(n: GLsizei, arrays: *mut GLuint);
  /// glCullFace
  /// * `mode` group: CullFaceMode
  unsafe fn CullFace(mode: CullFaceMode);
  /// glDebugMessageCallback
  unsafe fn DebugMessageCallback(callback: GLDEBUGPROC, userParam: *const void);
  /// glDebugMessageControl
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `ids` len: count
  /// * `enabled` group: Boolean
  unsafe fn DebugMessageControl(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);
  /// glDebugMessageInsert
  /// * `source` group: DebugSource
  /// * `type` group: DebugType
  /// * `severity` group: DebugSeverity
  /// * `buf` len: COMPSIZE(buf,length)
  unsafe fn DebugMessageInsert(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);
  /// glDeleteBuffers
  /// * `buffers` class: buffer
  /// * `buffers` len: n
  unsafe fn DeleteBuffers(n: GLsizei, buffers: *const GLuint);
  /// glDeleteFramebuffers
  /// * `framebuffers` class: framebuffer
  /// * `framebuffers` len: n
  unsafe fn DeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
  /// [glDeleteProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteProgram.xhtml)
  ///
  /// Marks a program object for deletion. If the shader program is not in use
  /// it will be immediately deleted, otherwise it will be deleted once it's
  /// no longer in use. When a program object is deleted any shaders attached
  /// to it are automatically unattached from it.
  ///
  /// * `program` names the program to mark for deletion.
  fn DeleteProgram(program: GLuint);
  /// glDeleteProgramPipelines
  /// * `pipelines` class: program pipeline
  /// * `pipelines` len: n
  unsafe fn DeleteProgramPipelines(n: GLsizei, pipelines: *const GLuint);
  /// glDeleteQueries
  /// * `ids` class: query
  /// * `ids` len: n
  unsafe fn DeleteQueries(n: GLsizei, ids: *const GLuint);
  /// glDeleteRenderbuffers
  /// * `renderbuffers` class: renderbuffer
  /// * `renderbuffers` len: n
  unsafe fn DeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);
  /// glDeleteSamplers
  /// * `samplers` class: sampler
  /// * `samplers` len: count
  unsafe fn DeleteSamplers(count: GLsizei, samplers: *const GLuint);
  /// [glDeleteShader](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteShader.xhtml)
  ///
  /// Marks a shader to be deleted. If it's not attached to a program it will be
  /// deleted immediately, otherwise it won't be deleted until it's unattached.
  ///
  /// * `shader` names the shader to mark for deletion.
  fn DeleteShader(shader: GLuint);
  /// glDeleteSync
  /// * `sync` group: sync
  /// * `sync` class: sync
  unsafe fn DeleteSync(sync: GLsync);
  /// glDeleteTextures
  /// * `textures` group: Texture
  /// * `textures` class: texture
  /// * `textures` len: n
  unsafe fn DeleteTextures(n: GLsizei, textures: *const GLuint);
  /// glDeleteTransformFeedbacks
  /// * `ids` class: transform feedback
  /// * `ids` len: n
  unsafe fn DeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint);
  /// [glDeleteVertexArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDeleteVertexArrays.xhtml)
  ///
  /// Deletes a list of vertex array objects. If a vertex array object that is
  /// bound is deleted then the binding reverts to 0 and the default vertex
  /// array becomes current. Passing any vertex array object IDs not currently
  /// in use, or passing 0, is silently ignored.
  ///
  /// * `n` the size of the list
  /// * `arrays` the vertex array objects to delete.
  unsafe fn DeleteVertexArrays(n: GLsizei, arrays: *const GLuint);
  /// glDepthFunc
  /// * `func` group: DepthFunction
  unsafe fn DepthFunc(func: DepthFunction);
  /// glDepthMask
  /// * `flag` group: Boolean
  unsafe fn DepthMask(flag: GLboolean);
  /// glDepthRange
  unsafe fn DepthRange(n: GLdouble, f: GLdouble);
  /// glDepthRangeArrayv
  /// * `v` len: COMPSIZE(count)
  unsafe fn DepthRangeArrayv(first: GLuint, count: GLsizei, v: *const GLdouble);
  /// glDepthRangeIndexed
  unsafe fn DepthRangeIndexed(index: GLuint, n: GLdouble, f: GLdouble);
  /// glDepthRangef
  unsafe fn DepthRangef(n: GLfloat, f: GLfloat);
  /// glDetachShader
  /// * `program` class: program
  /// * `shader` class: shader
  unsafe fn DetachShader(program: GLuint, shader: GLuint);
  /// glDisable
  /// * `cap` group: EnableCap
  unsafe fn Disable(cap: EnableCap);
  /// glDisableVertexArrayAttrib
  /// * `vaobj` class: vertex array
  unsafe fn DisableVertexArrayAttrib(vaobj: GLuint, index: GLuint);
  /// glDisableVertexAttribArray
  unsafe fn DisableVertexAttribArray(index: GLuint);
  /// glDisablei
  /// * `target` group: EnableCap
  unsafe fn Disablei(target: EnableCap, index: GLuint);
  /// glDispatchCompute
  unsafe fn DispatchCompute(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);
  /// glDispatchComputeIndirect
  /// * `indirect` group: BufferOffset
  unsafe fn DispatchComputeIndirect(indirect: GLintptr);
  /// [glDrawArrays](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glDrawArrays.xhtml)
  ///
  /// Draws `count` sequential indices using each enabled array to construct a
  /// sequence of primitives, starting from `first`. Depending on the `mode`
  /// specified, the number of indices required per primitive can vary.
  ///
  /// If vertex attributes are modified by `glDrawArrays` the values are
  /// unspecified after the call returns. Any other attributes remain well
  /// defined.
  ///
  /// * `mode` is the type of primitive to render.
  /// * `first` is the starting index to use within the enabled arrays.
  /// * `count` is the number of **indices** to be rendered.
  unsafe fn DrawArrays(mode: PrimitiveType, first: GLint, count: GLsizei);
  /// glDrawArraysIndirect
  /// * `mode` group: PrimitiveType
  unsafe fn DrawArraysIndirect(mode: PrimitiveType, indirect: *const void);
  /// glDrawArraysInstanced
  /// * `mode` group: PrimitiveType
  unsafe fn DrawArraysInstanced(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei);
  /// glDrawArraysInstancedBaseInstance
  /// * `mode` group: PrimitiveType
  unsafe fn DrawArraysInstancedBaseInstance(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);
  /// glDrawBuffer
  /// * `buf` group: DrawBufferMode
  unsafe fn DrawBuffer(buf: DrawBufferMode);
  /// glDrawBuffers
  /// * `bufs` group: DrawBufferMode
  /// * `bufs` len: n
  unsafe fn DrawBuffers(n: GLsizei, bufs: *const DrawBufferMode);
  /// glDrawElements
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  unsafe fn DrawElements(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void);
  /// glDrawElementsBaseVertex
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  unsafe fn DrawElementsBaseVertex(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);
  /// glDrawElementsIndirect
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  unsafe fn DrawElementsIndirect(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void);
  /// glDrawElementsInstanced
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  unsafe fn DrawElementsInstanced(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei);
  /// glDrawElementsInstancedBaseInstance
  /// * `mode` group: PrimitiveType
  /// * `type` group: PrimitiveType
  /// * `indices` len: count
  unsafe fn DrawElementsInstancedBaseInstance(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, instancecount: GLsizei, baseinstance: GLuint);
  /// glDrawElementsInstancedBaseVertex
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  unsafe fn DrawElementsInstancedBaseVertex(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);
  /// glDrawElementsInstancedBaseVertexBaseInstance
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: count
  unsafe fn DrawElementsInstancedBaseVertexBaseInstance(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);
  /// glDrawRangeElements
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  unsafe fn DrawRangeElements(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void);
  /// glDrawRangeElementsBaseVertex
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(count,type)
  unsafe fn DrawRangeElementsBaseVertex(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);
  /// glDrawTransformFeedback
  /// * `mode` group: PrimitiveType
  /// * `id` class: transform feedback
  unsafe fn DrawTransformFeedback(mode: PrimitiveType, id: GLuint);
  /// glDrawTransformFeedbackInstanced
  /// * `mode` group: PrimitiveType
  /// * `id` class: transform feedback
  unsafe fn DrawTransformFeedbackInstanced(mode: PrimitiveType, id: GLuint, instancecount: GLsizei);
  /// glDrawTransformFeedbackStream
  /// * `mode` group: PrimitiveType
  /// * `id` class: transform feedback
  unsafe fn DrawTransformFeedbackStream(mode: PrimitiveType, id: GLuint, stream: GLuint);
  /// glDrawTransformFeedbackStreamInstanced
  /// * `mode` group: PrimitiveType
  /// * `id` class: transform feedback
  unsafe fn DrawTransformFeedbackStreamInstanced(mode: PrimitiveType, id: GLuint, stream: GLuint, instancecount: GLsizei);
  /// glEnable
  /// * `cap` group: EnableCap
  unsafe fn Enable(cap: EnableCap);
  /// glEnableVertexArrayAttrib
  /// * `vaobj` class: vertex array
  unsafe fn EnableVertexArrayAttrib(vaobj: GLuint, index: GLuint);
  /// glEnableVertexAttribArray
  unsafe fn EnableVertexAttribArray(index: GLuint);
  /// glEnablei
  /// * `target` group: EnableCap
  unsafe fn Enablei(target: EnableCap, index: GLuint);
  /// glEndConditionalRender
  unsafe fn EndConditionalRender();
  /// glEndQuery
  /// * `target` group: QueryTarget
  unsafe fn EndQuery(target: QueryTarget);
  /// glEndQueryIndexed
  /// * `target` group: QueryTarget
  unsafe fn EndQueryIndexed(target: QueryTarget, index: GLuint);
  /// glEndTransformFeedback
  unsafe fn EndTransformFeedback();
  /// glFenceSync
  /// * `condition` group: SyncCondition
  /// * `flags` group: SyncBehaviorFlags
  unsafe fn FenceSync(condition: SyncCondition, flags: GLbitfield) -> GLsync;
  /// glFinish
  unsafe fn Finish();
  /// glFlush
  unsafe fn Flush();
  /// glFlushMappedBufferRange
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `length` group: BufferSize
  unsafe fn FlushMappedBufferRange(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr);
  /// glFlushMappedNamedBufferRange
  /// * `buffer` class: buffer
  /// * `length` group: BufferSize
  unsafe fn FlushMappedNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
  /// glFramebufferParameteri
  /// * `target` group: FramebufferTarget
  /// * `pname` group: FramebufferParameterName
  unsafe fn FramebufferParameteri(target: FramebufferTarget, pname: FramebufferParameterName, param: GLint);
  /// glFramebufferRenderbuffer
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `renderbuffertarget` group: RenderbufferTarget
  /// * `renderbuffer` class: renderbuffer
  unsafe fn FramebufferRenderbuffer(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);
  /// glFramebufferTexture
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` class: texture
  unsafe fn FramebufferTexture(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);
  /// glFramebufferTexture1D
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  unsafe fn FramebufferTexture1D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);
  /// glFramebufferTexture2D
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  unsafe fn FramebufferTexture2D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);
  /// glFramebufferTexture3D
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `textarget` group: TextureTarget
  /// * `texture` class: texture
  unsafe fn FramebufferTexture3D(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);
  /// glFramebufferTextureLayer
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` group: Texture
  /// * `texture` class: texture
  /// * `level` group: CheckedInt32
  /// * `layer` group: CheckedInt32
  unsafe fn FramebufferTextureLayer(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);
  /// glFrontFace
  /// * `mode` group: FrontFaceDirection
  unsafe fn FrontFace(mode: FrontFaceDirection);
  /// glGenBuffers
  /// * `buffers` class: buffer
  /// * `buffers` len: n
  unsafe fn GenBuffers(n: GLsizei, buffers: *mut GLuint);
  /// glGenFramebuffers
  /// * `framebuffers` class: framebuffer
  /// * `framebuffers` len: n
  unsafe fn GenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
  /// glGenProgramPipelines
  /// * `pipelines` class: program pipeline
  /// * `pipelines` len: n
  unsafe fn GenProgramPipelines(n: GLsizei, pipelines: *mut GLuint);
  /// glGenQueries
  /// * `ids` class: query
  /// * `ids` len: n
  unsafe fn GenQueries(n: GLsizei, ids: *mut GLuint);
  /// glGenRenderbuffers
  /// * `renderbuffers` class: renderbuffer
  /// * `renderbuffers` len: n
  unsafe fn GenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
  /// glGenSamplers
  /// * `samplers` class: sampler
  /// * `samplers` len: count
  unsafe fn GenSamplers(count: GLsizei, samplers: *mut GLuint);
  /// glGenTextures
  /// * `textures` group: Texture
  /// * `textures` class: texture
  /// * `textures` len: n
  unsafe fn GenTextures(n: GLsizei, textures: *mut GLuint);
  /// glGenTransformFeedbacks
  /// * `ids` class: transform feedback
  /// * `ids` len: n
  unsafe fn GenTransformFeedbacks(n: GLsizei, ids: *mut GLuint);
  /// glGenVertexArrays
  /// * `arrays` class: vertex array
  /// * `arrays` len: n
  unsafe fn GenVertexArrays(n: GLsizei, arrays: *mut GLuint);
  /// glGenerateMipmap
  /// * `target` group: TextureTarget
  unsafe fn GenerateMipmap(target: TextureTarget);
  /// glGenerateTextureMipmap
  /// * `texture` class: texture
  unsafe fn GenerateTextureMipmap(texture: GLuint);
  /// glGetActiveAtomicCounterBufferiv
  /// * `program` class: program
  /// * `pname` group: AtomicCounterBufferPName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetActiveAtomicCounterBufferiv(program: GLuint, bufferIndex: GLuint, pname: AtomicCounterBufferPName, params: *mut GLint);
  /// glGetActiveAttrib
  /// * `program` class: program
  /// * `type` group: AttributeType
  /// * `name` len: bufSize
  unsafe fn GetActiveAttrib(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar);
  /// glGetActiveSubroutineName
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  /// * `name` len: bufSize
  unsafe fn GetActiveSubroutineName(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
  /// glGetActiveSubroutineUniformName
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  /// * `name` len: bufSize
  unsafe fn GetActiveSubroutineUniformName(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
  /// glGetActiveSubroutineUniformiv
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  /// * `pname` group: SubroutineParameterName
  /// * `values` len: COMPSIZE(pname)
  unsafe fn GetActiveSubroutineUniformiv(program: GLuint, shadertype: ShaderType, index: GLuint, pname: SubroutineParameterName, values: *mut GLint);
  /// glGetActiveUniform
  /// * `program` class: program
  /// * `type` group: UniformType
  /// * `name` len: bufSize
  unsafe fn GetActiveUniform(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar);
  /// glGetActiveUniformBlockName
  /// * `program` class: program
  /// * `uniformBlockName` len: bufSize
  unsafe fn GetActiveUniformBlockName(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);
  /// glGetActiveUniformBlockiv
  /// * `program` class: program
  /// * `pname` group: UniformBlockPName
  /// * `params` len: COMPSIZE(program,uniformBlockIndex,pname)
  unsafe fn GetActiveUniformBlockiv(program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint);
  /// glGetActiveUniformName
  /// * `program` class: program
  /// * `uniformName` len: bufSize
  unsafe fn GetActiveUniformName(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);
  /// glGetActiveUniformsiv
  /// * `program` class: program
  /// * `uniformIndices` len: uniformCount
  /// * `pname` group: UniformPName
  /// * `params` len: COMPSIZE(uniformCount,pname)
  unsafe fn GetActiveUniformsiv(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint);
  /// glGetAttachedShaders
  /// * `program` class: program
  /// * `shaders` class: shader
  /// * `shaders` len: maxCount
  unsafe fn GetAttachedShaders(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);
  /// glGetAttribLocation
  /// * `program` class: program
  unsafe fn GetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
  /// glGetBooleani_v
  /// * `target` group: BufferTargetARB
  /// * `data` group: Boolean
  /// * `data` len: COMPSIZE(target)
  unsafe fn GetBooleani_v(target: BufferTargetARB, index: GLuint, data: *mut GLboolean);
  /// glGetBooleanv
  /// * `pname` group: GetPName
  /// * `data` group: Boolean
  /// * `data` len: COMPSIZE(pname)
  unsafe fn GetBooleanv(pname: GetPName, data: *mut GLboolean);
  /// glGetBufferParameteri64v
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPNameARB
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetBufferParameteri64v(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64);
  /// glGetBufferParameteriv
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPNameARB
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetBufferParameteriv(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint);
  /// glGetBufferPointerv
  /// * `target` group: BufferTargetARB
  /// * `pname` group: BufferPointerNameARB
  unsafe fn GetBufferPointerv(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);
  /// glGetBufferSubData
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  /// * `data` len: size
  unsafe fn GetBufferSubData(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void);
  /// glGetCompressedTexImage
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `img` group: CompressedTextureARB
  /// * `img` len: COMPSIZE(target,level)
  unsafe fn GetCompressedTexImage(target: TextureTarget, level: GLint, img: *mut void);
  /// glGetCompressedTextureImage
  /// * `texture` class: texture
  unsafe fn GetCompressedTextureImage(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut void);
  /// glGetCompressedTextureSubImage
  /// * `texture` class: texture
  unsafe fn GetCompressedTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut void);
  /// glGetDebugMessageLog
  /// * `sources` group: DebugSource
  /// * `sources` len: count
  /// * `types` group: DebugType
  /// * `types` len: count
  /// * `ids` len: count
  /// * `severities` group: DebugSeverity
  /// * `severities` len: count
  /// * `lengths` len: count
  /// * `messageLog` len: bufSize
  unsafe fn GetDebugMessageLog(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;
  /// glGetDoublei_v
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  unsafe fn GetDoublei_v(target: GetPName, index: GLuint, data: *mut GLdouble);
  /// glGetDoublev
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  unsafe fn GetDoublev(pname: GetPName, data: *mut GLdouble);
  /// glGetError
  unsafe fn GetError() -> ErrorCode;
  /// glGetFloati_v
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  unsafe fn GetFloati_v(target: GetPName, index: GLuint, data: *mut GLfloat);
  /// glGetFloatv
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  unsafe fn GetFloatv(pname: GetPName, data: *mut GLfloat);
  /// glGetFragDataIndex
  /// * `program` class: program
  unsafe fn GetFragDataIndex(program: GLuint, name: *const GLchar) -> GLint;
  /// glGetFragDataLocation
  /// * `program` class: program
  /// * `name` len: COMPSIZE(name)
  unsafe fn GetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint;
  /// glGetFramebufferAttachmentParameteriv
  /// * `target` group: FramebufferTarget
  /// * `attachment` group: FramebufferAttachment
  /// * `pname` group: FramebufferAttachmentParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetFramebufferAttachmentParameteriv(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);
  /// glGetFramebufferParameteriv
  /// * `target` group: FramebufferTarget
  /// * `pname` group: FramebufferAttachmentParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetFramebufferParameteriv(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut GLint);
  /// glGetGraphicsResetStatus
  unsafe fn GetGraphicsResetStatus() -> GraphicsResetStatus;
  /// glGetInteger64i_v
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  unsafe fn GetInteger64i_v(target: GetPName, index: GLuint, data: *mut GLint64);
  /// glGetInteger64v
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  unsafe fn GetInteger64v(pname: GetPName, data: *mut GLint64);
  /// glGetIntegeri_v
  /// * `target` group: GetPName
  /// * `data` len: COMPSIZE(target)
  unsafe fn GetIntegeri_v(target: GetPName, index: GLuint, data: *mut GLint);
  /// glGetIntegerv
  /// * `pname` group: GetPName
  /// * `data` len: COMPSIZE(pname)
  unsafe fn GetIntegerv(pname: GetPName, data: *mut GLint);
  /// glGetInternalformati64v
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `pname` group: InternalFormatPName
  /// * `params` len: count
  unsafe fn GetInternalformati64v(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint64);
  /// glGetInternalformativ
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `pname` group: InternalFormatPName
  /// * `params` len: count
  unsafe fn GetInternalformativ(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint);
  /// glGetMultisamplefv
  /// * `pname` group: GetMultisamplePNameNV
  /// * `val` len: COMPSIZE(pname)
  unsafe fn GetMultisamplefv(pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat);
  /// glGetNamedBufferParameteri64v
  /// * `buffer` class: buffer
  /// * `pname` group: BufferPNameARB
  unsafe fn GetNamedBufferParameteri64v(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint64);
  /// glGetNamedBufferParameteriv
  /// * `buffer` class: buffer
  /// * `pname` group: BufferPNameARB
  unsafe fn GetNamedBufferParameteriv(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint);
  /// glGetNamedBufferPointerv
  /// * `buffer` class: buffer
  /// * `pname` group: BufferPointerNameARB
  unsafe fn GetNamedBufferPointerv(buffer: GLuint, pname: BufferPointerNameARB, params: *mut *mut void);
  /// glGetNamedBufferSubData
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  unsafe fn GetNamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void);
  /// glGetNamedFramebufferAttachmentParameteriv
  /// * `framebuffer` class: framebuffer
  /// * `attachment` group: FramebufferAttachment
  /// * `pname` group: FramebufferAttachmentParameterName
  unsafe fn GetNamedFramebufferAttachmentParameteriv(framebuffer: GLuint, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);
  /// glGetNamedFramebufferParameteriv
  /// * `framebuffer` class: framebuffer
  /// * `pname` group: GetFramebufferParameter
  unsafe fn GetNamedFramebufferParameteriv(framebuffer: GLuint, pname: GetFramebufferParameter, param: *mut GLint);
  /// glGetNamedRenderbufferParameteriv
  /// * `renderbuffer` class: renderbuffer
  /// * `pname` group: RenderbufferParameterName
  unsafe fn GetNamedRenderbufferParameteriv(renderbuffer: GLuint, pname: RenderbufferParameterName, params: *mut GLint);
  /// glGetObjectLabel
  /// * `identifier` group: ObjectIdentifier
  /// * `label` len: bufSize
  unsafe fn GetObjectLabel(identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
  /// glGetObjectPtrLabel
  /// * `label` len: bufSize
  unsafe fn GetObjectPtrLabel(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);
  /// glGetPointerv
  /// * `pname` group: GetPointervPName
  unsafe fn GetPointerv(pname: GetPointervPName, params: *mut *mut void);
  /// glGetProgramBinary
  /// * `program` class: program
  /// * `binary` len: bufSize
  unsafe fn GetProgramBinary(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void);
  /// glGetProgramInfoLog
  /// * `program` class: program
  /// * `infoLog` len: bufSize
  unsafe fn GetProgramInfoLog(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
  /// glGetProgramInterfaceiv
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `pname` group: ProgramInterfacePName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetProgramInterfaceiv(program: GLuint, programInterface: ProgramInterface, pname: ProgramInterfacePName, params: *mut GLint);
  /// glGetProgramPipelineInfoLog
  /// * `pipeline` class: program pipeline
  /// * `infoLog` len: bufSize
  unsafe fn GetProgramPipelineInfoLog(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
  /// glGetProgramPipelineiv
  /// * `pipeline` class: program pipeline
  /// * `pname` group: PipelineParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetProgramPipelineiv(pipeline: GLuint, pname: PipelineParameterName, params: *mut GLint);
  /// glGetProgramResourceIndex
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `name` len: COMPSIZE(name)
  unsafe fn GetProgramResourceIndex(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLuint;
  /// glGetProgramResourceLocation
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `name` len: COMPSIZE(name)
  unsafe fn GetProgramResourceLocation(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;
  /// glGetProgramResourceLocationIndex
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `name` len: COMPSIZE(name)
  unsafe fn GetProgramResourceLocationIndex(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;
  /// glGetProgramResourceName
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `name` len: bufSize
  unsafe fn GetProgramResourceName(program: GLuint, programInterface: ProgramInterface, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);
  /// glGetProgramResourceiv
  /// * `program` class: program
  /// * `programInterface` group: ProgramInterface
  /// * `props` group: ProgramResourceProperty
  /// * `props` len: propCount
  /// * `params` len: count
  unsafe fn GetProgramResourceiv(program: GLuint, programInterface: ProgramInterface, index: GLuint, propCount: GLsizei, props: *const ProgramResourceProperty, count: GLsizei, length: *mut GLsizei, params: *mut GLint);
  /// glGetProgramStageiv
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  /// * `pname` group: ProgramStagePName
  unsafe fn GetProgramStageiv(program: GLuint, shadertype: ShaderType, pname: ProgramStagePName, values: *mut GLint);
  /// glGetProgramiv
  /// * `program` class: program
  /// * `pname` group: ProgramPropertyARB
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetProgramiv(program: GLuint, pname: ProgramPropertyARB, params: *mut GLint);
  /// glGetQueryBufferObjecti64v
  /// * `id` class: query
  /// * `buffer` class: buffer
  /// * `pname` group: QueryObjectParameterName
  unsafe fn GetQueryBufferObjecti64v(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);
  /// glGetQueryBufferObjectiv
  /// * `id` class: query
  /// * `buffer` class: buffer
  /// * `pname` group: QueryObjectParameterName
  unsafe fn GetQueryBufferObjectiv(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);
  /// glGetQueryBufferObjectui64v
  /// * `id` class: query
  /// * `buffer` class: buffer
  /// * `pname` group: QueryObjectParameterName
  unsafe fn GetQueryBufferObjectui64v(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);
  /// glGetQueryBufferObjectuiv
  /// * `id` class: query
  /// * `buffer` class: buffer
  /// * `pname` group: QueryObjectParameterName
  unsafe fn GetQueryBufferObjectuiv(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);
  /// glGetQueryIndexediv
  /// * `target` group: QueryTarget
  /// * `pname` group: QueryParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetQueryIndexediv(target: QueryTarget, index: GLuint, pname: QueryParameterName, params: *mut GLint);
  /// glGetQueryObjecti64v
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetQueryObjecti64v(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64);
  /// glGetQueryObjectiv
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetQueryObjectiv(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);
  /// glGetQueryObjectui64v
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetQueryObjectui64v(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64);
  /// glGetQueryObjectuiv
  /// * `id` class: query
  /// * `pname` group: QueryObjectParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetQueryObjectuiv(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);
  /// glGetQueryiv
  /// * `target` group: QueryTarget
  /// * `pname` group: QueryParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetQueryiv(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);
  /// glGetRenderbufferParameteriv
  /// * `target` group: RenderbufferTarget
  /// * `pname` group: RenderbufferParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetRenderbufferParameteriv(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);
  /// glGetSamplerParameterIiv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetSamplerParameterIiv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);
  /// glGetSamplerParameterIuiv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetSamplerParameterIuiv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);
  /// glGetSamplerParameterfv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetSamplerParameterfv(sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat);
  /// glGetSamplerParameteriv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetSamplerParameteriv(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);
  /// glGetShaderInfoLog
  /// * `shader` class: shader
  /// * `infoLog` len: bufSize
  unsafe fn GetShaderInfoLog(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);
  /// glGetShaderPrecisionFormat
  /// * `shadertype` group: ShaderType
  /// * `precisiontype` group: PrecisionType
  unsafe fn GetShaderPrecisionFormat(shadertype: ShaderType, precisiontype: PrecisionType, range: *mut [GLint; 2], precision: *mut GLint);
  /// glGetShaderSource
  /// * `shader` class: shader
  /// * `source` len: bufSize
  unsafe fn GetShaderSource(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);
  /// glGetShaderiv
  /// * `shader` class: shader
  /// * `pname` group: ShaderParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetShaderiv(shader: GLuint, pname: ShaderParameterName, params: *mut GLint);
  /// glGetString
  /// * `name` group: StringName
  unsafe fn GetString(name: StringName) -> GLubyte;
  /// glGetStringi
  /// * `name` group: StringName
  unsafe fn GetStringi(name: StringName, index: GLuint) -> GLubyte;
  /// glGetSubroutineIndex
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  unsafe fn GetSubroutineIndex(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLuint;
  /// glGetSubroutineUniformLocation
  /// * `program` class: program
  /// * `shadertype` group: ShaderType
  unsafe fn GetSubroutineUniformLocation(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLint;
  /// glGetSynciv
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `pname` group: SyncParameterName
  /// * `values` len: count
  unsafe fn GetSynciv(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint);
  /// glGetTexImage
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(target,level,format,type)
  unsafe fn GetTexImage(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);
  /// glGetTexLevelParameterfv
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetTexLevelParameterfv(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);
  /// glGetTexLevelParameteriv
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetTexLevelParameteriv(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);
  /// glGetTexParameterIiv
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetTexParameterIiv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);
  /// glGetTexParameterIuiv
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetTexParameterIuiv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);
  /// glGetTexParameterfv
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetTexParameterfv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);
  /// glGetTexParameteriv
  /// * `target` group: TextureTarget
  /// * `pname` group: GetTextureParameter
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetTexParameteriv(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);
  /// glGetTextureImage
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  unsafe fn GetTextureImage(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);
  /// glGetTextureLevelParameterfv
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  unsafe fn GetTextureLevelParameterfv(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);
  /// glGetTextureLevelParameteriv
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  unsafe fn GetTextureLevelParameteriv(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLint);
  /// glGetTextureParameterIiv
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  unsafe fn GetTextureParameterIiv(texture: GLuint, pname: GetTextureParameter, params: *mut GLint);
  /// glGetTextureParameterIuiv
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  unsafe fn GetTextureParameterIuiv(texture: GLuint, pname: GetTextureParameter, params: *mut GLuint);
  /// glGetTextureParameterfv
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  unsafe fn GetTextureParameterfv(texture: GLuint, pname: GetTextureParameter, params: *mut GLfloat);
  /// glGetTextureParameteriv
  /// * `texture` class: texture
  /// * `pname` group: GetTextureParameter
  unsafe fn GetTextureParameteriv(texture: GLuint, pname: GetTextureParameter, params: *mut GLint);
  /// glGetTextureSubImage
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  unsafe fn GetTextureSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);
  /// glGetTransformFeedbackVarying
  /// * `program` class: program
  /// * `type` group: AttributeType
  /// * `name` len: bufSize
  unsafe fn GetTransformFeedbackVarying(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar);
  /// glGetTransformFeedbacki64_v
  /// * `xfb` class: transform feedback
  /// * `pname` group: TransformFeedbackPName
  unsafe fn GetTransformFeedbacki64_v(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint64);
  /// glGetTransformFeedbacki_v
  /// * `xfb` class: transform feedback
  /// * `pname` group: TransformFeedbackPName
  unsafe fn GetTransformFeedbacki_v(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint);
  /// glGetTransformFeedbackiv
  /// * `xfb` class: transform feedback
  /// * `pname` group: TransformFeedbackPName
  unsafe fn GetTransformFeedbackiv(xfb: GLuint, pname: TransformFeedbackPName, param: *mut GLint);
  /// glGetUniformBlockIndex
  /// * `program` class: program
  /// * `uniformBlockName` len: COMPSIZE()
  unsafe fn GetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
  /// glGetUniformIndices
  /// * `program` class: program
  /// * `uniformNames` len: COMPSIZE(uniformCount)
  /// * `uniformIndices` len: COMPSIZE(uniformCount)
  unsafe fn GetUniformIndices(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);
  /// glGetUniformLocation
  /// * `program` class: program
  unsafe fn GetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
  /// glGetUniformSubroutineuiv
  /// * `shadertype` group: ShaderType
  unsafe fn GetUniformSubroutineuiv(shadertype: ShaderType, location: GLint, params: *mut GLuint);
  /// glGetUniformdv
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  unsafe fn GetUniformdv(program: GLuint, location: GLint, params: *mut GLdouble);
  /// glGetUniformfv
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  unsafe fn GetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat);
  /// glGetUniformiv
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  unsafe fn GetUniformiv(program: GLuint, location: GLint, params: *mut GLint);
  /// glGetUniformuiv
  /// * `program` class: program
  /// * `params` len: COMPSIZE(program,location)
  unsafe fn GetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint);
  /// glGetVertexArrayIndexed64iv
  /// * `vaobj` class: vertex array
  /// * `pname` group: VertexArrayPName
  unsafe fn GetVertexArrayIndexed64iv(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint64);
  /// glGetVertexArrayIndexediv
  /// * `vaobj` class: vertex array
  /// * `pname` group: VertexArrayPName
  unsafe fn GetVertexArrayIndexediv(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint);
  /// glGetVertexArrayiv
  /// * `vaobj` class: vertex array
  /// * `pname` group: VertexArrayPName
  unsafe fn GetVertexArrayiv(vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint);
  /// glGetVertexAttribIiv
  /// * `pname` group: VertexAttribEnum
  unsafe fn GetVertexAttribIiv(index: GLuint, pname: VertexAttribEnum, params: *mut GLint);
  /// glGetVertexAttribIuiv
  /// * `pname` group: VertexAttribEnum
  unsafe fn GetVertexAttribIuiv(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint);
  /// glGetVertexAttribLdv
  /// * `pname` group: VertexAttribEnum
  /// * `params` len: COMPSIZE(pname)
  unsafe fn GetVertexAttribLdv(index: GLuint, pname: VertexAttribEnum, params: *mut GLdouble);
  /// glGetVertexAttribPointerv
  /// * `pname` group: VertexAttribPointerPropertyARB
  unsafe fn GetVertexAttribPointerv(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void);
  /// glGetVertexAttribdv
  /// * `pname` group: VertexAttribPropertyARB
  unsafe fn GetVertexAttribdv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]);
  /// glGetVertexAttribfv
  /// * `pname` group: VertexAttribPropertyARB
  unsafe fn GetVertexAttribfv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]);
  /// glGetVertexAttribiv
  /// * `pname` group: VertexAttribPropertyARB
  unsafe fn GetVertexAttribiv(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]);
  /// glGetnCompressedTexImage
  /// * `target` group: TextureTarget
  /// * `pixels` len: bufSize
  unsafe fn GetnCompressedTexImage(target: TextureTarget, lod: GLint, bufSize: GLsizei, pixels: *mut void);
  /// glGetnTexImage
  /// * `target` group: TextureTarget
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: bufSize
  unsafe fn GetnTexImage(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);
  /// glGetnUniformdv
  /// * `program` class: program
  /// * `params` len: bufSize / 8
  unsafe fn GetnUniformdv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);
  /// glGetnUniformfv
  /// * `program` class: program
  /// * `params` len: bufSize / 4
  unsafe fn GetnUniformfv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);
  /// glGetnUniformiv
  /// * `program` class: program
  /// * `params` len: bufSize / 4
  unsafe fn GetnUniformiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);
  /// glGetnUniformuiv
  /// * `program` class: program
  /// * `params` len: bufSize / 4
  unsafe fn GetnUniformuiv(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);
  /// glHint
  /// * `target` group: HintTarget
  /// * `mode` group: HintMode
  unsafe fn Hint(target: HintTarget, mode: HintMode);
  /// glInvalidateBufferData
  /// * `buffer` class: buffer
  unsafe fn InvalidateBufferData(buffer: GLuint);
  /// glInvalidateBufferSubData
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  /// * `length` group: BufferSize
  unsafe fn InvalidateBufferSubData(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);
  /// glInvalidateFramebuffer
  /// * `target` group: FramebufferTarget
  /// * `attachments` group: InvalidateFramebufferAttachment
  /// * `attachments` len: numAttachments
  unsafe fn InvalidateFramebuffer(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment);
  /// glInvalidateNamedFramebufferData
  /// * `framebuffer` class: framebuffer
  /// * `attachments` group: FramebufferAttachment
  unsafe fn InvalidateNamedFramebufferData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment);
  /// glInvalidateNamedFramebufferSubData
  /// * `framebuffer` class: framebuffer
  /// * `attachments` group: FramebufferAttachment
  unsafe fn InvalidateNamedFramebufferSubData(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  /// glInvalidateSubFramebuffer
  /// * `target` group: FramebufferTarget
  /// * `attachments` group: InvalidateFramebufferAttachment
  /// * `attachments` len: numAttachments
  unsafe fn InvalidateSubFramebuffer(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  /// glInvalidateTexImage
  /// * `texture` class: texture
  unsafe fn InvalidateTexImage(texture: GLuint, level: GLint);
  /// glInvalidateTexSubImage
  /// * `texture` class: texture
  unsafe fn InvalidateTexSubImage(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);
  /// glIsBuffer
  /// * `buffer` class: buffer
  unsafe fn IsBuffer(buffer: GLuint) -> GLboolean;
  /// glIsEnabled
  /// * `cap` group: EnableCap
  unsafe fn IsEnabled(cap: EnableCap) -> GLboolean;
  /// glIsEnabledi
  /// * `target` group: EnableCap
  unsafe fn IsEnabledi(target: EnableCap, index: GLuint) -> GLboolean;
  /// glIsFramebuffer
  /// * `framebuffer` class: framebuffer
  unsafe fn IsFramebuffer(framebuffer: GLuint) -> GLboolean;
  /// glIsProgram
  /// * `program` class: program
  unsafe fn IsProgram(program: GLuint) -> GLboolean;
  /// glIsProgramPipeline
  /// * `pipeline` class: program pipeline
  unsafe fn IsProgramPipeline(pipeline: GLuint) -> GLboolean;
  /// glIsQuery
  /// * `id` class: query
  unsafe fn IsQuery(id: GLuint) -> GLboolean;
  /// glIsRenderbuffer
  /// * `renderbuffer` class: renderbuffer
  unsafe fn IsRenderbuffer(renderbuffer: GLuint) -> GLboolean;
  /// glIsSampler
  /// * `sampler` class: sampler
  unsafe fn IsSampler(sampler: GLuint) -> GLboolean;
  /// glIsShader
  /// * `shader` class: shader
  unsafe fn IsShader(shader: GLuint) -> GLboolean;
  /// glIsSync
  /// * `sync` group: sync
  /// * `sync` class: sync
  unsafe fn IsSync(sync: GLsync) -> GLboolean;
  /// glIsTexture
  /// * `texture` group: Texture
  /// * `texture` class: texture
  unsafe fn IsTexture(texture: GLuint) -> GLboolean;
  /// glIsTransformFeedback
  /// * `id` class: transform feedback
  unsafe fn IsTransformFeedback(id: GLuint) -> GLboolean;
  /// glIsVertexArray
  /// * `array` class: vertex array
  unsafe fn IsVertexArray(array: GLuint) -> GLboolean;
  /// glLineWidth
  /// * `width` group: CheckedFloat32
  unsafe fn LineWidth(width: GLfloat);
  /// [glLinkProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml)
  ///
  /// Performs linking on a program object. The link status of the program will
  /// be stored in its object state, you can check it with `glGetProgram`
  /// and/or `glGetProgramInfoLog`.
  ///
  /// * `program` the name of the program to link
  fn LinkProgram(program: GLuint);
  /// glLogicOp
  /// * `opcode` group: LogicOp
  unsafe fn LogicOp(opcode: LogicOp);
  /// glMapBuffer
  /// * `target` group: BufferTargetARB
  /// * `access` group: BufferAccessARB
  unsafe fn MapBuffer(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;
  /// glMapBufferRange
  /// * `target` group: BufferTargetARB
  /// * `offset` group: BufferOffset
  /// * `length` group: BufferSize
  /// * `access` group: MapBufferAccessMask
  unsafe fn MapBufferRange(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;
  /// glMapNamedBuffer
  /// * `buffer` class: buffer
  /// * `access` group: BufferAccessARB
  unsafe fn MapNamedBuffer(buffer: GLuint, access: BufferAccessARB) -> *mut void;
  /// glMapNamedBufferRange
  /// * `buffer` class: buffer
  /// * `length` group: BufferSize
  /// * `access` group: MapBufferAccessMask
  unsafe fn MapNamedBufferRange(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;
  /// glMemoryBarrier
  /// * `barriers` group: MemoryBarrierMask
  unsafe fn MemoryBarrier(barriers: GLbitfield);
  /// glMemoryBarrierByRegion
  /// * `barriers` group: MemoryBarrierMask
  unsafe fn MemoryBarrierByRegion(barriers: GLbitfield);
  /// glMinSampleShading
  /// * `value` group: ColorF
  unsafe fn MinSampleShading(value: GLfloat);
  /// glMultiDrawArrays
  /// * `mode` group: PrimitiveType
  /// * `first` len: COMPSIZE(drawcount)
  /// * `count` len: COMPSIZE(drawcount)
  unsafe fn MultiDrawArrays(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);
  /// glMultiDrawArraysIndirect
  /// * `mode` group: PrimitiveType
  /// * `indirect` len: COMPSIZE(drawcount,stride)
  unsafe fn MultiDrawArraysIndirect(mode: PrimitiveType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);
  /// glMultiDrawArraysIndirectCount
  /// * `mode` group: PrimitiveType
  unsafe fn MultiDrawArraysIndirectCount(mode: PrimitiveType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
  /// glMultiDrawElements
  /// * `mode` group: PrimitiveType
  /// * `count` len: COMPSIZE(drawcount)
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(drawcount)
  unsafe fn MultiDrawElements(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei);
  /// glMultiDrawElementsBaseVertex
  /// * `mode` group: PrimitiveType
  /// * `count` len: COMPSIZE(drawcount)
  /// * `type` group: DrawElementsType
  /// * `indices` len: COMPSIZE(drawcount)
  /// * `basevertex` len: COMPSIZE(drawcount)
  unsafe fn MultiDrawElementsBaseVertex(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);
  /// glMultiDrawElementsIndirect
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  /// * `indirect` len: COMPSIZE(drawcount,stride)
  unsafe fn MultiDrawElementsIndirect(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);
  /// glMultiDrawElementsIndirectCount
  /// * `mode` group: PrimitiveType
  /// * `type` group: DrawElementsType
  unsafe fn MultiDrawElementsIndirectCount(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);
  /// glNamedBufferData
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  /// * `usage` group: VertexBufferObjectUsage
  unsafe fn NamedBufferData(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: VertexBufferObjectUsage);
  /// glNamedBufferStorage
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  /// * `data` len: size
  /// * `flags` group: BufferStorageMask
  unsafe fn NamedBufferStorage(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield);
  /// glNamedBufferSubData
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  /// * `data` len: COMPSIZE(size)
  unsafe fn NamedBufferSubData(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);
  /// glNamedFramebufferDrawBuffer
  /// * `framebuffer` class: framebuffer
  /// * `buf` group: ColorBuffer
  unsafe fn NamedFramebufferDrawBuffer(framebuffer: GLuint, buf: ColorBuffer);
  /// glNamedFramebufferDrawBuffers
  /// * `framebuffer` class: framebuffer
  /// * `bufs` group: ColorBuffer
  unsafe fn NamedFramebufferDrawBuffers(framebuffer: GLuint, n: GLsizei, bufs: *const ColorBuffer);
  /// glNamedFramebufferParameteri
  /// * `framebuffer` class: framebuffer
  /// * `pname` group: FramebufferParameterName
  unsafe fn NamedFramebufferParameteri(framebuffer: GLuint, pname: FramebufferParameterName, param: GLint);
  /// glNamedFramebufferReadBuffer
  /// * `framebuffer` class: framebuffer
  /// * `src` group: ColorBuffer
  unsafe fn NamedFramebufferReadBuffer(framebuffer: GLuint, src: ColorBuffer);
  /// glNamedFramebufferRenderbuffer
  /// * `framebuffer` class: framebuffer
  /// * `attachment` group: FramebufferAttachment
  /// * `renderbuffertarget` group: RenderbufferTarget
  /// * `renderbuffer` class: renderbuffer
  unsafe fn NamedFramebufferRenderbuffer(framebuffer: GLuint, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);
  /// glNamedFramebufferTexture
  /// * `framebuffer` class: framebuffer
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` class: texture
  unsafe fn NamedFramebufferTexture(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint);
  /// glNamedFramebufferTextureLayer
  /// * `framebuffer` class: framebuffer
  /// * `attachment` group: FramebufferAttachment
  /// * `texture` class: texture
  unsafe fn NamedFramebufferTextureLayer(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);
  /// glNamedRenderbufferStorage
  /// * `renderbuffer` class: renderbuffer
  /// * `internalformat` group: InternalFormat
  unsafe fn NamedRenderbufferStorage(renderbuffer: GLuint, internalformat: InternalFormat, width: GLsizei, height: GLsizei);
  /// glNamedRenderbufferStorageMultisample
  /// * `renderbuffer` class: renderbuffer
  /// * `internalformat` group: InternalFormat
  unsafe fn NamedRenderbufferStorageMultisample(renderbuffer: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);
  /// glObjectLabel
  /// * `identifier` group: ObjectIdentifier
  /// * `label` len: COMPSIZE(label,length)
  unsafe fn ObjectLabel(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar);
  /// glObjectPtrLabel
  /// * `label` len: COMPSIZE(label,length)
  unsafe fn ObjectPtrLabel(ptr: *const void, length: GLsizei, label: *const GLchar);
  /// glPatchParameterfv
  /// * `pname` group: PatchParameterName
  /// * `values` len: COMPSIZE(pname)
  unsafe fn PatchParameterfv(pname: PatchParameterName, values: *const GLfloat);
  /// glPatchParameteri
  /// * `pname` group: PatchParameterName
  unsafe fn PatchParameteri(pname: PatchParameterName, value: GLint);
  /// glPauseTransformFeedback
  unsafe fn PauseTransformFeedback();
  /// glPixelStoref
  /// * `pname` group: PixelStoreParameter
  /// * `param` group: CheckedFloat32
  unsafe fn PixelStoref(pname: PixelStoreParameter, param: GLfloat);
  /// glPixelStorei
  /// * `pname` group: PixelStoreParameter
  /// * `param` group: CheckedInt32
  unsafe fn PixelStorei(pname: PixelStoreParameter, param: GLint);
  /// glPointParameterf
  /// * `pname` group: PointParameterNameARB
  /// * `param` group: CheckedFloat32
  unsafe fn PointParameterf(pname: PointParameterNameARB, param: GLfloat);
  /// glPointParameterfv
  /// * `pname` group: PointParameterNameARB
  /// * `params` group: CheckedFloat32
  /// * `params` len: COMPSIZE(pname)
  unsafe fn PointParameterfv(pname: PointParameterNameARB, params: *const GLfloat);
  /// glPointParameteri
  /// * `pname` group: PointParameterNameARB
  unsafe fn PointParameteri(pname: PointParameterNameARB, param: GLint);
  /// glPointParameteriv
  /// * `pname` group: PointParameterNameARB
  /// * `params` len: COMPSIZE(pname)
  unsafe fn PointParameteriv(pname: PointParameterNameARB, params: *const GLint);
  /// [glPointSize](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glPointSize.xhtml)
  ///
  /// Sets the diameter of rasterized points if `GL_PROGRAM_POINT_SIZE` is
  /// *disabled*. (Otherwise, this setting is ignored and you must modify
  /// `gl_PointSize` from within a shader to change point size.)
  ///
  /// The default point size is 1.0, and it cannot be set to less than 0.0.
  fn PointSize(size: GLfloat);
  /// glPolygonMode
  /// * `face` group: MaterialFace
  /// * `mode` group: PolygonMode
  unsafe fn PolygonMode(face: MaterialFace, mode: PolygonMode);
  /// glPolygonOffset
  unsafe fn PolygonOffset(factor: GLfloat, units: GLfloat);
  /// glPolygonOffsetClamp
  unsafe fn PolygonOffsetClamp(factor: GLfloat, units: GLfloat, clamp: GLfloat);
  /// glPopDebugGroup
  unsafe fn PopDebugGroup();
  /// glPrimitiveRestartIndex
  unsafe fn PrimitiveRestartIndex(index: GLuint);
  /// glProgramBinary
  /// * `program` class: program
  /// * `binary` len: length
  unsafe fn ProgramBinary(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLsizei);
  /// glProgramParameteri
  /// * `program` class: program
  /// * `pname` group: ProgramParameterPName
  unsafe fn ProgramParameteri(program: GLuint, pname: ProgramParameterPName, value: GLint);
  /// glProgramUniform1d
  /// * `program` class: program
  unsafe fn ProgramUniform1d(program: GLuint, location: GLint, v0: GLdouble);
  /// glProgramUniform1dv
  /// * `program` class: program
  /// * `value` len: count
  unsafe fn ProgramUniform1dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
  /// glProgramUniform1f
  /// * `program` class: program
  unsafe fn ProgramUniform1f(program: GLuint, location: GLint, v0: GLfloat);
  /// glProgramUniform1fv
  /// * `program` class: program
  /// * `value` len: count
  unsafe fn ProgramUniform1fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
  /// glProgramUniform1i
  /// * `program` class: program
  unsafe fn ProgramUniform1i(program: GLuint, location: GLint, v0: GLint);
  /// glProgramUniform1iv
  /// * `program` class: program
  /// * `value` len: count
  unsafe fn ProgramUniform1iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
  /// glProgramUniform1ui
  /// * `program` class: program
  unsafe fn ProgramUniform1ui(program: GLuint, location: GLint, v0: GLuint);
  /// glProgramUniform1uiv
  /// * `program` class: program
  /// * `value` len: count
  unsafe fn ProgramUniform1uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
  /// glProgramUniform2d
  /// * `program` class: program
  unsafe fn ProgramUniform2d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);
  /// glProgramUniform2dv
  /// * `program` class: program
  /// * `value` len: count*2
  unsafe fn ProgramUniform2dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
  /// glProgramUniform2f
  /// * `program` class: program
  unsafe fn ProgramUniform2f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);
  /// glProgramUniform2fv
  /// * `program` class: program
  /// * `value` len: count*2
  unsafe fn ProgramUniform2fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
  /// glProgramUniform2i
  /// * `program` class: program
  unsafe fn ProgramUniform2i(program: GLuint, location: GLint, v0: GLint, v1: GLint);
  /// glProgramUniform2iv
  /// * `program` class: program
  /// * `value` len: count*2
  unsafe fn ProgramUniform2iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
  /// glProgramUniform2ui
  /// * `program` class: program
  unsafe fn ProgramUniform2ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);
  /// glProgramUniform2uiv
  /// * `program` class: program
  /// * `value` len: count*2
  unsafe fn ProgramUniform2uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
  /// glProgramUniform3d
  /// * `program` class: program
  unsafe fn ProgramUniform3d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);
  /// glProgramUniform3dv
  /// * `program` class: program
  /// * `value` len: count*3
  unsafe fn ProgramUniform3dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
  /// glProgramUniform3f
  /// * `program` class: program
  unsafe fn ProgramUniform3f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
  /// glProgramUniform3fv
  /// * `program` class: program
  /// * `value` len: count*3
  unsafe fn ProgramUniform3fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
  /// glProgramUniform3i
  /// * `program` class: program
  unsafe fn ProgramUniform3i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);
  /// glProgramUniform3iv
  /// * `program` class: program
  /// * `value` len: count*3
  unsafe fn ProgramUniform3iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
  /// glProgramUniform3ui
  /// * `program` class: program
  unsafe fn ProgramUniform3ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
  /// glProgramUniform3uiv
  /// * `program` class: program
  /// * `value` len: count*3
  unsafe fn ProgramUniform3uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
  /// glProgramUniform4d
  /// * `program` class: program
  unsafe fn ProgramUniform4d(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);
  /// glProgramUniform4dv
  /// * `program` class: program
  /// * `value` len: count*4
  unsafe fn ProgramUniform4dv(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);
  /// glProgramUniform4f
  /// * `program` class: program
  unsafe fn ProgramUniform4f(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
  /// glProgramUniform4fv
  /// * `program` class: program
  /// * `value` len: count*4
  unsafe fn ProgramUniform4fv(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);
  /// glProgramUniform4i
  /// * `program` class: program
  unsafe fn ProgramUniform4i(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
  /// glProgramUniform4iv
  /// * `program` class: program
  /// * `value` len: count*4
  unsafe fn ProgramUniform4iv(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);
  /// glProgramUniform4ui
  /// * `program` class: program
  unsafe fn ProgramUniform4ui(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
  /// glProgramUniform4uiv
  /// * `program` class: program
  /// * `value` len: count*4
  unsafe fn ProgramUniform4uiv(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);
  /// glProgramUniformMatrix2dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  unsafe fn ProgramUniformMatrix2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix2fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  unsafe fn ProgramUniformMatrix2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProgramUniformMatrix2x3dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  unsafe fn ProgramUniformMatrix2x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix2x3fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  unsafe fn ProgramUniformMatrix2x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProgramUniformMatrix2x4dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  unsafe fn ProgramUniformMatrix2x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix2x4fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  unsafe fn ProgramUniformMatrix2x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProgramUniformMatrix3dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  unsafe fn ProgramUniformMatrix3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix3fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  unsafe fn ProgramUniformMatrix3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProgramUniformMatrix3x2dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  unsafe fn ProgramUniformMatrix3x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix3x2fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  unsafe fn ProgramUniformMatrix3x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProgramUniformMatrix3x4dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  unsafe fn ProgramUniformMatrix3x4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix3x4fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  unsafe fn ProgramUniformMatrix3x4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProgramUniformMatrix4dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  unsafe fn ProgramUniformMatrix4dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix4fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  unsafe fn ProgramUniformMatrix4fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProgramUniformMatrix4x2dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  unsafe fn ProgramUniformMatrix4x2dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix4x2fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  unsafe fn ProgramUniformMatrix4x2fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProgramUniformMatrix4x3dv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  unsafe fn ProgramUniformMatrix4x3dv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glProgramUniformMatrix4x3fv
  /// * `program` class: program
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  unsafe fn ProgramUniformMatrix4x3fv(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glProvokingVertex
  /// * `mode` group: VertexProvokingMode
  unsafe fn ProvokingVertex(mode: VertexProvokingMode);
  /// glPushDebugGroup
  /// * `source` group: DebugSource
  /// * `message` len: COMPSIZE(message,length)
  unsafe fn PushDebugGroup(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar);
  /// glQueryCounter
  /// * `id` class: query
  /// * `target` group: QueryCounterTarget
  unsafe fn QueryCounter(id: GLuint, target: QueryCounterTarget);
  /// glReadBuffer
  /// * `src` group: ReadBufferMode
  unsafe fn ReadBuffer(src: ReadBufferMode);
  /// glReadPixels
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void);
  /// glReadnPixels
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `data` len: bufSize
  unsafe fn ReadnPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);
  /// glReleaseShaderCompiler
  unsafe fn ReleaseShaderCompiler();
  /// glRenderbufferStorage
  /// * `target` group: RenderbufferTarget
  /// * `internalformat` group: InternalFormat
  unsafe fn RenderbufferStorage(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);
  /// glRenderbufferStorageMultisample
  /// * `target` group: RenderbufferTarget
  /// * `internalformat` group: InternalFormat
  unsafe fn RenderbufferStorageMultisample(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);
  /// glResumeTransformFeedback
  unsafe fn ResumeTransformFeedback();
  /// glSampleCoverage
  /// * `invert` group: Boolean
  unsafe fn SampleCoverage(value: GLfloat, invert: GLboolean);
  /// glSampleMaski
  unsafe fn SampleMaski(maskNumber: GLuint, mask: GLbitfield);
  /// glSamplerParameterIiv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  unsafe fn SamplerParameterIiv(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);
  /// glSamplerParameterIuiv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  unsafe fn SamplerParameterIuiv(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);
  /// glSamplerParameterf
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  unsafe fn SamplerParameterf(sampler: GLuint, pname: SamplerParameterF, param: GLfloat);
  /// glSamplerParameterfv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterF
  /// * `param` len: COMPSIZE(pname)
  unsafe fn SamplerParameterfv(sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat);
  /// glSamplerParameteri
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  unsafe fn SamplerParameteri(sampler: GLuint, pname: SamplerParameterI, param: GLint);
  /// glSamplerParameteriv
  /// * `sampler` class: sampler
  /// * `pname` group: SamplerParameterI
  /// * `param` len: COMPSIZE(pname)
  unsafe fn SamplerParameteriv(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);
  /// glScissor
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  /// glScissorArrayv
  /// * `v` len: COMPSIZE(count)
  unsafe fn ScissorArrayv(first: GLuint, count: GLsizei, v: *const GLint);
  /// glScissorIndexed
  unsafe fn ScissorIndexed(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);
  /// glScissorIndexedv
  unsafe fn ScissorIndexedv(index: GLuint, v: *const [GLint; 4]);
  /// glShaderBinary
  /// * `shaders` class: shader
  /// * `shaders` len: count
  /// * `binaryFormat` group: ShaderBinaryFormat
  /// * `binary` len: length
  unsafe fn ShaderBinary(count: GLsizei, shaders: *const GLuint, binaryFormat: ShaderBinaryFormat, binary: *const void, length: GLsizei);
  /// [glShaderSource](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glShaderSource.xhtml)
  ///
  /// Sets the source string of the named shader. This replaces any previously
  /// set source. OpenGL copies the data into its own memory, so you can free
  /// your instance of the source string after the call (if necessary).
  ///
  /// * `shader` the shader ID to attach the source to.
  /// * `count` the length of the `string` and `length` arrays.
  /// * `string` an array of pointers to the start of shader source code
  ///   strings.
  /// * `length` if non-null, this is an array of lengths for each pointer in
  ///   `string` (or negative if that entry is null-termiated). if `length` is
  ///   null then *all* strings in `string` must each be null-termianted.
  unsafe fn ShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);
  /// glShaderStorageBlockBinding
  /// * `program` class: program
  unsafe fn ShaderStorageBlockBinding(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);
  /// glSpecializeShader
  /// * `shader` class: shader
  unsafe fn SpecializeShader(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);
  /// glStencilFunc
  /// * `func` group: StencilFunction
  /// * `ref` group: StencilValue
  /// * `mask` group: MaskedStencilValue
  unsafe fn StencilFunc(func: StencilFunction, ref_: GLint, mask: GLuint);
  /// glStencilFuncSeparate
  /// * `face` group: StencilFaceDirection
  /// * `func` group: StencilFunction
  /// * `ref` group: StencilValue
  /// * `mask` group: MaskedStencilValue
  unsafe fn StencilFuncSeparate(face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint);
  /// glStencilMask
  /// * `mask` group: MaskedStencilValue
  unsafe fn StencilMask(mask: GLuint);
  /// glStencilMaskSeparate
  /// * `face` group: StencilFaceDirection
  /// * `mask` group: MaskedStencilValue
  unsafe fn StencilMaskSeparate(face: StencilFaceDirection, mask: GLuint);
  /// glStencilOp
  /// * `fail` group: StencilOp
  /// * `zfail` group: StencilOp
  /// * `zpass` group: StencilOp
  unsafe fn StencilOp(fail: StencilOp, zfail: StencilOp, zpass: StencilOp);
  /// glStencilOpSeparate
  /// * `face` group: StencilFaceDirection
  /// * `sfail` group: StencilOp
  /// * `dpfail` group: StencilOp
  /// * `dppass` group: StencilOp
  unsafe fn StencilOpSeparate(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp);
  /// glTexBuffer
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  unsafe fn TexBuffer(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);
  /// glTexBufferRange
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  /// * `offset` group: BufferOffset
  /// * `size` group: BufferSize
  unsafe fn TexBufferRange(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
  /// glTexImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width)
  unsafe fn TexImage1D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTexImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  unsafe fn TexImage2D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTexImage2DMultisample
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  unsafe fn TexImage2DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
  /// glTexImage3D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `internalformat` group: InternalFormat
  /// * `border` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height,depth)
  unsafe fn TexImage3D(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTexImage3DMultisample
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  unsafe fn TexImage3DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
  /// glTexParameterIiv
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn TexParameterIiv(target: TextureTarget, pname: TextureParameterName, params: *const GLint);
  /// glTexParameterIuiv
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` len: COMPSIZE(pname)
  unsafe fn TexParameterIuiv(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);
  /// glTexParameterf
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `param` group: CheckedFloat32
  unsafe fn TexParameterf(target: TextureTarget, pname: TextureParameterName, param: GLfloat);
  /// glTexParameterfv
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` group: CheckedFloat32
  /// * `params` len: COMPSIZE(pname)
  unsafe fn TexParameterfv(target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);
  /// glTexParameteri
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `param` group: CheckedInt32
  unsafe fn TexParameteri(target: TextureTarget, pname: TextureParameterName, param: GLint);
  /// glTexParameteriv
  /// * `target` group: TextureTarget
  /// * `pname` group: TextureParameterName
  /// * `params` group: CheckedInt32
  /// * `params` len: COMPSIZE(pname)
  unsafe fn TexParameteriv(target: TextureTarget, pname: TextureParameterName, params: *const GLint);
  /// glTexStorage1D
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  unsafe fn TexStorage1D(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);
  /// glTexStorage2D
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  unsafe fn TexStorage2D(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);
  /// glTexStorage2DMultisample
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  unsafe fn TexStorage2DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
  /// glTexStorage3D
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  unsafe fn TexStorage3D(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);
  /// glTexStorage3DMultisample
  /// * `target` group: TextureTarget
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  unsafe fn TexStorage3DMultisample(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
  /// glTexSubImage1D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width)
  unsafe fn TexSubImage1D(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTexSubImage2D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height)
  unsafe fn TexSubImage2D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTexSubImage3D
  /// * `target` group: TextureTarget
  /// * `level` group: CheckedInt32
  /// * `xoffset` group: CheckedInt32
  /// * `yoffset` group: CheckedInt32
  /// * `zoffset` group: CheckedInt32
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  /// * `pixels` len: COMPSIZE(format,type,width,height,depth)
  unsafe fn TexSubImage3D(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTextureBarrier
  unsafe fn TextureBarrier();
  /// glTextureBuffer
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  unsafe fn TextureBuffer(texture: GLuint, internalformat: InternalFormat, buffer: GLuint);
  /// glTextureBufferRange
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  unsafe fn TextureBufferRange(texture: GLuint, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
  /// glTextureParameterIiv
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  unsafe fn TextureParameterIiv(texture: GLuint, pname: TextureParameterName, params: *const GLint);
  /// glTextureParameterIuiv
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  unsafe fn TextureParameterIuiv(texture: GLuint, pname: TextureParameterName, params: *const GLuint);
  /// glTextureParameterf
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  unsafe fn TextureParameterf(texture: GLuint, pname: TextureParameterName, param: GLfloat);
  /// glTextureParameterfv
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  unsafe fn TextureParameterfv(texture: GLuint, pname: TextureParameterName, param: *const GLfloat);
  /// glTextureParameteri
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  unsafe fn TextureParameteri(texture: GLuint, pname: TextureParameterName, param: GLint);
  /// glTextureParameteriv
  /// * `texture` class: texture
  /// * `pname` group: TextureParameterName
  unsafe fn TextureParameteriv(texture: GLuint, pname: TextureParameterName, param: *const GLint);
  /// glTextureStorage1D
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  unsafe fn TextureStorage1D(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);
  /// glTextureStorage2D
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  unsafe fn TextureStorage2D(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);
  /// glTextureStorage2DMultisample
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  unsafe fn TextureStorage2DMultisample(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);
  /// glTextureStorage3D
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  unsafe fn TextureStorage3D(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);
  /// glTextureStorage3DMultisample
  /// * `texture` class: texture
  /// * `internalformat` group: InternalFormat
  /// * `fixedsamplelocations` group: Boolean
  unsafe fn TextureStorage3DMultisample(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);
  /// glTextureSubImage1D
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  unsafe fn TextureSubImage1D(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTextureSubImage2D
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  unsafe fn TextureSubImage2D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTextureSubImage3D
  /// * `texture` class: texture
  /// * `format` group: PixelFormat
  /// * `type` group: PixelType
  unsafe fn TextureSubImage3D(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);
  /// glTextureView
  /// * `texture` class: texture
  /// * `target` group: TextureTarget
  /// * `origtexture` class: texture
  /// * `internalformat` group: InternalFormat
  unsafe fn TextureView(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);
  /// glTransformFeedbackBufferBase
  /// * `xfb` class: transform feedback
  /// * `buffer` class: buffer
  unsafe fn TransformFeedbackBufferBase(xfb: GLuint, index: GLuint, buffer: GLuint);
  /// glTransformFeedbackBufferRange
  /// * `xfb` class: transform feedback
  /// * `buffer` class: buffer
  /// * `size` group: BufferSize
  unsafe fn TransformFeedbackBufferRange(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);
  /// glTransformFeedbackVaryings
  /// * `program` class: program
  /// * `varyings` len: count
  /// * `bufferMode` group: TransformFeedbackBufferMode
  unsafe fn TransformFeedbackVaryings(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode);
  /// glUniform1d
  unsafe fn Uniform1d(location: GLint, x: GLdouble);
  /// glUniform1dv
  /// * `value` len: count*1
  unsafe fn Uniform1dv(location: GLint, count: GLsizei, value: *const GLdouble);
  /// glUniform1f
  unsafe fn Uniform1f(location: GLint, v0: GLfloat);
  /// glUniform1fv
  /// * `value` len: count*1
  unsafe fn Uniform1fv(location: GLint, count: GLsizei, value: *const GLfloat);
  /// glUniform1i
  unsafe fn Uniform1i(location: GLint, v0: GLint);
  /// glUniform1iv
  /// * `value` len: count*1
  unsafe fn Uniform1iv(location: GLint, count: GLsizei, value: *const GLint);
  /// glUniform1ui
  unsafe fn Uniform1ui(location: GLint, v0: GLuint);
  /// glUniform1uiv
  /// * `value` len: count*1
  unsafe fn Uniform1uiv(location: GLint, count: GLsizei, value: *const GLuint);
  /// glUniform2d
  unsafe fn Uniform2d(location: GLint, x: GLdouble, y: GLdouble);
  /// glUniform2dv
  /// * `value` len: count*2
  unsafe fn Uniform2dv(location: GLint, count: GLsizei, value: *const GLdouble);
  /// glUniform2f
  unsafe fn Uniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
  /// glUniform2fv
  /// * `value` len: count*2
  unsafe fn Uniform2fv(location: GLint, count: GLsizei, value: *const GLfloat);
  /// glUniform2i
  unsafe fn Uniform2i(location: GLint, v0: GLint, v1: GLint);
  /// glUniform2iv
  /// * `value` len: count*2
  unsafe fn Uniform2iv(location: GLint, count: GLsizei, value: *const GLint);
  /// glUniform2ui
  unsafe fn Uniform2ui(location: GLint, v0: GLuint, v1: GLuint);
  /// glUniform2uiv
  /// * `value` len: count*2
  unsafe fn Uniform2uiv(location: GLint, count: GLsizei, value: *const GLuint);
  /// glUniform3d
  unsafe fn Uniform3d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);
  /// glUniform3dv
  /// * `value` len: count*3
  unsafe fn Uniform3dv(location: GLint, count: GLsizei, value: *const GLdouble);
  /// glUniform3f
  unsafe fn Uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
  /// glUniform3fv
  /// * `value` len: count*3
  unsafe fn Uniform3fv(location: GLint, count: GLsizei, value: *const GLfloat);
  /// glUniform3i
  unsafe fn Uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint);
  /// glUniform3iv
  /// * `value` len: count*3
  unsafe fn Uniform3iv(location: GLint, count: GLsizei, value: *const GLint);
  /// glUniform3ui
  unsafe fn Uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
  /// glUniform3uiv
  /// * `value` len: count*3
  unsafe fn Uniform3uiv(location: GLint, count: GLsizei, value: *const GLuint);
  /// glUniform4d
  unsafe fn Uniform4d(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
  /// glUniform4dv
  /// * `value` len: count*4
  unsafe fn Uniform4dv(location: GLint, count: GLsizei, value: *const GLdouble);
  /// glUniform4f
  unsafe fn Uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
  /// glUniform4fv
  /// * `value` len: count*4
  unsafe fn Uniform4fv(location: GLint, count: GLsizei, value: *const GLfloat);
  /// glUniform4i
  unsafe fn Uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
  /// glUniform4iv
  /// * `value` len: count*4
  unsafe fn Uniform4iv(location: GLint, count: GLsizei, value: *const GLint);
  /// glUniform4ui
  unsafe fn Uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
  /// glUniform4uiv
  /// * `value` len: count*4
  unsafe fn Uniform4uiv(location: GLint, count: GLsizei, value: *const GLuint);
  /// glUniformBlockBinding
  /// * `program` class: program
  unsafe fn UniformBlockBinding(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);
  /// glUniformMatrix2dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  unsafe fn UniformMatrix2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix2fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*4
  unsafe fn UniformMatrix2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformMatrix2x3dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  unsafe fn UniformMatrix2x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix2x3fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  unsafe fn UniformMatrix2x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformMatrix2x4dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  unsafe fn UniformMatrix2x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix2x4fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  unsafe fn UniformMatrix2x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformMatrix3dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  unsafe fn UniformMatrix3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix3fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*9
  unsafe fn UniformMatrix3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformMatrix3x2dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  unsafe fn UniformMatrix3x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix3x2fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*6
  unsafe fn UniformMatrix3x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformMatrix3x4dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  unsafe fn UniformMatrix3x4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix3x4fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  unsafe fn UniformMatrix3x4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformMatrix4dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  unsafe fn UniformMatrix4dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix4fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*16
  unsafe fn UniformMatrix4fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformMatrix4x2dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  unsafe fn UniformMatrix4x2dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix4x2fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*8
  unsafe fn UniformMatrix4x2fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformMatrix4x3dv
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  unsafe fn UniformMatrix4x3dv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);
  /// glUniformMatrix4x3fv
  /// * `transpose` group: Boolean
  /// * `value` len: count*12
  unsafe fn UniformMatrix4x3fv(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);
  /// glUniformSubroutinesuiv
  /// * `shadertype` group: ShaderType
  /// * `indices` len: count
  unsafe fn UniformSubroutinesuiv(shadertype: ShaderType, count: GLsizei, indices: *const GLuint);
  /// glUnmapBuffer
  /// * `target` group: BufferTargetARB
  unsafe fn UnmapBuffer(target: BufferTargetARB) -> GLboolean;
  /// glUnmapNamedBuffer
  /// * `buffer` class: buffer
  unsafe fn UnmapNamedBuffer(buffer: GLuint) -> GLboolean;
  /// [glUseProgram](https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glUseProgram.xhtml)
  ///
  /// Sets a given shader program for use during rendering.
  ///
  /// Setting 0 as the program object makes the output of all rendering actions
  /// undefined, but this is not an error.
  ///
  /// * `program` names the program to set for use.
  fn UseProgram(program: GLuint);
  /// glUseProgramStages
  /// * `pipeline` class: program pipeline
  /// * `stages` group: UseProgramStageMask
  /// * `program` class: program
  unsafe fn UseProgramStages(pipeline: GLuint, stages: GLbitfield, program: GLuint);
  /// glValidateProgram
  /// * `program` class: program
  unsafe fn ValidateProgram(program: GLuint);
  /// glValidateProgramPipeline
  /// * `pipeline` class: program pipeline
  unsafe fn ValidateProgramPipeline(pipeline: GLuint);
  /// glVertexArrayAttribBinding
  /// * `vaobj` class: vertex array
  unsafe fn VertexArrayAttribBinding(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);
  /// glVertexArrayAttribFormat
  /// * `vaobj` class: vertex array
  /// * `type` group: VertexAttribType
  /// * `normalized` group: Boolean
  unsafe fn VertexArrayAttribFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);
  /// glVertexArrayAttribIFormat
  /// * `vaobj` class: vertex array
  /// * `type` group: VertexAttribIType
  unsafe fn VertexArrayAttribIFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);
  /// glVertexArrayAttribLFormat
  /// * `vaobj` class: vertex array
  /// * `type` group: VertexAttribLType
  unsafe fn VertexArrayAttribLFormat(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);
  /// glVertexArrayBindingDivisor
  /// * `vaobj` class: vertex array
  unsafe fn VertexArrayBindingDivisor(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);
  /// glVertexArrayElementBuffer
  /// * `vaobj` class: vertex array
  /// * `buffer` class: buffer
  unsafe fn VertexArrayElementBuffer(vaobj: GLuint, buffer: GLuint);
  /// glVertexArrayVertexBuffer
  /// * `vaobj` class: vertex array
  /// * `buffer` class: buffer
  unsafe fn VertexArrayVertexBuffer(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);
  /// glVertexArrayVertexBuffers
  /// * `vaobj` class: vertex array
  /// * `buffers` class: buffer
  unsafe fn VertexArrayVertexBuffers(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);
  /// glVertexAttrib1d
  unsafe fn VertexAttrib1d(index: GLuint, x: GLdouble);
  /// glVertexAttrib1dv
  unsafe fn VertexAttrib1dv(index: GLuint, v: *const GLdouble);
  /// glVertexAttrib1f
  unsafe fn VertexAttrib1f(index: GLuint, x: GLfloat);
  /// glVertexAttrib1fv
  unsafe fn VertexAttrib1fv(index: GLuint, v: *const GLfloat);
  /// glVertexAttrib1s
  unsafe fn VertexAttrib1s(index: GLuint, x: GLshort);
  /// glVertexAttrib1sv
  unsafe fn VertexAttrib1sv(index: GLuint, v: *const GLshort);
  /// glVertexAttrib2d
  unsafe fn VertexAttrib2d(index: GLuint, x: GLdouble, y: GLdouble);
  /// glVertexAttrib2dv
  unsafe fn VertexAttrib2dv(index: GLuint, v: *const [GLdouble; 2]);
  /// glVertexAttrib2f
  unsafe fn VertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat);
  /// glVertexAttrib2fv
  unsafe fn VertexAttrib2fv(index: GLuint, v: *const [GLfloat; 2]);
  /// glVertexAttrib2s
  unsafe fn VertexAttrib2s(index: GLuint, x: GLshort, y: GLshort);
  /// glVertexAttrib2sv
  unsafe fn VertexAttrib2sv(index: GLuint, v: *const [GLshort; 2]);
  /// glVertexAttrib3d
  unsafe fn VertexAttrib3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
  /// glVertexAttrib3dv
  unsafe fn VertexAttrib3dv(index: GLuint, v: *const [GLdouble; 3]);
  /// glVertexAttrib3f
  unsafe fn VertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
  /// glVertexAttrib3fv
  unsafe fn VertexAttrib3fv(index: GLuint, v: *const [GLfloat; 3]);
  /// glVertexAttrib3s
  unsafe fn VertexAttrib3s(index: GLuint, x: GLshort, y: GLshort, z: GLshort);
  /// glVertexAttrib3sv
  unsafe fn VertexAttrib3sv(index: GLuint, v: *const [GLshort; 3]);
  /// glVertexAttrib4Nbv
  unsafe fn VertexAttrib4Nbv(index: GLuint, v: *const [GLbyte; 4]);
  /// glVertexAttrib4Niv
  unsafe fn VertexAttrib4Niv(index: GLuint, v: *const [GLint; 4]);
  /// glVertexAttrib4Nsv
  unsafe fn VertexAttrib4Nsv(index: GLuint, v: *const [GLshort; 4]);
  /// glVertexAttrib4Nub
  unsafe fn VertexAttrib4Nub(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);
  /// glVertexAttrib4Nubv
  unsafe fn VertexAttrib4Nubv(index: GLuint, v: *const [GLubyte; 4]);
  /// glVertexAttrib4Nuiv
  unsafe fn VertexAttrib4Nuiv(index: GLuint, v: *const [GLuint; 4]);
  /// glVertexAttrib4Nusv
  unsafe fn VertexAttrib4Nusv(index: GLuint, v: *const [GLushort; 4]);
  /// glVertexAttrib4bv
  unsafe fn VertexAttrib4bv(index: GLuint, v: *const [GLbyte; 4]);
  /// glVertexAttrib4d
  unsafe fn VertexAttrib4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
  /// glVertexAttrib4dv
  unsafe fn VertexAttrib4dv(index: GLuint, v: *const [GLdouble; 4]);
  /// glVertexAttrib4f
  unsafe fn VertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
  /// glVertexAttrib4fv
  unsafe fn VertexAttrib4fv(index: GLuint, v: *const [GLfloat; 4]);
  /// glVertexAttrib4iv
  unsafe fn VertexAttrib4iv(index: GLuint, v: *const [GLint; 4]);
  /// glVertexAttrib4s
  unsafe fn VertexAttrib4s(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);
  /// glVertexAttrib4sv
  unsafe fn VertexAttrib4sv(index: GLuint, v: *const [GLshort; 4]);
  /// glVertexAttrib4ubv
  unsafe fn VertexAttrib4ubv(index: GLuint, v: *const [GLubyte; 4]);
  /// glVertexAttrib4uiv
  unsafe fn VertexAttrib4uiv(index: GLuint, v: *const [GLuint; 4]);
  /// glVertexAttrib4usv
  unsafe fn VertexAttrib4usv(index: GLuint, v: *const [GLushort; 4]);
  /// glVertexAttribBinding
  unsafe fn VertexAttribBinding(attribindex: GLuint, bindingindex: GLuint);
  /// glVertexAttribDivisor
  unsafe fn VertexAttribDivisor(index: GLuint, divisor: GLuint);
  /// glVertexAttribFormat
  /// * `type` group: VertexAttribType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribFormat(attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);
  /// glVertexAttribI1i
  unsafe fn VertexAttribI1i(index: GLuint, x: GLint);
  /// glVertexAttribI1iv
  unsafe fn VertexAttribI1iv(index: GLuint, v: *const GLint);
  /// glVertexAttribI1ui
  unsafe fn VertexAttribI1ui(index: GLuint, x: GLuint);
  /// glVertexAttribI1uiv
  unsafe fn VertexAttribI1uiv(index: GLuint, v: *const GLuint);
  /// glVertexAttribI2i
  unsafe fn VertexAttribI2i(index: GLuint, x: GLint, y: GLint);
  /// glVertexAttribI2iv
  unsafe fn VertexAttribI2iv(index: GLuint, v: *const [GLint; 2]);
  /// glVertexAttribI2ui
  unsafe fn VertexAttribI2ui(index: GLuint, x: GLuint, y: GLuint);
  /// glVertexAttribI2uiv
  unsafe fn VertexAttribI2uiv(index: GLuint, v: *const [GLuint; 2]);
  /// glVertexAttribI3i
  unsafe fn VertexAttribI3i(index: GLuint, x: GLint, y: GLint, z: GLint);
  /// glVertexAttribI3iv
  unsafe fn VertexAttribI3iv(index: GLuint, v: *const [GLint; 3]);
  /// glVertexAttribI3ui
  unsafe fn VertexAttribI3ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint);
  /// glVertexAttribI3uiv
  unsafe fn VertexAttribI3uiv(index: GLuint, v: *const [GLuint; 3]);
  /// glVertexAttribI4bv
  unsafe fn VertexAttribI4bv(index: GLuint, v: *const [GLbyte; 4]);
  /// glVertexAttribI4i
  unsafe fn VertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
  /// glVertexAttribI4iv
  unsafe fn VertexAttribI4iv(index: GLuint, v: *const [GLint; 4]);
  /// glVertexAttribI4sv
  unsafe fn VertexAttribI4sv(index: GLuint, v: *const [GLshort; 4]);
  /// glVertexAttribI4ubv
  unsafe fn VertexAttribI4ubv(index: GLuint, v: *const [GLubyte; 4]);
  /// glVertexAttribI4ui
  unsafe fn VertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
  /// glVertexAttribI4uiv
  unsafe fn VertexAttribI4uiv(index: GLuint, v: *const [GLuint; 4]);
  /// glVertexAttribI4usv
  unsafe fn VertexAttribI4usv(index: GLuint, v: *const [GLushort; 4]);
  /// glVertexAttribIFormat
  /// * `type` group: VertexAttribIType
  unsafe fn VertexAttribIFormat(attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);
  /// glVertexAttribIPointer
  /// * `type` group: VertexAttribIType
  /// * `pointer` len: COMPSIZE(size,type,stride)
  unsafe fn VertexAttribIPointer(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void);
  /// glVertexAttribL1d
  unsafe fn VertexAttribL1d(index: GLuint, x: GLdouble);
  /// glVertexAttribL1dv
  unsafe fn VertexAttribL1dv(index: GLuint, v: *const GLdouble);
  /// glVertexAttribL2d
  unsafe fn VertexAttribL2d(index: GLuint, x: GLdouble, y: GLdouble);
  /// glVertexAttribL2dv
  unsafe fn VertexAttribL2dv(index: GLuint, v: *const [GLdouble; 2]);
  /// glVertexAttribL3d
  unsafe fn VertexAttribL3d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);
  /// glVertexAttribL3dv
  unsafe fn VertexAttribL3dv(index: GLuint, v: *const [GLdouble; 3]);
  /// glVertexAttribL4d
  unsafe fn VertexAttribL4d(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);
  /// glVertexAttribL4dv
  unsafe fn VertexAttribL4dv(index: GLuint, v: *const [GLdouble; 4]);
  /// glVertexAttribLFormat
  /// * `type` group: VertexAttribLType
  unsafe fn VertexAttribLFormat(attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);
  /// glVertexAttribLPointer
  /// * `type` group: VertexAttribLType
  /// * `pointer` len: size
  unsafe fn VertexAttribLPointer(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, pointer: *const void);
  /// glVertexAttribP1ui
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribP1ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);
  /// glVertexAttribP1uiv
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribP1uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);
  /// glVertexAttribP2ui
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribP2ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);
  /// glVertexAttribP2uiv
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribP2uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);
  /// glVertexAttribP3ui
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribP3ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);
  /// glVertexAttribP3uiv
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribP3uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);
  /// glVertexAttribP4ui
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribP4ui(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);
  /// glVertexAttribP4uiv
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  unsafe fn VertexAttribP4uiv(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);
  /// glVertexAttribPointer
  /// * `type` group: VertexAttribPointerType
  /// * `normalized` group: Boolean
  /// * `pointer` len: COMPSIZE(size,type,stride)
  unsafe fn VertexAttribPointer(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void);
  /// glVertexBindingDivisor
  unsafe fn VertexBindingDivisor(bindingindex: GLuint, divisor: GLuint);
  /// glViewport
  /// * `x` group: WinCoord
  /// * `y` group: WinCoord
  unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
  /// glViewportArrayv
  /// * `v` len: COMPSIZE(count)
  unsafe fn ViewportArrayv(first: GLuint, count: GLsizei, v: *const GLfloat);
  /// glViewportIndexedf
  unsafe fn ViewportIndexedf(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);
  /// glViewportIndexedfv
  unsafe fn ViewportIndexedfv(index: GLuint, v: *const [GLfloat; 4]);
  /// glWaitSync
  /// * `sync` group: sync
  /// * `sync` class: sync
  /// * `flags` group: SyncBehaviorFlags
  unsafe fn WaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64);

  /// glGetImageHandleARB
  /// * `texture` class: texture
  /// * `layered` group: Boolean
  /// * `format` group: PixelFormat
  unsafe ext fn GetImageHandleARB(texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: PixelFormat) -> GLuint64;
  /// glGetTextureHandleARB
  /// * `texture` class: texture
  unsafe ext fn GetTextureHandleARB(texture: GLuint) -> GLuint64;
  /// glGetTextureSamplerHandleARB
  /// * `texture` class: texture
  /// * `sampler` class: sampler
  unsafe ext fn GetTextureSamplerHandleARB(texture: GLuint, sampler: GLuint) -> GLuint64;
  /// glGetVertexAttribLui64vARB
  /// * `pname` group: VertexAttribEnum
  unsafe ext fn GetVertexAttribLui64vARB(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint64EXT);
  /// glIsImageHandleResidentARB
  unsafe ext fn IsImageHandleResidentARB(handle: GLuint64) -> GLboolean;
  /// glIsTextureHandleResidentARB
  unsafe ext fn IsTextureHandleResidentARB(handle: GLuint64) -> GLboolean;
  /// glMakeImageHandleNonResidentARB
  unsafe ext fn MakeImageHandleNonResidentARB(handle: GLuint64);
  /// glMakeImageHandleResidentARB
  unsafe ext fn MakeImageHandleResidentARB(handle: GLuint64, access: GLenum);
  /// glMakeTextureHandleNonResidentARB
  unsafe ext fn MakeTextureHandleNonResidentARB(handle: GLuint64);
  /// glMakeTextureHandleResidentARB
  unsafe ext fn MakeTextureHandleResidentARB(handle: GLuint64);
  /// glProgramUniformHandleui64ARB
  /// * `program` class: program
  unsafe ext fn ProgramUniformHandleui64ARB(program: GLuint, location: GLint, value: GLuint64);
  /// glProgramUniformHandleui64vARB
  /// * `program` class: program
  /// * `values` len: count
  unsafe ext fn ProgramUniformHandleui64vARB(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);
  /// glTexPageCommitmentARB
  /// * `commit` group: Boolean
  unsafe ext fn TexPageCommitmentARB(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);
  /// glUniformHandleui64ARB
  unsafe ext fn UniformHandleui64ARB(location: GLint, value: GLuint64);
  /// glUniformHandleui64vARB
  /// * `value` len: count
  unsafe ext fn UniformHandleui64vARB(location: GLint, count: GLsizei, value: *const GLuint64);
  /// glVertexAttribL1ui64ARB
  unsafe ext fn VertexAttribL1ui64ARB(index: GLuint, x: GLuint64EXT);
  /// glVertexAttribL1ui64vARB
  unsafe ext fn VertexAttribL1ui64vARB(index: GLuint, v: *const GLuint64EXT);
}
