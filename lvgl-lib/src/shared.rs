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

use crate::prelude::*;
use std::any::Any;
use std::mem;
use std::os::raw;
use std::{thread, time};

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum LvglEvent {
    PRESSED,
    PRESSING,
    PRESS_LOST,
    SHORT_CLICKED,
    LONG_PRESSED,
    LONG_PRESSED_REPEAT,
    CLICKED,
    RELEASED,
    FOCUSED,
    DEFOCUSED,
    LEAVE,
    VALUE_CHANGED,
    UNKNOWN,
}

impl LvglEvent {
    pub(crate) fn from(code: u32) -> Self {
        match code {
            1 => Self::PRESSED,
            2 => Self::PRESSING,
            3 => Self::PRESS_LOST,
            4 => Self::SHORT_CLICKED,
            5 => Self::LONG_PRESSED,
            6 => Self::LONG_PRESSED_REPEAT,
            7 => Self::CLICKED,
            8 => Self::RELEASED,
            14 => Self::FOCUSED,
            15 => Self::DEFOCUSED,
            16 => Self::LEAVE,
            28 => Self::VALUE_CHANGED,

            _ => Self::UNKNOWN,
        }
    }
}

pub trait LvglCommon {
    fn get_handle(&self) -> *mut cglue::lv_obj_t;
    fn get_style(&self) -> *mut cglue::lv_style_t;
    fn get_uid(&self) -> &'static str;
    fn get_info(&self) -> &'static str;
    fn get_generic(&'static self) -> LvglWidget;
    fn get_callback(&self) -> Option<*mut dyn LvglHandler>;
    fn finalize(&'static self) -> &'static LvglWidget;
    fn callback(&self, _event: &cglue::lv_event_t) {}
    fn get_action(&self) -> &'static str{"[]"}
    fn set_callback(&'static self, ctrlbox: Option<*mut dyn LvglHandler>);
    fn set_info(&self, info: &'static str) -> &Self;
    fn as_any(&self) -> &dyn Any;
}
// common trait should be implemented for each widget because internal object struct is not identical
#[macro_export]
macro_rules! impl_widget_trait {
    ($widget:ty, $object:ident) => {
        impl LvglCommon for $widget {
            fn get_uid(&self) -> &'static str {
                self.uid
            }
            fn get_info(&self) -> &'static str {
                self.info.get()
            }
            fn set_info(&self, info: &'static str) -> &Self {
                self.info.set(info);
                self
            }
            fn get_handle(&self) -> *mut cglue::_lv_obj_t {
                self.handle
            }
            fn get_style(&self) -> *mut cglue::lv_style_t {
                self.style
            }
            fn get_callback(&self) -> Option<*mut dyn LvglHandler> {
                self.ctrlbox.get()
            }
            fn get_generic(&'static self) -> LvglWidget {
                LvglWidget::$object(self)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            // if callback not set do it
            fn set_callback(&'static self, ctrlbox: Option<*mut dyn LvglHandler>) {
                if let None = self.ctrlbox.get() {
                    match ctrlbox {
                        None => {}
                        Some(_) => {
                            self.ctrlbox.set(ctrlbox);
                            let context = Box::leak(Box::new(LvglWidget::$object(self)));
                            unsafe {
                                cglue::lv_obj_add_event_cb(
                                    self.get_handle(),
                                    Some(lvgl_events_cb),
                                    cglue::lv_event_code_t_LV_EVENT_ALL,
                                    context as *const _ as *mut raw::c_void,
                                );
                            }
                        }
                    }
                }
            }
            fn finalize(&'static self) -> &'static LvglWidget {
                Box::leak(Box::new(LvglWidget::$object(self)))
            }
        }
        impl LvglMethod for $widget {}
    };
}

pub type LvglPoint = cglue::lv_point_t;

pub struct LvglColor {
    pub(crate) handle: cglue::lv_color_t,
}
impl LvglColor {
    pub fn rvb(red: u8, green: u8, blue: u8) -> Self {
        let handle = unsafe { cglue::lv_color_mk(red, green, blue) };
        LvglColor { handle }
    }

    pub fn palette(palette: u32) -> Self {
        let handle = unsafe { cglue::lv_palette_main(palette) };
        LvglColor { handle }
    }
}

pub trait LvglMethod {
    fn set_size(&self, width: i16, height: i16) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            cglue::lv_obj_set_width(handle, width);
            cglue::lv_obj_set_height(handle, height);
        }
        self
    }

    fn set_width(&self, width: i16) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            cglue::lv_obj_set_width(handle, width);
        }
        self
    }

    fn set_height(&self, height: i16) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        unsafe {
            cglue::lv_obj_set_height(handle, height);
        }
        self
    }
    fn set_color(&self, color: LvglColor) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        let style = self.get_style();
        unsafe {
            cglue::lv_style_set_text_color(style, color.handle);
            cglue::lv_obj_add_style(handle, style, cglue::LV_STATE_DEFAULT);
        }
        self
    }

    fn set_border(&self, width: i16, color: LvglColor) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        let style = self.get_style();
        unsafe {
            cglue::lv_style_set_border_width(style, width);
            cglue::lv_style_set_border_color(style, color.handle);
            cglue::lv_obj_add_style(handle, style, cglue::LV_STATE_DEFAULT);
            cglue::lv_obj_set_style_radius(
                handle,
                cglue::LV_RADIUS_CIRCLE as i16,
                cglue::LV_STATE_DEFAULT,
            );
        }
        self
    }

    fn set_background(&self, color: LvglColor) -> &Self
    where
        Self: LvglCommon,
    {
        let handle = self.get_handle();
        let style = self.get_style();
        unsafe {
            cglue::lv_style_set_bg_color(style, color.handle);
            cglue::lv_style_set_bg_opa(style, cglue::LV_OPA_50 as u8);
            cglue::lv_obj_add_style(handle, style, cglue::LV_STATE_DEFAULT);
        }
        self
    }

}

pub struct LvglHandle {
    _fbdev_handle: &'static cglue::lv_disp_drv_t,
}

impl LvglHandle {
    pub fn new(x_res: i16, y_res: i16, draw_ratio: u32) -> Self {
        unsafe {
            cglue::lv_init();
            cglue::fbdev_init();
            cglue::evdev_init();

            // drawing buffer that can be smaller than screen definition
            let buffer_sz = x_res as u32 * y_res as u32 / draw_ratio;
            let disp_buffer = Vec::<cglue::lv_color_t>::with_capacity(buffer_sz as usize).leak();

            // draw buffer handle
            let draw_buffer = Box::leak(Box::new(mem::zeroed::<cglue::_lv_disp_draw_buf_t>()));
            cglue::lv_disp_draw_buf_init(
                draw_buffer,
                disp_buffer as *const _ as *mut raw::c_void,
                0 as *mut raw::c_void,
                buffer_sz as u32,
            );

            // frame buffer driver handle
            let fbdev_handle = Box::leak(Box::new(mem::zeroed::<cglue::lv_disp_drv_t>()));
            cglue::lv_disp_drv_init(fbdev_handle);
            fbdev_handle.draw_buf = draw_buffer;
            fbdev_handle.flush_cb = Some(cglue::fbdev_flush);
            fbdev_handle.hor_res = x_res;
            fbdev_handle.ver_res = y_res;
            //fbdev_handle.physical_hor_res = x_res;
            //fbdev_handle.physical_ver_res = y_res;
            cglue::lv_disp_drv_register(fbdev_handle);

            LvglHandle {
                _fbdev_handle: fbdev_handle,
            }
        }
    }

    pub fn set_theme(
        &mut self,
        primary: LvglColor,
        secondary: LvglColor,
        dark: bool,
        font: &LvglFont,
    ) -> &Self {
        unsafe {
            let display = cglue::lv_disp_get_default();
            cglue::lv_disp_set_bg_color(display, cglue::lv_color_mk(100, 100, 100));
            cglue::lv_disp_set_bg_opa(display, 128);

            let theme = cglue::lv_theme_default_init(
                display,
                primary.handle,
                secondary.handle,
                dark,
                font as *const _ as *const cglue::lv_font_t,
            );
            cglue::lv_disp_set_theme(display as *mut cglue::_lv_disp_t, theme);

            // input event handler
            let indev_handle = Box::leak(Box::new(mem::zeroed::<cglue::lv_indev_drv_t>()));
            indev_handle.type_ = cglue::lv_indev_type_t_LV_INDEV_TYPE_POINTER;
            indev_handle.read_cb = Some(cglue::evdev_read);
            let mouse_handle = cglue::lv_indev_drv_register(indev_handle);
            let cursor_handle = cglue::lv_img_create(cglue::lv_scr_action());
            cglue::lv_img_set_src(
                cursor_handle,
                &cglue::lv_mouse_cursor as *const _ as *const raw::c_void,
            );
            cglue::lv_indev_set_cursor(mouse_handle, cursor_handle);
        }
        self
    }

    // notify lvgl how long we've been sleeping update event and return next expected wait in ms
    pub fn start_loop(&self) {
        thread::spawn(|| {
            let mut tic = 5; // foOptione lvgl to process waiting events
            loop {
                unsafe {
                    cglue::lv_tick_inc(tic);
                    tic = cglue::lv_timer_handler()
                };
                let delay = time::Duration::from_millis(tic as u64);
                thread::sleep(delay);
            }
        });
    }
}
