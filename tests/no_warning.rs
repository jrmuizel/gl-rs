#![feature(phase)]

//! Making sure that no warning is generated by code from generate_gl_bindings!
#![deny(warnings)]

#[phase(plugin)]
extern crate gl_generator;

extern crate libc;

mod gl_global {
    generate_gl_bindings! {
        api: "gl",
        profile: "core",
        version: "4.5",
        generator: "global",
    }
}

mod gl_static {
    generate_gl_bindings! {
        api: "gl",
        profile: "core",
        version: "4.5",
        generator: "static",
    }
}

mod gl_struct {
    generate_gl_bindings! {
        api: "gl",
        profile: "core",
        version: "4.5",
        generator: "struct",
    }
}

mod gl_static_struct {
    generate_gl_bindings! {
        api: "gl",
        profile: "core",
        version: "4.5",
        generator: "static_struct",
    }
}

mod glx_global {
    generate_gl_bindings! {
        api: "glx",
        profile: "core",
        version: "1.4",
        generator: "global",
    }
}

mod glx_static {
    generate_gl_bindings! {
        api: "glx",
        profile: "core",
        version: "1.4",
        generator: "static",
    }
}

mod glx_struct {
    generate_gl_bindings! {
        api: "glx",
        profile: "core",
        version: "1.4",
        generator: "struct",
    }
}

mod wgl_global {
    generate_gl_bindings! {
        api: "wgl",
        profile: "core",
        version: "1.0",
        generator: "global",
    }
}

mod wgl_static {
    generate_gl_bindings! {
        api: "wgl",
        profile: "core",
        version: "1.0",
        generator: "static",
    }
}

mod wgl_struct {
    generate_gl_bindings! {
        api: "wgl",
        profile: "core",
        version: "1.0",
        generator: "struct",
    }
}

mod egl_global {
    use libc;

    #[allow(non_camel_case_types)]
    pub type khronos_utime_nanoseconds_t = libc::c_int;
    #[allow(non_camel_case_types)]
    pub type khronos_uint64_t = libc::uint64_t;
    #[allow(non_camel_case_types)]
    pub type khronos_ssize_t = libc::ssize_t;
    pub type EGLNativeDisplayType = *const libc::c_void;
    pub type EGLNativePixmapType = *const libc::c_void;
    pub type EGLNativeWindowType = *const libc::c_void;
    pub type EGLint = libc::c_int;
    pub type NativeDisplayType = *const libc::c_void;
    pub type NativePixmapType = *const libc::c_void;
    pub type NativeWindowType = *const libc::c_void;

    generate_gl_bindings! {
        api: "egl",
        profile: "core",
        version: "1.5",
        generator: "global",
    }
}

mod egl_static {
    use libc;

    #[allow(non_camel_case_types)]
    pub type khronos_utime_nanoseconds_t = libc::c_int;
    #[allow(non_camel_case_types)]
    pub type khronos_uint64_t = libc::uint64_t;
    #[allow(non_camel_case_types)]
    pub type khronos_ssize_t = libc::ssize_t;
    pub type EGLNativeDisplayType = *const libc::c_void;
    pub type EGLNativePixmapType = *const libc::c_void;
    pub type EGLNativeWindowType = *const libc::c_void;
    pub type EGLint = libc::c_int;
    pub type NativeDisplayType = *const libc::c_void;
    pub type NativePixmapType = *const libc::c_void;
    pub type NativeWindowType = *const libc::c_void;

    generate_gl_bindings! {
        api: "egl",
        profile: "core",
        version: "1.5",
        generator: "static",
    }
}

mod egl_struct {
    use libc;

    #[allow(non_camel_case_types)]
    pub type khronos_utime_nanoseconds_t = libc::c_int;
    #[allow(non_camel_case_types)]
    pub type khronos_uint64_t = libc::uint64_t;
    #[allow(non_camel_case_types)]
    pub type khronos_ssize_t = libc::ssize_t;
    pub type EGLNativeDisplayType = *const libc::c_void;
    pub type EGLNativePixmapType = *const libc::c_void;
    pub type EGLNativeWindowType = *const libc::c_void;
    pub type EGLint = libc::c_int;
    pub type NativeDisplayType = *const libc::c_void;
    pub type NativePixmapType = *const libc::c_void;
    pub type NativeWindowType = *const libc::c_void;

    generate_gl_bindings! {
        api: "egl",
        profile: "core",
        version: "1.5",
        generator: "struct",
    }
}

mod gles1_global {
    generate_gl_bindings! {
        api: "gles1",
        profile: "core",
        version: "1.1",
        generator: "global",
    }
}

mod gles1_static {
    generate_gl_bindings! {
        api: "gles1",
        profile: "core",
        version: "1.1",
        generator: "static",
    }
}

mod gles1_struct {
    generate_gl_bindings! {
        api: "gles1",
        profile: "core",
        version: "1.1",
        generator: "struct",
    }
}

mod gles2_global {
    generate_gl_bindings! {
        api: "gles2",
        profile: "core",
        version: "3.1",
        generator: "global",
    }
}

mod gles2_static {
    generate_gl_bindings! {
        api: "gles2",
        profile: "core",
        version: "3.1",
        generator: "static",
    }
}

mod gles2_struct {
    generate_gl_bindings! {
        api: "gles2",
        profile: "core",
        version: "3.1",
        generator: "struct",
    }
}
