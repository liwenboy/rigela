/*
 * Copyright (c) 2024. The RigelA open source project team and
 * its contributors reserve all rights.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and limitations under the License.
 */

use crate::combo_key;
use crate::commander::keyboard::combo_keys::ComboKey;
use crate::commander::keyboard::combo_keys::State;
#[allow(unused_imports)]
use crate::commander::keyboard::keys::Keys::*;
use crate::commander::keyboard::modify_keys::ModifierKeys;
use crate::configs::operations::apply_mouse_config;
use crate::context::Context;
use async_trait::async_trait;
use rigela_macros::talent;
#[allow(unused_imports)]
use std::sync::Weak;
use win_wrap::input::{click, get_cur_mouse_point, right_click};

//noinspection RsUnresolvedPath
#[talent(doc = "鼠标单击", key = combo_key!(VkNumPadDiv))]
async fn click(context: Weak<Context>) {
    let (x, y) = get_point(context.clone()).await;
    click(x, y);
    unsafe { &*context.as_ptr() }
        .performer
        .speak(&t!("mouse.click"))
        .await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "鼠标右击", key = combo_key!(VkNumPadMul))]
async fn right_click(context: Weak<Context>) {
    let (x, y) = get_point(context.clone()).await;
    right_click(x, y);
    unsafe { &*context.as_ptr() }
        .performer
        .speak(&t!("mouse.right_click"))
        .await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "鼠标朗读", key = combo_key!("RigelA", VkM))]
async fn read_mouse(context: Weak<Context>) {
    let is_read = !unsafe { &*context.as_ptr() }
        .config_manager
        .get_config()
        .mouse_config
        .is_read;
    apply_mouse_config(context.clone(), is_read);
    let state = match is_read {
        true => t!("mouse.state_on"),
        false => t!("mouse.state_off"),
    };
    unsafe { &*context.as_ptr() }.performer.speak(&state).await;
}

async fn get_point(context: Weak<Context>) -> (i32, i32) {
    let context = unsafe { &*context.as_ptr() };
    let ele = match context.ui_navigator.get_last_visit().await {
        None => None,
        e => e,
    };
    match ele {
        None => get_cur_mouse_point(),
        Some(e) => {
            if let Some(r) = e.get_rect() {
                (r.left, r.top)
            } else {
                get_cur_mouse_point()
            }
        }
    }
}

/// 朗读鼠标元素
pub(crate) fn mouse_read(context: Weak<Context>, x: i32, y: i32) {
    let uia = unsafe { &*context.as_ptr() }.ui_automation.clone();
    let ele = uia.element_from_point(x, y).unwrap();
    let performer = unsafe { &*context.as_ptr() }.performer.clone();
    let work_runtime = unsafe { &*context.as_ptr() }.work_runtime;
    work_runtime.spawn(async move { performer.speak(&ele).await });
}
