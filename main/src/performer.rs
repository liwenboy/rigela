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
use std::future::Future;
use win_wrap::{common::Result, tts::Tts};
pub(crate) trait Speakable {
    fn get_sentence(&self) -> String;
}
/**
 * 表演者对象结构。
 * 可以进行语音输出或音效提示。
 * */
#[derive(Clone)]
pub(crate) struct Performer(Tts);
impl Performer {
    /**
     * 创建表演者对象。
     * */
    pub(crate) fn new() -> Self {
        Self(Tts::new())
    }
    /**
     * 使用语音输出，播报对象的信息。
     * */
    pub(crate) fn speak<'a>(&'a self, speakable: &'a (dyn Speakable + Sync)) -> impl Future<Output = Result<()>> + 'a {
        async {
            self.0.speak(speakable.get_sentence().as_str()).await
        }
    }
}