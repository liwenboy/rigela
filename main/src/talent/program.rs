/*
 * Copyright (c) 2023. The RigelA open source project team and
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
use crate::commander::keyboard::modify_keys::ModifierKeys;
use crate::{
    commander::keyboard::keys::Keys::*,
    context::Context,
    ext::window::AccessibleWindowExt,
    performer::{sound::SoundArgument::Single, Speakable},
};
use async_trait::async_trait;
use chrono::prelude::{DateTime, Local};
use log::error;
use rigela_macros::talent;
use std::{
    sync::{OnceLock, Weak},
    thread,
    time::Duration,
};
use win_wrap::{
    msaa::object::AccessibleObject,
    pdh::{PdhCounter, PdhCounterExt, PdhQuery},
};

//noinspection RsUnresolvedReference
#[talent(doc = "退出", key = combo_key!("RigelA", VkEscape))]
async fn exit(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    context.performer.speak(&t!("program.exit")).await;
    context.terminator.exit().await;
}

impl Speakable for DateTime<Local> {
    fn get_sentence(&self) -> String {
        self.format("%Y年%m月%d日 %H:%M:%S").to_string()
    }
}

//noinspection RsUnresolvedReference
#[talent(doc = "当前时间", key = combo_key!("RigelA", VkF12))]
async fn current_time(context: Weak<Context>) {
    let msg = Local::now().format("%H时%M分%S秒").to_string();
    unsafe { &*context.as_ptr() }.performer.speak(&msg).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "当前日期", key = combo_key!("RigelA", VkF12, double))]
async fn current_date(context: Weak<Context>) {
    let msg = Local::now().format("%Y年%m月%d日").to_string();
    unsafe { &*context.as_ptr() }.performer.speak(&msg).await;
}

impl Speakable for &PdhCounter {
    fn get_sentence(&self) -> String {
        t!(
            "program.current_cpu_usage",
            value = self.get_value().1.round()
        )
        .to_string()
    }
}

//noinspection RsUnresolvedReference
#[talent(doc = "查看CPU使用率", key = combo_key!("RigelA", VkQ))]
async fn current_cpu_usage(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    static CPU_QUERY: OnceLock<(PdhCounter, PdhQuery)> = OnceLock::new();
    let (counter, query) = CPU_QUERY.get_or_init(|| {
        let query = PdhQuery::new();
        let counter = query.add_counter(format!(
            r"\Processor Information({})\% Processor Time",
            "_Total"
        ));
        query.collect_data();
        thread::sleep(Duration::from_millis(20));
        (counter, query)
    });
    query.collect_data();
    context.performer.speak(&counter).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "弹出菜单", key = combo_key!("RigelA", VkR))]
async fn popup_menu(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    context.gui_provider.show_popup_menu();
}

//noinspection RsUnresolvedReference
#[talent(doc = "自定义热键", key = combo_key!("RigelA", VkK))]
async fn hotkeys(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    context.gui_provider.show_hotkeys_form();
}

//noinspection RsUnresolvedReference
#[talent(doc = "查看前景窗口标题", key = combo_key!("RigelA", VkT))]
async fn view_window_title(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    match AccessibleObject::from_foreground_window() {
        Ok(o) => {
            context.performer.speak(&(o, 0)).await;
        }
        Err(e) => {
            error!(
                "Can't get the object of the foreground window, because {}.",
                e
            );
            context.performer.play_sound(Single("error.wav")).await
        }
    }
}

//noinspection RsUnresolvedReference
#[talent(doc = "查看当前焦点", key = combo_key!("RigelA", VkTab))]
async fn view_focus(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    let Ok(focused) = context.ui_automation.get_focused_element() else {
        return;
    };
    context.performer.speak(&focused).await;
}

//noinspection RsUnresolvedReference
#[talent(doc = "停止正在输出的语音", key = combo_key!("Ctrl", VkSpace))]
async fn stop_tts_output(context: Weak<Context>) {
    let context = unsafe { &*context.as_ptr() };

    context.performer.get_tts().stop_all().await;
}
