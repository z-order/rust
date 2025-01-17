//! The `wasm32-wasi-preview2` target is the next evolution of the
//! wasm32-wasi target. While the wasi specification is still under
//! active development, the {review 2 iteration is considered an "island
//! of stability" that should allow users to rely on it indefinitely.
//!
//! The `wasi` target is a proposal to define a standardized set of WebAssembly
//! component imports that allow it to interoperate with the host system in a
//! standardized way. This set of imports is intended to empower WebAssembly
//! binaries with host capabilities such as filesystem access, network access, etc.
//!
//! Wasi Preview 2 relies on the WebAssembly component model which is an extension of
//! the core WebAssembly specification which allows interoperability between WebAssembly
//! modules (known as "components") through high-level, shared-nothing APIs instead of the
//! low-level, shared-everything linear memory model of the core WebAssembly specification.
//!
//! You can see more about wasi at <https://wasi.dev> and the component model at
//! <https://github.com/WebAssembly/component-model>.

use crate::spec::crt_objects;
use crate::spec::LinkSelfContainedDefault;
use crate::spec::{base, Target};

pub fn target() -> Target {
    let mut options = base::wasm::options();

    options.os = "wasi".into();
    options.env = "preview2".into();
    options.linker = Some("wasm-component-ld".into());

    options.pre_link_objects_self_contained = crt_objects::pre_wasi_self_contained();
    options.post_link_objects_self_contained = crt_objects::post_wasi_self_contained();

    // FIXME: Figure out cases in which WASM needs to link with a native toolchain.
    options.link_self_contained = LinkSelfContainedDefault::True;

    // Right now this is a bit of a workaround but we're currently saying that
    // the target by default has a static crt which we're taking as a signal
    // for "use the bundled crt". If that's turned off then the system's crt
    // will be used, but this means that default usage of this target doesn't
    // need an external compiler but it's still interoperable with an external
    // compiler if configured correctly.
    options.crt_static_default = true;
    options.crt_static_respected = true;

    // Allow `+crt-static` to create a "cdylib" output which is just a wasm file
    // without a main function.
    options.crt_static_allows_dylibs = true;

    // WASI's `sys::args::init` function ignores its arguments; instead,
    // `args::args()` makes the WASI API calls itself.
    options.main_needs_argc_argv = false;

    // And, WASI mangles the name of "main" to distinguish between different
    // signatures.
    options.entry_name = "__main_void".into();

    Target {
        llvm_target: "wasm32-unknown-unknown".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-p10:8:8-p20:8:8-i64:64-n32:64-S128-ni:1:10:20".into(),
        arch: "wasm32".into(),
        options,
    }
}
