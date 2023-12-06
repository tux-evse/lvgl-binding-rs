/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
*/
extern crate bindgen;
use std::env;

fn main() {
    // invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=capi/capi-map.c");

    let header = "
    // -----------------------------------------------------------------------
    //         <- private '_capi_map.rs' Rust/C unsafe binding ->
    // -----------------------------------------------------------------------
    //   Do not exit this file it will be regenerated automatically by cargo.
    //   Check:
    //     - build.rs for C/Rust glue options
    //     - src/capi/capi-map.c for C prototype inputs
    // -----------------------------------------------------------------------
    ";

    let use_gtk= match env::var("USE_GTK") {
        Ok(_value) => {
            println! ("cargo:warning=GTK driver backend selected");
            println! ("cargo:rustc-cfg=use_gtk");
            "1".to_owned()
        },
        Err(_) => "0".to_owned(),
    };

    let gtk_define= format!("-DUSE_GTK={}", use_gtk);

    let _capi_map = bindgen::Builder::default()
        .header("capi/capi-map.c")
        .clang_arg("-F/usr/local/include/lv_drivers")
        .clang_arg("-F/usr/local/include/lvgl")
        .clang_arg(gtk_define)
        .raw_line(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_debug(false)
        .layout_tests(false)
        .allowlist_item("gtkdrv_.*")
        .allowlist_item("evdev_.*")
        .allowlist_item("fbdev_.*")
        .allowlist_var("LV_.*")
        .allowlist_item("lv_.*")
        .allowlist_item("line_.*")
        .generate()
        .expect("Unable to generate _capi-map.rs");

    _capi_map
        .write_to_file("capi/_capi-map.rs")
        .expect("Couldn't write _capi-map.rs!");

    cc::Build::new()
        .file("capi/capi-map.c")
        .define("USE_GTK", use_gtk.as_str())
        .include("/usr/local/include")
        .include("/usr/local/include/lvgl/lv_drivers")
        .compile("lvgl-glue");
}
