use super::*;

pub(crate) type glActiveShaderProgram_t = unsafe extern "system" fn(pipeline: GLuint, program: GLuint);

pub(crate) type glActiveTexture_t = unsafe extern "system" fn(texture: TextureUnit);

pub(crate) type glAttachShader_t = extern "system" fn(program: GLuint, shader: GLuint);

pub(crate) type glBeginConditionalRender_t = unsafe extern "system" fn(id: GLuint, mode: ConditionalRenderMode);

pub(crate) type glBeginQuery_t = unsafe extern "system" fn(target: QueryTarget, id: GLuint);

pub(crate) type glBeginQueryIndexed_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint, id: GLuint);

pub(crate) type glBeginTransformFeedback_t = unsafe extern "system" fn(primitiveMode: PrimitiveType);

pub(crate) type glBindAttribLocation_t = unsafe extern "system" fn(program: GLuint, index: GLuint, name: *const GLchar);

pub(crate) type glBindBuffer_t = unsafe extern "system" fn(target: BufferTargetARB, buffer: GLuint);

pub(crate) type glBindBufferBase_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint);

pub(crate) type glBindBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub(crate) type glBindBuffersBase_t = unsafe extern "system" fn(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint);

pub(crate) type glBindBuffersRange_t = unsafe extern "system" fn(target: BufferTargetARB, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, sizes: *const GLsizeiptr);

pub(crate) type glBindFragDataLocation_t = unsafe extern "system" fn(program: GLuint, color: GLuint, name: *const GLchar);

pub(crate) type glBindFragDataLocationIndexed_t = unsafe extern "system" fn(program: GLuint, colorNumber: GLuint, index: GLuint, name: *const GLchar);

pub(crate) type glBindFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, framebuffer: GLuint);

pub(crate) type glBindImageTexture_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, access: BufferAccessARB, format: InternalFormat);

pub(crate) type glBindImageTextures_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);

pub(crate) type glBindProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);

pub(crate) type glBindRenderbuffer_t = unsafe extern "system" fn(target: RenderbufferTarget, renderbuffer: GLuint);

pub(crate) type glBindSampler_t = unsafe extern "system" fn(unit: GLuint, sampler: GLuint);

pub(crate) type glBindSamplers_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, samplers: *const GLuint);

pub(crate) type glBindTexture_t = unsafe extern "system" fn(target: TextureTarget, texture: GLuint);

pub(crate) type glBindTextureUnit_t = unsafe extern "system" fn(unit: GLuint, texture: GLuint);

pub(crate) type glBindTextures_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, textures: *const GLuint);

pub(crate) type glBindTransformFeedback_t = unsafe extern "system" fn(target: BindTransformFeedbackTarget, id: GLuint);

pub(crate) type glBindVertexArray_t = extern "system" fn(array: GLuint);

pub(crate) type glBindVertexBuffer_t = unsafe extern "system" fn(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

pub(crate) type glBindVertexBuffers_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);

pub(crate) type glBlendColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub(crate) type glBlendEquation_t = unsafe extern "system" fn(mode: BlendEquationModeEXT);

pub(crate) type glBlendEquationSeparate_t = unsafe extern "system" fn(modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

pub(crate) type glBlendEquationSeparatei_t = unsafe extern "system" fn(buf: GLuint, modeRGB: BlendEquationModeEXT, modeAlpha: BlendEquationModeEXT);

pub(crate) type glBlendEquationi_t = unsafe extern "system" fn(buf: GLuint, mode: BlendEquationModeEXT);

pub(crate) type glBlendFunc_t = unsafe extern "system" fn(sfactor: BlendingFactor, dfactor: BlendingFactor);

pub(crate) type glBlendFuncSeparate_t = unsafe extern "system" fn(sfactorRGB: BlendingFactor, dfactorRGB: BlendingFactor, sfactorAlpha: BlendingFactor, dfactorAlpha: BlendingFactor);

pub(crate) type glBlendFuncSeparatei_t = unsafe extern "system" fn(buf: GLuint, srcRGB: BlendingFactor, dstRGB: BlendingFactor, srcAlpha: BlendingFactor, dstAlpha: BlendingFactor);

pub(crate) type glBlendFunci_t = unsafe extern "system" fn(buf: GLuint, src: BlendingFactor, dst: BlendingFactor);

pub(crate) type glBlitFramebuffer_t = unsafe extern "system" fn(srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

pub(crate) type glBlitNamedFramebuffer_t = unsafe extern "system" fn(readFramebuffer: GLuint, drawFramebuffer: GLuint, srcX0: GLint, srcY0: GLint, srcX1: GLint, srcY1: GLint, dstX0: GLint, dstY0: GLint, dstX1: GLint, dstY1: GLint, mask: GLbitfield, filter: BlitFramebufferFilter);

pub(crate) type glBufferData_t = unsafe extern "system" fn(target: BufferTargetARB, size: GLsizeiptr, data: *const void, usage: BufferUsageARB);

pub(crate) type glBufferStorage_t = unsafe extern "system" fn(target: BufferStorageTarget, size: GLsizeiptr, data: *const void, flags: GLbitfield);

pub(crate) type glBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *const void);

pub(crate) type glCheckFramebufferStatus_t = unsafe extern "system" fn(target: FramebufferTarget) -> FramebufferStatus;

pub(crate) type glCheckNamedFramebufferStatus_t = unsafe extern "system" fn(framebuffer: GLuint, target: FramebufferTarget) -> FramebufferStatus;

pub(crate) type glClampColor_t = unsafe extern "system" fn(target: ClampColorTargetARB, clamp: ClampColorModeARB);

pub(crate) type glClear_t = unsafe extern "system" fn(mask: GLbitfield);

pub(crate) type glClearBufferData_t = unsafe extern "system" fn(target: BufferStorageTarget, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);

pub(crate) type glClearBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);

pub(crate) type glClearBufferfi_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

pub(crate) type glClearBufferfv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);

pub(crate) type glClearBufferiv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLint);

pub(crate) type glClearBufferuiv_t = unsafe extern "system" fn(buffer: Buffer, drawbuffer: GLint, value: *const GLuint);

pub(crate) type glClearColor_t = unsafe extern "system" fn(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);

pub(crate) type glClearDepth_t = unsafe extern "system" fn(depth: GLdouble);

pub(crate) type glClearDepthf_t = unsafe extern "system" fn(d: GLfloat);

pub(crate) type glClearNamedBufferData_t = unsafe extern "system" fn(buffer: GLuint, internalformat: InternalFormat, format: PixelFormat, type_: PixelType, data: *const void);

pub(crate) type glClearNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, internalformat: InternalFormat, offset: GLintptr, size: GLsizeiptr, format: PixelFormat, type_: PixelType, data: *const void);

pub(crate) type glClearNamedFramebufferfi_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, depth: GLfloat, stencil: GLint);

pub(crate) type glClearNamedFramebufferfv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLfloat);

pub(crate) type glClearNamedFramebufferiv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLint);

pub(crate) type glClearNamedFramebufferuiv_t = unsafe extern "system" fn(framebuffer: GLuint, buffer: Buffer, drawbuffer: GLint, value: *const GLuint);

pub(crate) type glClearStencil_t = unsafe extern "system" fn(s: GLint);

pub(crate) type glClearTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, data: *const void);

pub(crate) type glClearTexSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, data: *const void);

pub(crate) type glClientWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> SyncStatus;

pub(crate) type glClipControl_t = unsafe extern "system" fn(origin: ClipControlOrigin, depth: ClipControlDepth);

pub(crate) type glColorMask_t = unsafe extern "system" fn(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);

pub(crate) type glColorMaski_t = unsafe extern "system" fn(index: GLuint, r: GLboolean, g: GLboolean, b: GLboolean, a: GLboolean);

pub(crate) type glCompileShader_t = extern "system" fn(shader: GLuint);

pub(crate) type glCompressedTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCompressedTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, imageSize: GLsizei, data: *const void);

pub(crate) type glCopyBufferSubData_t = unsafe extern "system" fn(readTarget: CopyBufferSubDataTarget, writeTarget: CopyBufferSubDataTarget, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

pub(crate) type glCopyImageSubData_t = unsafe extern "system" fn(srcName: GLuint, srcTarget: CopyImageSubDataTarget, srcLevel: GLint, srcX: GLint, srcY: GLint, srcZ: GLint, dstName: GLuint, dstTarget: CopyImageSubDataTarget, dstLevel: GLint, dstX: GLint, dstY: GLint, dstZ: GLint, srcWidth: GLsizei, srcHeight: GLsizei, srcDepth: GLsizei);

pub(crate) type glCopyNamedBufferSubData_t = unsafe extern "system" fn(readBuffer: GLuint, writeBuffer: GLuint, readOffset: GLintptr, writeOffset: GLintptr, size: GLsizeiptr);

pub(crate) type glCopyTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, border: GLint);

pub(crate) type glCopyTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: InternalFormat, x: GLint, y: GLint, width: GLsizei, height: GLsizei, border: GLint);

pub(crate) type glCopyTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

pub(crate) type glCopyTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glCopyTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glCopyTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, x: GLint, y: GLint, width: GLsizei);

pub(crate) type glCopyTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glCopyTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glCreateBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

pub(crate) type glCreateFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

pub(crate) type glCreateProgram_t = extern "system" fn() -> GLuint;

pub(crate) type glCreateProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

pub(crate) type glCreateQueries_t = unsafe extern "system" fn(target: QueryTarget, n: GLsizei, ids: *mut GLuint);

pub(crate) type glCreateRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

pub(crate) type glCreateSamplers_t = unsafe extern "system" fn(n: GLsizei, samplers: *mut GLuint);

pub(crate) type glCreateShader_t = extern "system" fn(type_: ShaderType) -> GLuint;

pub(crate) type glCreateShaderProgramv_t = unsafe extern "system" fn(type_: ShaderType, count: GLsizei, strings: *const *const GLchar) -> GLuint;

pub(crate) type glCreateTextures_t = unsafe extern "system" fn(target: TextureTarget, n: GLsizei, textures: *mut GLuint);

pub(crate) type glCreateTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub(crate) type glCreateVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

pub(crate) type glCullFace_t = unsafe extern "system" fn(mode: CullFaceMode);

pub(crate) type glDebugMessageCallback_t = unsafe extern "system" fn(callback: GLDEBUGPROC, userParam: *const void);

pub(crate) type glDebugMessageControl_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, severity: DebugSeverity, count: GLsizei, ids: *const GLuint, enabled: GLboolean);

pub(crate) type glDebugMessageInsert_t = unsafe extern "system" fn(source: DebugSource, type_: DebugType, id: GLuint, severity: DebugSeverity, length: GLsizei, buf: *const GLchar);

pub(crate) type glDeleteBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *const GLuint);

pub(crate) type glDeleteFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *const GLuint);

pub(crate) type glDeleteProgram_t = extern "system" fn(program: GLuint);

pub(crate) type glDeleteProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *const GLuint);

pub(crate) type glDeleteQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub(crate) type glDeleteRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *const GLuint);

pub(crate) type glDeleteSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *const GLuint);

pub(crate) type glDeleteShader_t = extern "system" fn(shader: GLuint);

pub(crate) type glDeleteSync_t = unsafe extern "system" fn(sync: GLsync);

pub(crate) type glDeleteTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *const GLuint);

pub(crate) type glDeleteTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *const GLuint);

pub(crate) type glDeleteVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *const GLuint);

pub(crate) type glDepthFunc_t = unsafe extern "system" fn(func: DepthFunction);

pub(crate) type glDepthMask_t = unsafe extern "system" fn(flag: GLboolean);

pub(crate) type glDepthRange_t = unsafe extern "system" fn(n: GLdouble, f: GLdouble);

pub(crate) type glDepthRangeArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLdouble);

pub(crate) type glDepthRangeIndexed_t = unsafe extern "system" fn(index: GLuint, n: GLdouble, f: GLdouble);

pub(crate) type glDepthRangef_t = unsafe extern "system" fn(n: GLfloat, f: GLfloat);

pub(crate) type glDetachShader_t = unsafe extern "system" fn(program: GLuint, shader: GLuint);

pub(crate) type glDisable_t = unsafe extern "system" fn(cap: EnableCap);

pub(crate) type glDisableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

pub(crate) type glDisableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

pub(crate) type glDisablei_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

pub(crate) type glDispatchCompute_t = unsafe extern "system" fn(num_groups_x: GLuint, num_groups_y: GLuint, num_groups_z: GLuint);

pub(crate) type glDispatchComputeIndirect_t = unsafe extern "system" fn(indirect: GLintptr);

pub(crate) type glDrawArrays_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei);

pub(crate) type glDrawArraysIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void);

pub(crate) type glDrawArraysInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei);

pub(crate) type glDrawArraysInstancedBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, first: GLint, count: GLsizei, instancecount: GLsizei, baseinstance: GLuint);

pub(crate) type glDrawBuffer_t = unsafe extern "system" fn(buf: DrawBufferMode);

pub(crate) type glDrawBuffers_t = unsafe extern "system" fn(n: GLsizei, bufs: *const DrawBufferMode);

pub(crate) type glDrawElements_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void);

pub(crate) type glDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

pub(crate) type glDrawElementsIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void);

pub(crate) type glDrawElementsInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei);

pub(crate) type glDrawElementsInstancedBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: PrimitiveType, indices: *const void, instancecount: GLsizei, baseinstance: GLuint);

pub(crate) type glDrawElementsInstancedBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint);

pub(crate) type glDrawElementsInstancedBaseVertexBaseInstance_t = unsafe extern "system" fn(mode: PrimitiveType, count: GLsizei, type_: DrawElementsType, indices: *const void, instancecount: GLsizei, basevertex: GLint, baseinstance: GLuint);

pub(crate) type glDrawRangeElements_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void);

pub(crate) type glDrawRangeElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, start: GLuint, end: GLuint, count: GLsizei, type_: DrawElementsType, indices: *const void, basevertex: GLint);

pub(crate) type glDrawTransformFeedback_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint);

pub(crate) type glDrawTransformFeedbackInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, instancecount: GLsizei);

pub(crate) type glDrawTransformFeedbackStream_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, stream: GLuint);

pub(crate) type glDrawTransformFeedbackStreamInstanced_t = unsafe extern "system" fn(mode: PrimitiveType, id: GLuint, stream: GLuint, instancecount: GLsizei);

pub(crate) type glEnable_t = unsafe extern "system" fn(cap: EnableCap);

pub(crate) type glEnableVertexArrayAttrib_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint);

pub(crate) type glEnableVertexAttribArray_t = unsafe extern "system" fn(index: GLuint);

pub(crate) type glEnablei_t = unsafe extern "system" fn(target: EnableCap, index: GLuint);

pub(crate) type glEndConditionalRender_t = unsafe extern "system" fn();

pub(crate) type glEndQuery_t = unsafe extern "system" fn(target: QueryTarget);

pub(crate) type glEndQueryIndexed_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint);

pub(crate) type glEndTransformFeedback_t = unsafe extern "system" fn();

pub(crate) type glFenceSync_t = unsafe extern "system" fn(condition: SyncCondition, flags: GLbitfield) -> GLsync;

pub(crate) type glFinish_t = unsafe extern "system" fn();

pub(crate) type glFlush_t = unsafe extern "system" fn();

pub(crate) type glFlushMappedBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr);

pub(crate) type glFlushMappedNamedBufferRange_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

pub(crate) type glFramebufferParameteri_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferParameterName, param: GLint);

pub(crate) type glFramebufferRenderbuffer_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

pub(crate) type glFramebufferTexture_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

pub(crate) type glFramebufferTexture1D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

pub(crate) type glFramebufferTexture2D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint);

pub(crate) type glFramebufferTexture3D_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, textarget: TextureTarget, texture: GLuint, level: GLint, zoffset: GLint);

pub(crate) type glFramebufferTextureLayer_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

pub(crate) type glFrontFace_t = unsafe extern "system" fn(mode: FrontFaceDirection);

pub(crate) type glGenBuffers_t = unsafe extern "system" fn(n: GLsizei, buffers: *mut GLuint);

pub(crate) type glGenFramebuffers_t = unsafe extern "system" fn(n: GLsizei, framebuffers: *mut GLuint);

pub(crate) type glGenProgramPipelines_t = unsafe extern "system" fn(n: GLsizei, pipelines: *mut GLuint);

pub(crate) type glGenQueries_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub(crate) type glGenRenderbuffers_t = unsafe extern "system" fn(n: GLsizei, renderbuffers: *mut GLuint);

pub(crate) type glGenSamplers_t = unsafe extern "system" fn(count: GLsizei, samplers: *mut GLuint);

pub(crate) type glGenTextures_t = unsafe extern "system" fn(n: GLsizei, textures: *mut GLuint);

pub(crate) type glGenTransformFeedbacks_t = unsafe extern "system" fn(n: GLsizei, ids: *mut GLuint);

pub(crate) type glGenVertexArrays_t = unsafe extern "system" fn(n: GLsizei, arrays: *mut GLuint);

pub(crate) type glGenerateMipmap_t = unsafe extern "system" fn(target: TextureTarget);

pub(crate) type glGenerateTextureMipmap_t = unsafe extern "system" fn(texture: GLuint);

pub(crate) type glGetActiveAtomicCounterBufferiv_t = unsafe extern "system" fn(program: GLuint, bufferIndex: GLuint, pname: AtomicCounterBufferPName, params: *mut GLint);

pub(crate) type glGetActiveAttrib_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut AttributeType, name: *mut GLchar);

pub(crate) type glGetActiveSubroutineName_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

pub(crate) type glGetActiveSubroutineUniformName_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

pub(crate) type glGetActiveSubroutineUniformiv_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, index: GLuint, pname: SubroutineParameterName, values: *mut GLint);

pub(crate) type glGetActiveUniform_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLint, type_: *mut UniformType, name: *mut GLchar);

pub(crate) type glGetActiveUniformBlockName_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformBlockName: *mut GLchar);

pub(crate) type glGetActiveUniformBlockiv_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, pname: UniformBlockPName, params: *mut GLint);

pub(crate) type glGetActiveUniformName_t = unsafe extern "system" fn(program: GLuint, uniformIndex: GLuint, bufSize: GLsizei, length: *mut GLsizei, uniformName: *mut GLchar);

pub(crate) type glGetActiveUniformsiv_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformIndices: *const GLuint, pname: UniformPName, params: *mut GLint);

pub(crate) type glGetAttachedShaders_t = unsafe extern "system" fn(program: GLuint, maxCount: GLsizei, count: *mut GLsizei, shaders: *mut GLuint);

pub(crate) type glGetAttribLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub(crate) type glGetBooleani_v_t = unsafe extern "system" fn(target: BufferTargetARB, index: GLuint, data: *mut GLboolean);

pub(crate) type glGetBooleanv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLboolean);

pub(crate) type glGetBufferParameteri64v_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint64);

pub(crate) type glGetBufferParameteriv_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPNameARB, params: *mut GLint);

pub(crate) type glGetBufferPointerv_t = unsafe extern "system" fn(target: BufferTargetARB, pname: BufferPointerNameARB, params: *mut *mut void);

pub(crate) type glGetBufferSubData_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, size: GLsizeiptr, data: *mut void);

pub(crate) type glGetCompressedTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, img: *mut void);

pub(crate) type glGetCompressedTextureImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, bufSize: GLsizei, pixels: *mut void);

pub(crate) type glGetCompressedTextureSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, bufSize: GLsizei, pixels: *mut void);

pub(crate) type glGetDebugMessageLog_t = unsafe extern "system" fn(count: GLuint, bufSize: GLsizei, sources: *mut DebugSource, types: *mut DebugType, ids: *mut GLuint, severities: *mut DebugSeverity, lengths: *mut GLsizei, messageLog: *mut GLchar) -> GLuint;

pub(crate) type glGetDoublei_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLdouble);

pub(crate) type glGetDoublev_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLdouble);

pub(crate) type glGetError_t = unsafe extern "system" fn() -> ErrorCode;

pub(crate) type glGetFloati_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLfloat);

pub(crate) type glGetFloatv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLfloat);

pub(crate) type glGetFragDataIndex_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub(crate) type glGetFragDataLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub(crate) type glGetFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(target: FramebufferTarget, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

pub(crate) type glGetFramebufferParameteriv_t = unsafe extern "system" fn(target: FramebufferTarget, pname: FramebufferAttachmentParameterName, params: *mut GLint);

pub(crate) type glGetGraphicsResetStatus_t = unsafe extern "system" fn() -> GraphicsResetStatus;

pub(crate) type glGetImageHandleARB_t = unsafe extern "system" fn(texture: GLuint, level: GLint, layered: GLboolean, layer: GLint, format: PixelFormat) -> GLuint64;

pub(crate) type glGetInteger64i_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint64);

pub(crate) type glGetInteger64v_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint64);

pub(crate) type glGetIntegeri_v_t = unsafe extern "system" fn(target: GetPName, index: GLuint, data: *mut GLint);

pub(crate) type glGetIntegerv_t = unsafe extern "system" fn(pname: GetPName, data: *mut GLint);

pub(crate) type glGetInternalformati64v_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint64);

pub(crate) type glGetInternalformativ_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, pname: InternalFormatPName, count: GLsizei, params: *mut GLint);

pub(crate) type glGetMultisamplefv_t = unsafe extern "system" fn(pname: GetMultisamplePNameNV, index: GLuint, val: *mut GLfloat);

pub(crate) type glGetNamedBufferParameteri64v_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint64);

pub(crate) type glGetNamedBufferParameteriv_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPNameARB, params: *mut GLint);

pub(crate) type glGetNamedBufferPointerv_t = unsafe extern "system" fn(buffer: GLuint, pname: BufferPointerNameARB, params: *mut *mut void);

pub(crate) type glGetNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *mut void);

pub(crate) type glGetNamedFramebufferAttachmentParameteriv_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, pname: FramebufferAttachmentParameterName, params: *mut GLint);

pub(crate) type glGetNamedFramebufferParameteriv_t = unsafe extern "system" fn(framebuffer: GLuint, pname: GetFramebufferParameter, param: *mut GLint);

pub(crate) type glGetNamedRenderbufferParameteriv_t = unsafe extern "system" fn(renderbuffer: GLuint, pname: RenderbufferParameterName, params: *mut GLint);

pub(crate) type glGetObjectLabel_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub(crate) type glGetObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, bufSize: GLsizei, length: *mut GLsizei, label: *mut GLchar);

pub(crate) type glGetPointerv_t = unsafe extern "system" fn(pname: GetPointervPName, params: *mut *mut void);

pub(crate) type glGetProgramBinary_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, binaryFormat: *mut GLenum, binary: *mut void);

pub(crate) type glGetProgramInfoLog_t = unsafe extern "system" fn(program: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub(crate) type glGetProgramInterfaceiv_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, pname: ProgramInterfacePName, params: *mut GLint);

pub(crate) type glGetProgramPipelineInfoLog_t = unsafe extern "system" fn(pipeline: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub(crate) type glGetProgramPipelineiv_t = unsafe extern "system" fn(pipeline: GLuint, pname: PipelineParameterName, params: *mut GLint);

pub(crate) type glGetProgramResourceIndex_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLuint;

pub(crate) type glGetProgramResourceLocation_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;

pub(crate) type glGetProgramResourceLocationIndex_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, name: *const GLchar) -> GLint;

pub(crate) type glGetProgramResourceName_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, name: *mut GLchar);

pub(crate) type glGetProgramResourceiv_t = unsafe extern "system" fn(program: GLuint, programInterface: ProgramInterface, index: GLuint, propCount: GLsizei, props: *const ProgramResourceProperty, count: GLsizei, length: *mut GLsizei, params: *mut GLint);

pub(crate) type glGetProgramStageiv_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, pname: ProgramStagePName, values: *mut GLint);

pub(crate) type glGetProgramiv_t = unsafe extern "system" fn(program: GLuint, pname: ProgramPropertyARB, params: *mut GLint);

pub(crate) type glGetQueryBufferObjecti64v_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

pub(crate) type glGetQueryBufferObjectiv_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

pub(crate) type glGetQueryBufferObjectui64v_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

pub(crate) type glGetQueryBufferObjectuiv_t = unsafe extern "system" fn(id: GLuint, buffer: GLuint, pname: QueryObjectParameterName, offset: GLintptr);

pub(crate) type glGetQueryIndexediv_t = unsafe extern "system" fn(target: QueryTarget, index: GLuint, pname: QueryParameterName, params: *mut GLint);

pub(crate) type glGetQueryObjecti64v_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint64);

pub(crate) type glGetQueryObjectiv_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLint);

pub(crate) type glGetQueryObjectui64v_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint64);

pub(crate) type glGetQueryObjectuiv_t = unsafe extern "system" fn(id: GLuint, pname: QueryObjectParameterName, params: *mut GLuint);

pub(crate) type glGetQueryiv_t = unsafe extern "system" fn(target: QueryTarget, pname: QueryParameterName, params: *mut GLint);

pub(crate) type glGetRenderbufferParameteriv_t = unsafe extern "system" fn(target: RenderbufferTarget, pname: RenderbufferParameterName, params: *mut GLint);

pub(crate) type glGetSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

pub(crate) type glGetSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLuint);

pub(crate) type glGetSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, params: *mut GLfloat);

pub(crate) type glGetSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, params: *mut GLint);

pub(crate) type glGetShaderInfoLog_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, infoLog: *mut GLchar);

pub(crate) type glGetShaderPrecisionFormat_t = unsafe extern "system" fn(shadertype: ShaderType, precisiontype: PrecisionType, range: *mut [GLint; 2], precision: *mut GLint);

pub(crate) type glGetShaderSource_t = unsafe extern "system" fn(shader: GLuint, bufSize: GLsizei, length: *mut GLsizei, source: *mut GLchar);

pub(crate) type glGetShaderiv_t = unsafe extern "system" fn(shader: GLuint, pname: ShaderParameterName, params: *mut GLint);

pub(crate) type glGetString_t = unsafe extern "system" fn(name: StringName) -> *const GLubyte;

pub(crate) type glGetStringi_t = unsafe extern "system" fn(name: StringName, index: GLuint) -> *const GLubyte;

pub(crate) type glGetSubroutineIndex_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLuint;

pub(crate) type glGetSubroutineUniformLocation_t = unsafe extern "system" fn(program: GLuint, shadertype: ShaderType, name: *const GLchar) -> GLint;

pub(crate) type glGetSynciv_t = unsafe extern "system" fn(sync: GLsync, pname: SyncParameterName, count: GLsizei, length: *mut GLsizei, values: *mut GLint);

pub(crate) type glGetTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, pixels: *mut void);

pub(crate) type glGetTexLevelParameterfv_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

pub(crate) type glGetTexLevelParameteriv_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTexParameterIiv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTexParameterIuiv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLuint);

pub(crate) type glGetTexParameterfv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLfloat);

pub(crate) type glGetTexParameteriv_t = unsafe extern "system" fn(target: TextureTarget, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTextureHandleARB_t = unsafe extern "system" fn(texture: GLuint) -> GLuint64;

pub(crate) type glGetTextureImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

pub(crate) type glGetTextureLevelParameterfv_t = unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLfloat);

pub(crate) type glGetTextureLevelParameteriv_t = unsafe extern "system" fn(texture: GLuint, level: GLint, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTextureParameterIiv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTextureParameterIuiv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLuint);

pub(crate) type glGetTextureParameterfv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLfloat);

pub(crate) type glGetTextureParameteriv_t = unsafe extern "system" fn(texture: GLuint, pname: GetTextureParameter, params: *mut GLint);

pub(crate) type glGetTextureSamplerHandleARB_t = unsafe extern "system" fn(texture: GLuint, sampler: GLuint) -> GLuint64;

pub(crate) type glGetTextureSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

pub(crate) type glGetTransformFeedbackVarying_t = unsafe extern "system" fn(program: GLuint, index: GLuint, bufSize: GLsizei, length: *mut GLsizei, size: *mut GLsizei, type_: *mut AttributeType, name: *mut GLchar);

pub(crate) type glGetTransformFeedbacki64_v_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint64);

pub(crate) type glGetTransformFeedbacki_v_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, index: GLuint, param: *mut GLint);

pub(crate) type glGetTransformFeedbackiv_t = unsafe extern "system" fn(xfb: GLuint, pname: TransformFeedbackPName, param: *mut GLint);

pub(crate) type glGetUniformBlockIndex_t = unsafe extern "system" fn(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;

pub(crate) type glGetUniformIndices_t = unsafe extern "system" fn(program: GLuint, uniformCount: GLsizei, uniformNames: *const *const GLchar, uniformIndices: *mut GLuint);

pub(crate) type glGetUniformLocation_t = unsafe extern "system" fn(program: GLuint, name: *const GLchar) -> GLint;

pub(crate) type glGetUniformSubroutineuiv_t = unsafe extern "system" fn(shadertype: ShaderType, location: GLint, params: *mut GLuint);

pub(crate) type glGetUniformdv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLdouble);

pub(crate) type glGetUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLfloat);

pub(crate) type glGetUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLint);

pub(crate) type glGetUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, params: *mut GLuint);

pub(crate) type glGetVertexArrayIndexed64iv_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint64);

pub(crate) type glGetVertexArrayIndexediv_t = unsafe extern "system" fn(vaobj: GLuint, index: GLuint, pname: VertexArrayPName, param: *mut GLint);

pub(crate) type glGetVertexArrayiv_t = unsafe extern "system" fn(vaobj: GLuint, pname: VertexArrayPName, param: *mut GLint);

pub(crate) type glGetVertexAttribIiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLint);

pub(crate) type glGetVertexAttribIuiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint);

pub(crate) type glGetVertexAttribLdv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLdouble);

pub(crate) type glGetVertexAttribLui64vARB_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribEnum, params: *mut GLuint64EXT);

pub(crate) type glGetVertexAttribPointerv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPointerPropertyARB, pointer: *mut *mut void);

pub(crate) type glGetVertexAttribdv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLdouble; 4]);

pub(crate) type glGetVertexAttribfv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLfloat; 4]);

pub(crate) type glGetVertexAttribiv_t = unsafe extern "system" fn(index: GLuint, pname: VertexAttribPropertyARB, params: *mut [GLint; 4]);

pub(crate) type glGetnCompressedTexImage_t = unsafe extern "system" fn(target: TextureTarget, lod: GLint, bufSize: GLsizei, pixels: *mut void);

pub(crate) type glGetnTexImage_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, format: PixelFormat, type_: PixelType, bufSize: GLsizei, pixels: *mut void);

pub(crate) type glGetnUniformdv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLdouble);

pub(crate) type glGetnUniformfv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLfloat);

pub(crate) type glGetnUniformiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLint);

pub(crate) type glGetnUniformuiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, bufSize: GLsizei, params: *mut GLuint);

pub(crate) type glHint_t = unsafe extern "system" fn(target: HintTarget, mode: HintMode);

pub(crate) type glInvalidateBufferData_t = unsafe extern "system" fn(buffer: GLuint);

pub(crate) type glInvalidateBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr);

pub(crate) type glInvalidateFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment);

pub(crate) type glInvalidateNamedFramebufferData_t = unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment);

pub(crate) type glInvalidateNamedFramebufferSubData_t = unsafe extern "system" fn(framebuffer: GLuint, numAttachments: GLsizei, attachments: *const FramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glInvalidateSubFramebuffer_t = unsafe extern "system" fn(target: FramebufferTarget, numAttachments: GLsizei, attachments: *const InvalidateFramebufferAttachment, x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glInvalidateTexImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint);

pub(crate) type glInvalidateTexSubImage_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei);

pub(crate) type glIsBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub(crate) type glIsEnabled_t = unsafe extern "system" fn(cap: EnableCap) -> GLboolean;

pub(crate) type glIsEnabledi_t = unsafe extern "system" fn(target: EnableCap, index: GLuint) -> GLboolean;

pub(crate) type glIsFramebuffer_t = unsafe extern "system" fn(framebuffer: GLuint) -> GLboolean;

pub(crate) type glIsImageHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

pub(crate) type glIsProgram_t = unsafe extern "system" fn(program: GLuint) -> GLboolean;

pub(crate) type glIsProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint) -> GLboolean;

pub(crate) type glIsQuery_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub(crate) type glIsRenderbuffer_t = unsafe extern "system" fn(renderbuffer: GLuint) -> GLboolean;

pub(crate) type glIsSampler_t = unsafe extern "system" fn(sampler: GLuint) -> GLboolean;

pub(crate) type glIsShader_t = unsafe extern "system" fn(shader: GLuint) -> GLboolean;

pub(crate) type glIsSync_t = unsafe extern "system" fn(sync: GLsync) -> GLboolean;

pub(crate) type glIsTexture_t = unsafe extern "system" fn(texture: GLuint) -> GLboolean;

pub(crate) type glIsTextureHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64) -> GLboolean;

pub(crate) type glIsTransformFeedback_t = unsafe extern "system" fn(id: GLuint) -> GLboolean;

pub(crate) type glIsVertexArray_t = unsafe extern "system" fn(array: GLuint) -> GLboolean;

pub(crate) type glLineWidth_t = unsafe extern "system" fn(width: GLfloat);

pub(crate) type glLinkProgram_t = extern "system" fn(program: GLuint);

pub(crate) type glLogicOp_t = unsafe extern "system" fn(opcode: LogicOp);

pub(crate) type glMakeImageHandleNonResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

pub(crate) type glMakeImageHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64, access: GLenum);

pub(crate) type glMakeTextureHandleNonResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

pub(crate) type glMakeTextureHandleResidentARB_t = unsafe extern "system" fn(handle: GLuint64);

pub(crate) type glMapBuffer_t = unsafe extern "system" fn(target: BufferTargetARB, access: BufferAccessARB) -> *mut void;

pub(crate) type glMapBufferRange_t = unsafe extern "system" fn(target: BufferTargetARB, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

pub(crate) type glMapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint, access: BufferAccessARB) -> *mut void;

pub(crate) type glMapNamedBufferRange_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, length: GLsizeiptr, access: GLbitfield) -> *mut void;

pub(crate) type glMemoryBarrier_t = unsafe extern "system" fn(barriers: GLbitfield);

pub(crate) type glMemoryBarrierByRegion_t = unsafe extern "system" fn(barriers: GLbitfield);

pub(crate) type glMinSampleShading_t = unsafe extern "system" fn(value: GLfloat);

pub(crate) type glMultiDrawArrays_t = unsafe extern "system" fn(mode: PrimitiveType, first: *const GLint, count: *const GLsizei, drawcount: GLsizei);

pub(crate) type glMultiDrawArraysIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

pub(crate) type glMultiDrawArraysIndirectCount_t = unsafe extern "system" fn(mode: PrimitiveType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

pub(crate) type glMultiDrawElements_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei);

pub(crate) type glMultiDrawElementsBaseVertex_t = unsafe extern "system" fn(mode: PrimitiveType, count: *const GLsizei, type_: DrawElementsType, indices: *const *const void, drawcount: GLsizei, basevertex: *const GLint);

pub(crate) type glMultiDrawElementsIndirect_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLsizei, stride: GLsizei);

pub(crate) type glMultiDrawElementsIndirectCount_t = unsafe extern "system" fn(mode: PrimitiveType, type_: DrawElementsType, indirect: *const void, drawcount: GLintptr, maxdrawcount: GLsizei, stride: GLsizei);

pub(crate) type glNamedBufferData_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, usage: VertexBufferObjectUsage);

pub(crate) type glNamedBufferStorage_t = unsafe extern "system" fn(buffer: GLuint, size: GLsizeiptr, data: *const void, flags: GLbitfield);

pub(crate) type glNamedBufferSubData_t = unsafe extern "system" fn(buffer: GLuint, offset: GLintptr, size: GLsizeiptr, data: *const void);

pub(crate) type glNamedFramebufferDrawBuffer_t = unsafe extern "system" fn(framebuffer: GLuint, buf: ColorBuffer);

pub(crate) type glNamedFramebufferDrawBuffers_t = unsafe extern "system" fn(framebuffer: GLuint, n: GLsizei, bufs: *const ColorBuffer);

pub(crate) type glNamedFramebufferParameteri_t = unsafe extern "system" fn(framebuffer: GLuint, pname: FramebufferParameterName, param: GLint);

pub(crate) type glNamedFramebufferReadBuffer_t = unsafe extern "system" fn(framebuffer: GLuint, src: ColorBuffer);

pub(crate) type glNamedFramebufferRenderbuffer_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, renderbuffertarget: RenderbufferTarget, renderbuffer: GLuint);

pub(crate) type glNamedFramebufferTexture_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint);

pub(crate) type glNamedFramebufferTextureLayer_t = unsafe extern "system" fn(framebuffer: GLuint, attachment: FramebufferAttachment, texture: GLuint, level: GLint, layer: GLint);

pub(crate) type glNamedRenderbufferStorage_t = unsafe extern "system" fn(renderbuffer: GLuint, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

pub(crate) type glNamedRenderbufferStorageMultisample_t = unsafe extern "system" fn(renderbuffer: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

pub(crate) type glObjectLabel_t = unsafe extern "system" fn(identifier: ObjectIdentifier, name: GLuint, length: GLsizei, label: *const GLchar);

pub(crate) type glObjectPtrLabel_t = unsafe extern "system" fn(ptr: *const void, length: GLsizei, label: *const GLchar);

pub(crate) type glPatchParameterfv_t = unsafe extern "system" fn(pname: PatchParameterName, values: *const GLfloat);

pub(crate) type glPatchParameteri_t = unsafe extern "system" fn(pname: PatchParameterName, value: GLint);

pub(crate) type glPauseTransformFeedback_t = unsafe extern "system" fn();

pub(crate) type glPixelStoref_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLfloat);

pub(crate) type glPixelStorei_t = unsafe extern "system" fn(pname: PixelStoreParameter, param: GLint);

pub(crate) type glPointParameterf_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLfloat);

pub(crate) type glPointParameterfv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLfloat);

pub(crate) type glPointParameteri_t = unsafe extern "system" fn(pname: PointParameterNameARB, param: GLint);

pub(crate) type glPointParameteriv_t = unsafe extern "system" fn(pname: PointParameterNameARB, params: *const GLint);

pub(crate) type glPointSize_t = extern "system" fn(size: GLfloat);

pub(crate) type glPolygonMode_t = unsafe extern "system" fn(face: MaterialFace, mode: PolygonMode);

pub(crate) type glPolygonOffset_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat);

pub(crate) type glPolygonOffsetClamp_t = unsafe extern "system" fn(factor: GLfloat, units: GLfloat, clamp: GLfloat);

pub(crate) type glPopDebugGroup_t = unsafe extern "system" fn();

pub(crate) type glPrimitiveRestartIndex_t = unsafe extern "system" fn(index: GLuint);

pub(crate) type glProgramBinary_t = unsafe extern "system" fn(program: GLuint, binaryFormat: GLenum, binary: *const void, length: GLsizei);

pub(crate) type glProgramParameteri_t = unsafe extern "system" fn(program: GLuint, pname: ProgramParameterPName, value: GLint);

pub(crate) type glProgramUniform1d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble);

pub(crate) type glProgramUniform1dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub(crate) type glProgramUniform1f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat);

pub(crate) type glProgramUniform1fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glProgramUniform1i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint);

pub(crate) type glProgramUniform1iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glProgramUniform1ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint);

pub(crate) type glProgramUniform1uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glProgramUniform2d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble);

pub(crate) type glProgramUniform2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub(crate) type glProgramUniform2f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat);

pub(crate) type glProgramUniform2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glProgramUniform2i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint);

pub(crate) type glProgramUniform2iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glProgramUniform2ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint);

pub(crate) type glProgramUniform2uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glProgramUniform3d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble);

pub(crate) type glProgramUniform3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub(crate) type glProgramUniform3f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

pub(crate) type glProgramUniform3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glProgramUniform3i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint);

pub(crate) type glProgramUniform3iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glProgramUniform3ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

pub(crate) type glProgramUniform3uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glProgramUniform4d_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLdouble, v1: GLdouble, v2: GLdouble, v3: GLdouble);

pub(crate) type glProgramUniform4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLdouble);

pub(crate) type glProgramUniform4f_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

pub(crate) type glProgramUniform4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glProgramUniform4i_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

pub(crate) type glProgramUniform4iv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glProgramUniform4ui_t = unsafe extern "system" fn(program: GLuint, location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

pub(crate) type glProgramUniform4uiv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glProgramUniformHandleui64ARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, value: GLuint64);

pub(crate) type glProgramUniformHandleui64vARB_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, values: *const GLuint64);

pub(crate) type glProgramUniformMatrix2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProgramUniformMatrix2x3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix2x3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProgramUniformMatrix2x4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix2x4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProgramUniformMatrix3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProgramUniformMatrix3x2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix3x2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProgramUniformMatrix3x4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix3x4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProgramUniformMatrix4dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix4fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProgramUniformMatrix4x2dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix4x2fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProgramUniformMatrix4x3dv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glProgramUniformMatrix4x3fv_t = unsafe extern "system" fn(program: GLuint, location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glProvokingVertex_t = unsafe extern "system" fn(mode: VertexProvokingMode);

pub(crate) type glPushDebugGroup_t = unsafe extern "system" fn(source: DebugSource, id: GLuint, length: GLsizei, message: *const GLchar);

pub(crate) type glQueryCounter_t = unsafe extern "system" fn(id: GLuint, target: QueryCounterTarget);

pub(crate) type glReadBuffer_t = unsafe extern "system" fn(src: ReadBufferMode);

pub(crate) type glReadPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *mut void);

pub(crate) type glReadnPixels_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, bufSize: GLsizei, data: *mut void);

pub(crate) type glReleaseShaderCompiler_t = unsafe extern "system" fn();

pub(crate) type glRenderbufferStorage_t = unsafe extern "system" fn(target: RenderbufferTarget, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

pub(crate) type glRenderbufferStorageMultisample_t = unsafe extern "system" fn(target: RenderbufferTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

pub(crate) type glResumeTransformFeedback_t = unsafe extern "system" fn();

pub(crate) type glSampleCoverage_t = unsafe extern "system" fn(value: GLfloat, invert: GLboolean);

pub(crate) type glSampleMaski_t = unsafe extern "system" fn(maskNumber: GLuint, mask: GLbitfield);

pub(crate) type glSamplerParameterIiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

pub(crate) type glSamplerParameterIuiv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLuint);

pub(crate) type glSamplerParameterf_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, param: GLfloat);

pub(crate) type glSamplerParameterfv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterF, param: *const GLfloat);

pub(crate) type glSamplerParameteri_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: GLint);

pub(crate) type glSamplerParameteriv_t = unsafe extern "system" fn(sampler: GLuint, pname: SamplerParameterI, param: *const GLint);

pub(crate) type glScissor_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glScissorArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLint);

pub(crate) type glScissorIndexed_t = unsafe extern "system" fn(index: GLuint, left: GLint, bottom: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glScissorIndexedv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub(crate) type glShaderBinary_t = unsafe extern "system" fn(count: GLsizei, shaders: *const GLuint, binaryFormat: ShaderBinaryFormat, binary: *const void, length: GLsizei);

pub(crate) type glShaderSource_t = unsafe extern "system" fn(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint);

pub(crate) type glShaderStorageBlockBinding_t = unsafe extern "system" fn(program: GLuint, storageBlockIndex: GLuint, storageBlockBinding: GLuint);

pub(crate) type glSpecializeShader_t = unsafe extern "system" fn(shader: GLuint, pEntryPoint: *const GLchar, numSpecializationConstants: GLuint, pConstantIndex: *const GLuint, pConstantValue: *const GLuint);

pub(crate) type glStencilFunc_t = unsafe extern "system" fn(func: StencilFunction, ref_: GLint, mask: GLuint);

pub(crate) type glStencilFuncSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, func: StencilFunction, ref_: GLint, mask: GLuint);

pub(crate) type glStencilMask_t = unsafe extern "system" fn(mask: GLuint);

pub(crate) type glStencilMaskSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, mask: GLuint);

pub(crate) type glStencilOp_t = unsafe extern "system" fn(fail: StencilOp, zfail: StencilOp, zpass: StencilOp);

pub(crate) type glStencilOpSeparate_t = unsafe extern "system" fn(face: StencilFaceDirection, sfail: StencilOp, dpfail: StencilOp, dppass: StencilOp);

pub(crate) type glTexBuffer_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint);

pub(crate) type glTexBufferRange_t = unsafe extern "system" fn(target: TextureTarget, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub(crate) type glTexImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexImage2DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

pub(crate) type glTexImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, border: GLint, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexImage3DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub(crate) type glTexPageCommitmentARB_t = unsafe extern "system" fn(target: GLenum, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, commit: GLboolean);

pub(crate) type glTexParameterIiv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

pub(crate) type glTexParameterIuiv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLuint);

pub(crate) type glTexParameterf_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, param: GLfloat);

pub(crate) type glTexParameterfv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLfloat);

pub(crate) type glTexParameteri_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, param: GLint);

pub(crate) type glTexParameteriv_t = unsafe extern "system" fn(target: TextureTarget, pname: TextureParameterName, params: *const GLint);

pub(crate) type glTexStorage1D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

pub(crate) type glTexStorage2D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

pub(crate) type glTexStorage2DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

pub(crate) type glTexStorage3D_t = unsafe extern "system" fn(target: TextureTarget, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

pub(crate) type glTexStorage3DMultisample_t = unsafe extern "system" fn(target: TextureTarget, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub(crate) type glTexSubImage1D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexSubImage2D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTexSubImage3D_t = unsafe extern "system" fn(target: TextureTarget, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTextureBarrier_t = unsafe extern "system" fn();

pub(crate) type glTextureBuffer_t = unsafe extern "system" fn(texture: GLuint, internalformat: InternalFormat, buffer: GLuint);

pub(crate) type glTextureBufferRange_t = unsafe extern "system" fn(texture: GLuint, internalformat: InternalFormat, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub(crate) type glTextureParameterIiv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, params: *const GLint);

pub(crate) type glTextureParameterIuiv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, params: *const GLuint);

pub(crate) type glTextureParameterf_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: GLfloat);

pub(crate) type glTextureParameterfv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: *const GLfloat);

pub(crate) type glTextureParameteri_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: GLint);

pub(crate) type glTextureParameteriv_t = unsafe extern "system" fn(texture: GLuint, pname: TextureParameterName, param: *const GLint);

pub(crate) type glTextureStorage1D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei);

pub(crate) type glTextureStorage2D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei);

pub(crate) type glTextureStorage2DMultisample_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, fixedsamplelocations: GLboolean);

pub(crate) type glTextureStorage3D_t = unsafe extern "system" fn(texture: GLuint, levels: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei);

pub(crate) type glTextureStorage3DMultisample_t = unsafe extern "system" fn(texture: GLuint, samples: GLsizei, internalformat: InternalFormat, width: GLsizei, height: GLsizei, depth: GLsizei, fixedsamplelocations: GLboolean);

pub(crate) type glTextureSubImage1D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, width: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTextureSubImage2D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, width: GLsizei, height: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTextureSubImage3D_t = unsafe extern "system" fn(texture: GLuint, level: GLint, xoffset: GLint, yoffset: GLint, zoffset: GLint, width: GLsizei, height: GLsizei, depth: GLsizei, format: PixelFormat, type_: PixelType, pixels: *const void);

pub(crate) type glTextureView_t = unsafe extern "system" fn(texture: GLuint, target: TextureTarget, origtexture: GLuint, internalformat: InternalFormat, minlevel: GLuint, numlevels: GLuint, minlayer: GLuint, numlayers: GLuint);

pub(crate) type glTransformFeedbackBufferBase_t = unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint);

pub(crate) type glTransformFeedbackBufferRange_t = unsafe extern "system" fn(xfb: GLuint, index: GLuint, buffer: GLuint, offset: GLintptr, size: GLsizeiptr);

pub(crate) type glTransformFeedbackVaryings_t = unsafe extern "system" fn(program: GLuint, count: GLsizei, varyings: *const *const GLchar, bufferMode: TransformFeedbackBufferMode);

pub(crate) type glUniform1d_t = unsafe extern "system" fn(location: GLint, x: GLdouble);

pub(crate) type glUniform1dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

pub(crate) type glUniform1f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat);

pub(crate) type glUniform1fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glUniform1i_t = unsafe extern "system" fn(location: GLint, v0: GLint);

pub(crate) type glUniform1iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glUniform1ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint);

pub(crate) type glUniform1uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glUniform2d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble);

pub(crate) type glUniform2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

pub(crate) type glUniform2f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat);

pub(crate) type glUniform2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glUniform2i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint);

pub(crate) type glUniform2iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glUniform2ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint);

pub(crate) type glUniform2uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glUniform3d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble);

pub(crate) type glUniform3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

pub(crate) type glUniform3f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);

pub(crate) type glUniform3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glUniform3i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint);

pub(crate) type glUniform3iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glUniform3ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);

pub(crate) type glUniform3uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glUniform4d_t = unsafe extern "system" fn(location: GLint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub(crate) type glUniform4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLdouble);

pub(crate) type glUniform4f_t = unsafe extern "system" fn(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);

pub(crate) type glUniform4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLfloat);

pub(crate) type glUniform4i_t = unsafe extern "system" fn(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);

pub(crate) type glUniform4iv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLint);

pub(crate) type glUniform4ui_t = unsafe extern "system" fn(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);

pub(crate) type glUniform4uiv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint);

pub(crate) type glUniformBlockBinding_t = unsafe extern "system" fn(program: GLuint, uniformBlockIndex: GLuint, uniformBlockBinding: GLuint);

pub(crate) type glUniformHandleui64ARB_t = unsafe extern "system" fn(location: GLint, value: GLuint64);

pub(crate) type glUniformHandleui64vARB_t = unsafe extern "system" fn(location: GLint, count: GLsizei, value: *const GLuint64);

pub(crate) type glUniformMatrix2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix2x3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix2x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix2x4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix2x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix3x2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix3x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix3x4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix3x4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix4dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix4fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix4x2dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix4x2fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformMatrix4x3dv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLdouble);

pub(crate) type glUniformMatrix4x3fv_t = unsafe extern "system" fn(location: GLint, count: GLsizei, transpose: GLboolean, value: *const GLfloat);

pub(crate) type glUniformSubroutinesuiv_t = unsafe extern "system" fn(shadertype: ShaderType, count: GLsizei, indices: *const GLuint);

pub(crate) type glUnmapBuffer_t = unsafe extern "system" fn(target: BufferTargetARB) -> GLboolean;

pub(crate) type glUnmapNamedBuffer_t = unsafe extern "system" fn(buffer: GLuint) -> GLboolean;

pub(crate) type glUseProgram_t = extern "system" fn(program: GLuint);

pub(crate) type glUseProgramStages_t = unsafe extern "system" fn(pipeline: GLuint, stages: GLbitfield, program: GLuint);

pub(crate) type glValidateProgram_t = unsafe extern "system" fn(program: GLuint);

pub(crate) type glValidateProgramPipeline_t = unsafe extern "system" fn(pipeline: GLuint);

pub(crate) type glVertexArrayAttribBinding_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, bindingindex: GLuint);

pub(crate) type glVertexArrayAttribFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);

pub(crate) type glVertexArrayAttribIFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);

pub(crate) type glVertexArrayAttribLFormat_t = unsafe extern "system" fn(vaobj: GLuint, attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);

pub(crate) type glVertexArrayBindingDivisor_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, divisor: GLuint);

pub(crate) type glVertexArrayElementBuffer_t = unsafe extern "system" fn(vaobj: GLuint, buffer: GLuint);

pub(crate) type glVertexArrayVertexBuffer_t = unsafe extern "system" fn(vaobj: GLuint, bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei);

pub(crate) type glVertexArrayVertexBuffers_t = unsafe extern "system" fn(vaobj: GLuint, first: GLuint, count: GLsizei, buffers: *const GLuint, offsets: *const GLintptr, strides: *const GLsizei);

pub(crate) type glVertexAttrib1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

pub(crate) type glVertexAttrib1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

pub(crate) type glVertexAttrib1f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat);

pub(crate) type glVertexAttrib1fv_t = unsafe extern "system" fn(index: GLuint, v: *const GLfloat);

pub(crate) type glVertexAttrib1s_t = unsafe extern "system" fn(index: GLuint, x: GLshort);

pub(crate) type glVertexAttrib1sv_t = unsafe extern "system" fn(index: GLuint, v: *const GLshort);

pub(crate) type glVertexAttrib2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

pub(crate) type glVertexAttrib2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

pub(crate) type glVertexAttrib2f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat);

pub(crate) type glVertexAttrib2fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 2]);

pub(crate) type glVertexAttrib2s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort);

pub(crate) type glVertexAttrib2sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 2]);

pub(crate) type glVertexAttrib3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

pub(crate) type glVertexAttrib3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

pub(crate) type glVertexAttrib3f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);

pub(crate) type glVertexAttrib3fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 3]);

pub(crate) type glVertexAttrib3s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort);

pub(crate) type glVertexAttrib3sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 3]);

pub(crate) type glVertexAttrib4Nbv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub(crate) type glVertexAttrib4Niv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub(crate) type glVertexAttrib4Nsv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub(crate) type glVertexAttrib4Nub_t = unsafe extern "system" fn(index: GLuint, x: GLubyte, y: GLubyte, z: GLubyte, w: GLubyte);

pub(crate) type glVertexAttrib4Nubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub(crate) type glVertexAttrib4Nuiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub(crate) type glVertexAttrib4Nusv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub(crate) type glVertexAttrib4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub(crate) type glVertexAttrib4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub(crate) type glVertexAttrib4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

pub(crate) type glVertexAttrib4f_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);

pub(crate) type glVertexAttrib4fv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub(crate) type glVertexAttrib4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub(crate) type glVertexAttrib4s_t = unsafe extern "system" fn(index: GLuint, x: GLshort, y: GLshort, z: GLshort, w: GLshort);

pub(crate) type glVertexAttrib4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub(crate) type glVertexAttrib4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub(crate) type glVertexAttrib4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub(crate) type glVertexAttrib4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub(crate) type glVertexAttribBinding_t = unsafe extern "system" fn(attribindex: GLuint, bindingindex: GLuint);

pub(crate) type glVertexAttribDivisor_t = unsafe extern "system" fn(index: GLuint, divisor: GLuint);

pub(crate) type glVertexAttribFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribType, normalized: GLboolean, relativeoffset: GLuint);

pub(crate) type glVertexAttribI1i_t = unsafe extern "system" fn(index: GLuint, x: GLint);

pub(crate) type glVertexAttribI1iv_t = unsafe extern "system" fn(index: GLuint, v: *const GLint);

pub(crate) type glVertexAttribI1ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint);

pub(crate) type glVertexAttribI1uiv_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint);

pub(crate) type glVertexAttribI2i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint);

pub(crate) type glVertexAttribI2iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 2]);

pub(crate) type glVertexAttribI2ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint);

pub(crate) type glVertexAttribI2uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 2]);

pub(crate) type glVertexAttribI3i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint);

pub(crate) type glVertexAttribI3iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 3]);

pub(crate) type glVertexAttribI3ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint);

pub(crate) type glVertexAttribI3uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 3]);

pub(crate) type glVertexAttribI4bv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLbyte; 4]);

pub(crate) type glVertexAttribI4i_t = unsafe extern "system" fn(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);

pub(crate) type glVertexAttribI4iv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLint; 4]);

pub(crate) type glVertexAttribI4sv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLshort; 4]);

pub(crate) type glVertexAttribI4ubv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLubyte; 4]);

pub(crate) type glVertexAttribI4ui_t = unsafe extern "system" fn(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);

pub(crate) type glVertexAttribI4uiv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLuint; 4]);

pub(crate) type glVertexAttribI4usv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLushort; 4]);

pub(crate) type glVertexAttribIFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribIType, relativeoffset: GLuint);

pub(crate) type glVertexAttribIPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribIType, stride: GLsizei, pointer: *const void);

pub(crate) type glVertexAttribL1d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble);

pub(crate) type glVertexAttribL1dv_t = unsafe extern "system" fn(index: GLuint, v: *const GLdouble);

pub(crate) type glVertexAttribL1ui64ARB_t = unsafe extern "system" fn(index: GLuint, x: GLuint64EXT);

pub(crate) type glVertexAttribL1ui64vARB_t = unsafe extern "system" fn(index: GLuint, v: *const GLuint64EXT);

pub(crate) type glVertexAttribL2d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble);

pub(crate) type glVertexAttribL2dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 2]);

pub(crate) type glVertexAttribL3d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble);

pub(crate) type glVertexAttribL3dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 3]);

pub(crate) type glVertexAttribL4d_t = unsafe extern "system" fn(index: GLuint, x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble);

pub(crate) type glVertexAttribL4dv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLdouble; 4]);

pub(crate) type glVertexAttribLFormat_t = unsafe extern "system" fn(attribindex: GLuint, size: GLint, type_: VertexAttribLType, relativeoffset: GLuint);

pub(crate) type glVertexAttribLPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribLType, stride: GLsizei, pointer: *const void);

pub(crate) type glVertexAttribP1ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

pub(crate) type glVertexAttribP1uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

pub(crate) type glVertexAttribP2ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

pub(crate) type glVertexAttribP2uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

pub(crate) type glVertexAttribP3ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

pub(crate) type glVertexAttribP3uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

pub(crate) type glVertexAttribP4ui_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: GLuint);

pub(crate) type glVertexAttribP4uiv_t = unsafe extern "system" fn(index: GLuint, type_: VertexAttribPointerType, normalized: GLboolean, value: *const GLuint);

pub(crate) type glVertexAttribPointer_t = unsafe extern "system" fn(index: GLuint, size: GLint, type_: VertexAttribPointerType, normalized: GLboolean, stride: GLsizei, pointer: *const void);

pub(crate) type glVertexBindingDivisor_t = unsafe extern "system" fn(bindingindex: GLuint, divisor: GLuint);

pub(crate) type glViewport_t = unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei);

pub(crate) type glViewportArrayv_t = unsafe extern "system" fn(first: GLuint, count: GLsizei, v: *const GLfloat);

pub(crate) type glViewportIndexedf_t = unsafe extern "system" fn(index: GLuint, x: GLfloat, y: GLfloat, w: GLfloat, h: GLfloat);

pub(crate) type glViewportIndexedfv_t = unsafe extern "system" fn(index: GLuint, v: *const [GLfloat; 4]);

pub(crate) type glWaitSync_t = unsafe extern "system" fn(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
