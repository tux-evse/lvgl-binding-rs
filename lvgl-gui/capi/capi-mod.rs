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

use lvgl::prelude::*;

pub(crate) mod cglue {
    #![allow(dead_code)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    use lvgl::prelude::LvglImgDsc;
    type lv_img_dsc_t= LvglImgDsc;
    include!("_capi-map.rs");
}

macro_rules! impl_static_imgbin {
    ($label:ident, $imgbin:ident) => {
        pub fn $label() -> &'static LvglImgDsc {
            unsafe { &cglue::$imgbin }
        }
    }
}

// export static img asset
include!("../assets/@img-assets.rs");
