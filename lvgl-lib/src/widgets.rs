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
        }
    }
    pub fn get_info(&self) -> &'static str {
        match self {
            LvglWidget::Label(this) => this.get_info(),
            LvglWidget::Button(this) => this.get_info(),
            LvglWidget::ImgButton(this) => this.get_info(),
            LvglWidget::Icon(this) => this.get_info(),
            LvglWidget::TextArea(this) => this.get_info(),
        }
    }
    pub fn set_text(&self, text: &str) {
        match self {
            LvglWidget::Label(this) => {this.set_text(text);},
            LvglWidget::TextArea(this) => {this.set_text(text);},
            _ => {},
        }
    }
}

pub struct LvglButton {
    uid: &'static str,
    info: &'static str,
    handle: *mut cglue::_lv_obj_t,
    label: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}

impl_widget_trait!(LvglButton, Button);
impl LvglButton {
    pub fn new(uid: &'static str, info: &'static str,label: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
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
                info,
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
    info: &'static str,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    _style_pr: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}

impl_widget_trait!(LvglImgButton, ImgButton);
impl LvglImgButton {
    pub fn new(
        uid: &'static str,
        info: &'static str,
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
                info,
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
    info: &'static str,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglLabel, Label);
impl LvglLabel {
    pub fn new(uid: &'static str, info: &'static str, label: &str, x_ofs: i16, y_ofs: i16) -> &'static Self {
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
                info,
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
    info: &'static str,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglIcon, Icon);
impl LvglIcon {
    pub fn new(uid: &'static str, info: &'static str, pixmap: &[u8], x_ofs: i16, y_ofs: i16) -> &'static Self {
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
                info,
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

pub struct LvglTextArea {
    uid: &'static str,
    info: &'static str,
    handle: *mut cglue::_lv_obj_t,
    style: *mut cglue::lv_style_t,
    ctrlbox: Cell<Option<*mut dyn LvglHandler>>,
}
impl_widget_trait!(LvglTextArea, TextArea);
impl LvglTextArea {
    pub fn new(uid: &'static str, info: &'static str, x_ofs: i16, y_ofs: i16) -> &'static Self {
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
                info,
                handle,
                style,
                ctrlbox: Cell::new(None),
            };
            Box::leak(Box::new(widget))
        }
    }

    pub fn set_text(&self, text: &str) -> &Self {
        println! ("**** set_text {}", text);
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
