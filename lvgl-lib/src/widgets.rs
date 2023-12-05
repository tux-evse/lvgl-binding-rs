/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::impl_widget_trait;
use std::cell::Cell;
use std::ffi::CString;
use std::mem;
use std::os::raw;

use crate::prelude::*;

pub enum LvglWidget {
    ImgButton(&'static LvglImgButton),
    Label(&'static LvglLabel),
    Button(&'static LvglButton),
    Icon(&'static LvglIcon),
    TextArea(&'static LvglTextArea),
    Led(&'static LvglLed),
    Line(&'static LvglLine),
    Image(&'static LvglImage),
    Arc(&'static LvglArc),
    Meter(&'static LvglMeter),
}

pub trait LvglHandler {
    fn callback(&self, uid: &'static str, code: u32);
}

// has we share C widget callback, we have to retrieve initial object for callback
impl LvglWidget {
    pub(crate) fn callback(&self, event: &cglue::lv_event_t) {
        match self {
            LvglWidget::Label(this) => this.callback(event),
            LvglWidget::Button(this) => this.callback(event),
            LvglWidget::ImgButton(this) => this.callback(event),
            LvglWidget::Icon(this) => this.callback(event),
            LvglWidget::Image(this) => this.callback(event),
            _ => {}
        }
    }
    pub fn set_callback(&self, ctrlbox: Option<*mut dyn LvglHandler>) {
        match self {
            LvglWidget::Label(this) => this.set_callback(ctrlbox),
            LvglWidget::Button(this) => this.set_callback(ctrlbox),
            LvglWidget::ImgButton(this) => this.set_callback(ctrlbox),
            LvglWidget::Icon(this) => this.set_callback(ctrlbox),
            _ => {}
        }
    }
    pub fn get_uid(&self) -> &'static str {
        match self {
            LvglWidget::Label(this) => this.get_uid(),
            LvglWidget::Button(this) => this.get_uid(),
            LvglWidget::ImgButton(this) => this.get_uid(),
            LvglWidget::Icon(this) => this.get_uid(),
            LvglWidget::TextArea(this) => this.get_uid(),
            LvglWidget::Led(this) => this.get_uid(),
            LvglWidget::Line(this) => this.get_uid(),
            LvglWidget::Image(this) => this.get_uid(),
            LvglWidget::Arc(this) => this.get_uid(),
            LvglWidget::Meter(this) => this.get_uid(),
        }
    }
    pub fn get_info(&self) -> &'static str {
        match self {
            LvglWidget::Label(this) => this.get_info(),
            LvglWidget::Button(this) => this.get_info(),
            LvglWidget::ImgButton(this) => this.get_info(),
            LvglWidget::Icon(this) => this.get_info(),
            LvglWidget::TextArea(this) => this.get_info(),
            LvglWidget::Led(this) => this.get_info(),
            LvglWidget::Line(this) => this.get_info(),
            LvglWidget::Image(this) => this.get_info(),
            LvglWidget::Arc(this) => this.get_info(),
            LvglWidget::Meter(this) => this.get_info(),
        }
    }
    pub fn set_text(&self, text: &str) {
        match self {
            LvglWidget::Label(this) => {
                this.set_text(text);
            }
            LvglWidget::TextArea(this) => {
                this.set_text(text);
            }
            _ => {}
        }
    }
}

pub struct LvglButton {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    label: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}

impl_widget_trait!(LvglButton, Button);
impl LvglButton {
    pub fn new(uid: &'static str, label: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let text = match CString::new(label) {
                Err(_) => CString::new("Non UTF8 label").unwrap(),
                Ok(value) => value,
            };

            let handle = cglue::lv_btn_create(cglue::lv_scr_action());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            // create a label as children on button
            let label = cglue::lv_label_create(handle);
            cglue::lv_label_set_text(label, text.as_ptr());
            cglue::lv_obj_set_align(label, cglue::LV_ALIGN_CENTER as u8);
            cglue::lv_obj_set_pos(label, 0, 0);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            let widget = LvglButton {
                uid,
                info: Cell::new(""),
                handle,
                style,
                label,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_circular(&self) -> &Self {
        unsafe {
            cglue::lv_label_set_long_mode(self.handle, cglue::LV_LABEL_LONG_SCROLL_CIRCULAR as u8);
        }
        self
    }

    pub fn set_text(&self, label: &str) -> &Self
    where
        Self: LvglCommon,
    {
        unsafe {
            let text = match CString::new(label) {
                Err(_) => CString::new("Non UTF8 label").unwrap(),
                Ok(value) => value,
            };
            cglue::lv_label_set_text(self.label, text.as_ptr());
        }
        self
    }

    pub fn callback(&self, event: &cglue::lv_event_t) {
        if let Some(ctrlbox) = self.ctrlbox.get() {
            match event.code {
                cglue::lv_event_code_t_LV_EVENT_PRESSED => {}
                cglue::lv_event_code_t_LV_EVENT_CLICKED => {}
                _ => return, // ignore other event
            }
            println!("LvglButton  uid:{} code:{}", self.uid, event.code);
            unsafe { (*ctrlbox).callback(self.uid, event.code) };
        }
    }
}

pub struct LvglImgButton {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    _style_pr: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}

impl_widget_trait!(LvglImgButton, ImgButton);
impl LvglImgButton {
    pub fn new(
        uid: &'static str,
        label: &'static str,
        x_ofs: i16,
        y_ofs: i16,
        color: LvglColor,
        time: u32,
        delay: u32,
    ) -> &'static Self {
        unsafe {
            const TR_PRO: [cglue::lv_style_prop_t; 3] = [
                cglue::lv_style_prop_t_LV_STYLE_TRANSFORM_WIDTH,
                cglue::lv_style_prop_t_LV_STYLE_IMG_RECOLOR_OPA,
                0,
            ];

            let tr_sty = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_transition_dsc_t>()));
            cglue::lv_style_transition_dsc_init(
                tr_sty,
                &TR_PRO as *const u32,
                Some(cglue::lv_anim_path_linear),
                time,
                delay,
                0 as *mut raw::c_void,
            );

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_style_set_transition(style, tr_sty);

            // Darken the button when pressed and make it wider
            let style_pr = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style_pr);
            cglue::lv_style_set_img_recolor_opa(style_pr, cglue::LV_OPA_30 as u8);
            cglue::lv_style_set_img_recolor(style_pr, color.handle);
            cglue::lv_style_set_transform_width(style_pr, 20);

            // Create an image button
            let handle = cglue::lv_imgbtn_create(cglue::lv_scr_action());
            cglue::lv_imgbtn_set_src(
                handle,
                cglue::lv_imgbtn_state_t_LV_IMGBTN_STATE_RELEASED,
                &cglue::lv_button_left as *const _ as *const raw::c_void,
                &cglue::lv_button_mid as *const _ as *const raw::c_void,
                &cglue::lv_button_right as *const _ as *const raw::c_void,
            );
            cglue::lv_obj_add_style(handle, style, cglue::LV_STATE_DEFAULT);
            cglue::lv_obj_add_style(handle, style_pr, cglue::LV_STATE_PRESSED);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            //Create a label on the img_btn button
            let text = match CString::new(label) {
                Err(_) => CString::new("Non UTF8 label").unwrap(),
                Ok(value) => value,
            };

            let label_btn = cglue::lv_label_create(handle);
            cglue::lv_label_set_text(label_btn, text.as_ptr());
            cglue::lv_obj_align(label_btn, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let widget = LvglImgButton {
                uid,
                info: Cell::new(""),
                handle,
                style,
                _style_pr: style_pr,
                ctrlbox: Cell::new(None),
            };

            Box::leak(Box::new(widget))
        }
    }

    pub fn callback(&self, event: &cglue::lv_event_t) {
        match event.code {
            cglue::lv_event_code_t_LV_EVENT_DRAW_MAIN_BEGIN => {}
            cglue::lv_event_code_t_LV_EVENT_PRESSED => {}
            cglue::lv_event_code_t_LV_EVENT_CLICKED => {}
            _ => return, // ignore other event
        }
        println!("LvglImgButton uid:{} code:{}", self.uid, event.code);
    }
}

pub struct LvglLabel {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglLabel, Label);
impl LvglLabel {
    pub fn new(uid: &'static str, label: &str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_label_create(cglue::lv_scr_action());
            cglue::lv_label_set_recolor(handle, false);
            cglue::lv_obj_set_style_text_align(handle, cglue::LV_TEXT_ALIGN_CENTER as u8, 0);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);

            // create widget object and set label text
            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            let widget = LvglLabel {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            widget.set_text(label);
            Box::leak(Box::new(widget))
        }
    }
    pub fn callback(&self, event: &cglue::lv_event_t) {
        match event.code {
            //cglue::lv_event_code_t_LV_EVENT_DRAW_MAIN_BEGIN => {}
            cglue::lv_event_code_t_LV_EVENT_DRAW_POST_END => {}
            cglue::lv_event_code_t_LV_EVENT_PRESSED => {}
            cglue::lv_event_code_t_LV_EVENT_CLICKED => {}
            _ => return, // ignore other events
        }
        println!("LvgLabel  uid:{} code:{}", self.uid, event.code);
    }
}

pub struct LvglIcon {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglIcon, Icon);
impl LvglIcon {
    pub fn new(uid: &'static str, pixmap: &[u8], x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_img_create(cglue::lv_scr_action());
            cglue::lv_img_set_src(handle, pixmap as *const _ as *mut raw::c_void);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);

            // create widget object and set icon text
            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            let widget = LvglIcon {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }
    pub fn callback(&self, event: &cglue::lv_event_t) {
        match event.code {
            cglue::lv_event_code_t_LV_EVENT_PRESSED => {}
            cglue::lv_event_code_t_LV_EVENT_CLICKED => {}
            _ => return, // ignore other events
        }
        println!("LvgIcon  uid:{} code:{}", self.uid, event.code);
    }
}

pub struct LvglImage {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglImage, Image);
impl LvglImage {
    pub fn new(uid: &'static str, path: &str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        let filepath = match CString::new(path) {
            Err(_) => CString::new("Non UTF8 path").unwrap(),
            Ok(value) => value,
        };

        unsafe {
            let handle = cglue::lv_img_create(cglue::lv_scr_action());
            cglue::lv_img_set_src(handle, filepath.as_ptr() as *mut raw::c_void);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);

            // create widget object and set image text
            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            let widget = LvglImage {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }
    pub fn callback(&self, event: &cglue::lv_event_t) {
        match event.code {
            cglue::lv_event_code_t_LV_EVENT_PRESSED => {}
            cglue::lv_event_code_t_LV_EVENT_CLICKED => {}
            _ => return, // ignore other events
        }
        println!("LvgImage  uid:{} code:{}", self.uid, event.code);
    }
}

pub struct LvglTextArea {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglTextArea, TextArea);
impl LvglTextArea {
    pub fn new(uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_textarea_create(cglue::lv_scr_action());
            cglue::lv_obj_set_style_text_align(handle, cglue::LV_TEXT_ALIGN_CENTER as u8, 0);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            cglue::lv_textarea_set_one_line(handle, true);
            cglue::lv_textarea_set_align(handle, cglue::LV_TEXT_ALIGN_LEFT as u8);
            cglue::lv_obj_set_style_text_opa(
                handle,
                cglue::LV_PART_MAIN as u8,
                cglue::LV_STATE_FOCUSED,
            );

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);

            // create widget object and set text text
            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            let widget = LvglTextArea {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_text(&self, text: &str) -> &Self {
        unsafe {
            let text = match CString::new(text) {
                Err(_) => CString::new("Non UTF8 text").unwrap(),
                Ok(value) => value,
            };
            cglue::lv_textarea_set_text(self.handle, text.as_ptr());
        }
        self
    }

    pub fn insert_text(&self, text: &str) -> &Self {
        unsafe {
            let text = match CString::new(text) {
                Err(_) => CString::new("Non UTF8 text").unwrap(),
                Ok(value) => value,
            };
            cglue::lv_textarea_set_text(self.handle, text.as_ptr());
        }
        self
    }
}

pub struct LvglLed {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglLed, Led);
impl LvglLed {
    pub fn new(uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_led_create(cglue::lv_scr_action());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            cglue::lv_led_off(handle);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);

            // create widget object and set text text
            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            let widget = LvglLed {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_color(&self, color: LvglColor) -> &Self {
        unsafe { cglue::lv_led_set_color(self.handle, color.handle) };
        self
    }

    pub fn set_brightness(&self, bright: u8) -> &Self {
        unsafe { cglue::lv_led_set_brightness(self.handle, bright) };
        self
    }

    pub fn set_on(&self, status: bool) -> &Self {
        unsafe {
            if status {
                cglue::lv_led_on(self.handle);
            } else {
                cglue::lv_led_off(self.handle);
            }
        }
        self
    }
}

pub struct LvglLine {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglLine, Line);
impl LvglLine {
    pub fn new(uid: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
        unsafe {
            let handle = cglue::lv_line_create(cglue::lv_scr_action());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglLine {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_points(&self, points: Box<[LvglPoint]>) -> &Self {
        let list = Box::leak(points);
        let count = list.len();
        unsafe {
            cglue::lv_line_set_points(
                self.handle,
                list.as_ptr() as *const cglue::lv_point_t,
                count as u16,
            );
        }
        self
    }

    pub fn set_width(&self, width: i16) -> &Self {
        unsafe {
            cglue::lv_style_set_line_width(self.style, width);
        }
        self
    }

    pub fn set_color(&self, color: LvglColor) -> &Self {
        unsafe {
            cglue::lv_style_set_line_color(self.style, color.handle);
        };
        self
    }

    pub fn set_rounded(&self, value: bool) -> &Self {
        unsafe {
            cglue::lv_style_set_line_rounded(self.style, value);
        };
        self
    }
}

pub struct LvglArc {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglArc, Arc);
impl LvglArc {
    pub fn new(
        uid: &'static str,
        angle_start: u16,
        angle_end: u16,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        unsafe {
            let handle = cglue::lv_arc_create(cglue::lv_scr_action());
            cglue::lv_arc_set_bg_angles(handle, angle_start, angle_end);
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);
            cglue::lv_obj_clear_flag(handle, cglue::LV_OBJ_FLAG_CLICKABLE);

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglArc {
                uid,
                info: Cell::new(""),
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_rotation(&self, angle: u16) -> &Self {
        unsafe {
            cglue::lv_arc_set_rotation(self.handle, angle);
        }
        self
    }

    pub fn set_range(&self, min: i16, max: i16) -> &Self {
        unsafe {
            cglue::lv_arc_set_range(self.handle, min, max);
        }
        self
    }

    pub fn set_value(&self, value: i16) -> &Self {
        unsafe {
            cglue::lv_arc_set_value(self.handle, value);
        }
        self
    }

    pub fn remove_knob(&self) -> &Self {
        unsafe {
            cglue::lv_obj_remove_style(
                self.handle,
                0 as *mut cglue::lv_style_t,
                cglue::LV_PART_KNOB,
            );
        }
        self
    }

    pub fn set_width(&self, width: i16) -> &Self {
        unsafe {
            cglue::lv_style_set_arc_width(self.style, width);
        }
        self
    }

    pub fn set_color(&self, color: LvglColor) -> &Self {
        unsafe {
            cglue::lv_style_set_arc_color(self.style, color.handle);
        };
        self
    }

    pub fn set_rounded(&self, value: bool) -> &Self {
        unsafe {
            cglue::lv_style_set_arc_rounded(self.style, value);
        };
        self
    }
}

pub struct LvglMeter {
    uid: &'static str,
    info: Cell<&'static str>,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    scale: *mut cglue::lv_meter_scale_t,
    needle: *mut cglue::lv_meter_indicator_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglMeter, Meter);
impl LvglMeter {
    pub fn new(
        uid: &'static str,
        needle_width: u16,
        needle_ratio: i16,
        needle_color: LvglColor,
        x_ofs: i16,
        y_ofs: i16,
    ) -> &'static Self {
        unsafe {
            let handle = cglue::lv_meter_create(cglue::lv_scr_action());
            cglue::lv_obj_align(handle, cglue::LV_ALIGN_TOP_LEFT as u8, x_ofs, y_ofs);

            // add scale
            let scale = cglue::lv_meter_add_scale(handle);

            // add needel
            let needle = cglue::lv_meter_add_needle_line(
                handle,
                scale,
                needle_width,
                needle_color.handle,
                needle_ratio,
            );

            let style = Box::leak(Box::new(mem::zeroed::<cglue::lv_style_t>()));
            cglue::lv_style_init(style);
            cglue::lv_obj_add_style(handle, style, 0);

            let widget = LvglMeter {
                uid,
                info: Cell::new(""),
                handle,
                scale,
                needle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_tic(
        &self,
        line_width: u16,
        label_gap: i16, // gap text to tick
        tick_count: u16,
        tick_length: u16,
        nth_major: u16, // number of tick to major
        minor_color: LvglColor,
        major_color: LvglColor,
    ) -> &Self {
        unsafe {
            cglue::lv_meter_set_scale_ticks(
                self.handle,
                self.scale,
                tick_count,
                line_width,
                tick_length,
                minor_color.handle,
            );
            cglue::lv_meter_set_scale_major_ticks(
                self.handle,
                self.scale,
                nth_major,
                (line_width as f32 * 1.5) as u16,
                (tick_length as f32 * 1.5) as u16,
                major_color.handle,
                label_gap,
            );
        }
        self
    }

    pub fn set_zone(&self, start: i32, end: i32, width: u16, color: LvglColor) -> &Self {
        unsafe {
            let indic = cglue::lv_meter_add_arc(self.handle, self.scale, width, color.handle, 0);
            cglue::lv_meter_set_indicator_start_value(self.handle, indic, start);
            cglue::lv_meter_set_indicator_end_value(self.handle, indic, end);
            let indic = cglue::lv_meter_add_scale_lines(
                self.handle,
                self.scale,
                color.handle,
                color.handle,
                false,
                0,
            );
            cglue::lv_meter_set_indicator_start_value(self.handle, indic, start);
            cglue::lv_meter_set_indicator_end_value(self.handle, indic, end);
        }
        self
    }

    pub fn set_value(&self, value: i32) -> &Self {
        unsafe {
            cglue::lv_meter_set_indicator_value(self.handle, self.needle, value);
        }
        self
    }
}
