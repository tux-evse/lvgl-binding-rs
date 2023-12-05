/*
 * Copyright (C) 2015-2023 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Redpesk interface code/config use MIT License and can be freely copy/modified even within proprietary code
 * License: $RP_BEGIN_LICENSE$ SPDX:MIT https://opensource.org/licenses/MIT $RP_END_LICENSE$
 *
*/
extern crate bindgen;

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

    let _capi_map
     = bindgen::Builder::default()
        .header("capi/capi-map.c")
        .clang_arg("-F/usr/local/include/lv_drivers")
        .clang_arg("-F/usr/local/include/lvgl")
        .raw_line(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_debug(false)
        .layout_tests(false)
        .allowlist_var("LV_.*")
        .allowlist_item("lv_init")
        .allowlist_item("lv_indev_.*")
        .allowlist_item("evdev_.*")
        .allowlist_item("lv_mouse_.*")
        .allowlist_item("fbdev_.*")
        .allowlist_item("lv_disp_.*")
        .allowlist_item("lv_obj_.*")
        .allowlist_item("lv_timer_.*")
        .allowlist_item("lv_tick_.*")
        .allowlist_item("lv_event_.*")
        .allowlist_item("lv_style_.*")
        .allowlist_item("lv_theme_.*")
        .allowlist_item("lv_anim_.*")
        .allowlist_item("lv_palette_.*")
        .allowlist_item("lv_color_.*")
        .allowlist_item("lv_font_.*")
        .allowlist_item("lv_scr_.*")
        .allowlist_item("lv_label_.*")
        .allowlist_item("lv_btn_.*")
        .allowlist_item("lv_button_.*")
        .allowlist_item("lv_img_.*")
        .allowlist_item("lv_textarea_.*")
        .allowlist_item("lv_imgbtn_.*")
        .allowlist_item("lv_led_.*")
        .allowlist_item("lv_line_.*")
        .allowlist_item("line_.*")
        .allowlist_item("lv_arc_.*")
        .allowlist_item("lv_meter_.*")
        .allowlist_item("lv_switch_.*")
        .allowlist_item("lv_bar_.*")


        .generate()
        .expect("Unable to generate _capi-map.rs");

    _capi_map
        .write_to_file("capi/_capi-map.rs")
        .expect("Couldn't write _capi-map.rs!");

    cc::Build::new()
         .file("capi/capi-map.c")
         .include("/usr/local/include")
         .include("/usr/local/include/lvgl/lv_drivers")
         .compile("lvgl-glue");
}
