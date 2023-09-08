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
 *
 */

use crate::*;
//use libafb::prelude::*;

pub struct ApiUserData {
    pub uid: &'static str,
    pub socname: Option<&'static str>,
    pub devname: Option<&'static str>,
    pub eptname: &'static str,
    pub tic: u32,
    pub acls: &'static str,
}

fn to_static_str(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

impl AfbApiControls for ApiUserData {
    fn config(&mut self, api: &AfbApi, jconf: JsoncObj) -> Result<(),AfbError> {
        afb_log_msg!(Debug, api, "api={} config={}", api.get_uid(), jconf);

        Ok(())
    }

    // mandatory for downcasting back to custom api data object
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

// Binding init callback started at binding load time before any API exist
// -----------------------------------------
pub fn binding_init(rootv4: AfbApiV4, jconf: JsoncObj) -> Result<&'static AfbApi, AfbError> {
    afb_log_msg!(Info,rootv4, "config:{}",jconf);

    let apiname = if let Ok(value) = jconf.get::<String>("api_name") {
        to_static_str(value)
    } else {
        "lvgl"
    };

    let info = if let Ok(value) = jconf.get::<String>("info") {
        to_static_str(value)
    } else {
        ""
    };

    let socname = if let Ok(value) = jconf.get::<String>("soc_name") {
        Some(to_static_str(value))
    } else {
        None
    };

    let devname = if let Ok(value) = jconf.get::<String>("dev_name") {
        Some(to_static_str(value))
    } else {
        None
    };

    let eptname = if let Ok(value) = jconf.get::<String>("ept_name") {
        to_static_str(value)
    } else {
        "powerboard_rmsg"
    };

    let tic = if let Ok(value) = jconf.get::<u32>("tic") {
        value
    } else {
        10
    };

    let acls = if let Ok(value) = jconf.get::<String>("acls") {
        to_static_str(value)
    } else {
        "acl:rmsg:ti"
    };

    let config = ApiUserData {
        uid: apiname,
        socname,
        devname,
        eptname,
        acls,
        tic,
    };


    // register data converter
    //sockdata_register(rootv4)?;

    // create a new api
    let rmsgapi = AfbApi::new(apiname)
        .set_info(info)
        .set_permission(AfbPermission::new(to_static_str(acls.to_owned())))
        .seal(false);

    // register verbs and events
    verbs::register(rmsgapi, &config)?;

    // finalize api
    Ok(rmsgapi.finalize()?)
}

// register binding within libafb
AfbBindingRegister!(binding_init);
