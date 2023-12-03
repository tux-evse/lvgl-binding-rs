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

pub(crate) mod cglue {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("_capi-map.rs");
}

#[no_mangle]
pub extern "C" fn lvgl_events_cb(event: *mut cglue::lv_event_t) {
    unsafe {
        let _widget = cglue::lv_event_get_target(event);
        let ctx = cglue::lv_event_get_user_data(event);
        let event = &*(event as *mut cglue::lv_event_t);
        let widget = &mut *(ctx as *mut LvglWidget);
        widget.callback(event);
    }
}