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

use crate::common::{BOOL, HIMC, HWND, LPARAM};
use crate::ext::StringExt;
use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::Ime::{
    ImmGetCandidateListCountW, ImmGetCandidateListW, ImmGetContext, ImmReleaseContext,
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    keybd_event, mouse_event, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
};
use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos};
pub use windows::Win32::UI::{
    Input::{
        Ime::{
            CANDIDATELIST, IMN_CHANGECANDIDATE, IMN_CLOSECANDIDATE, IMN_CLOSESTATUSWINDOW,
            IMN_GUIDELINE, IMN_OPENCANDIDATE, IMN_OPENSTATUSWINDOW, IMN_PRIVATE,
            IMN_SETCANDIDATEPOS, IMN_SETCOMPOSITIONFONT, IMN_SETCOMPOSITIONWINDOW,
            IMN_SETCONVERSIONMODE, IMN_SETOPENSTATUS, IMN_SETSENTENCEMODE, IMN_SETSTATUSWINDOWPOS,
            IMN_SOFTKBDDESTROYED,
        },
        KeyboardAndMouse::{
            VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A, VK_ABNT_C1,
            VK_ABNT_C2, VK_ACCEPT, VK_ADD, VK_APPS, VK_ATTN, VK_B, VK_BACK, VK_BROWSER_BACK,
            VK_BROWSER_FAVORITES, VK_BROWSER_FORWARD, VK_BROWSER_HOME, VK_BROWSER_REFRESH,
            VK_BROWSER_SEARCH, VK_BROWSER_STOP, VK_C, VK_CANCEL, VK_CAPITAL, VK_CLEAR, VK_CONTROL,
            VK_CONVERT, VK_CRSEL, VK_D, VK_DBE_ALPHANUMERIC, VK_DBE_CODEINPUT, VK_DBE_DBCSCHAR,
            VK_DBE_DETERMINESTRING, VK_DBE_ENTERDLGCONVERSIONMODE, VK_DBE_ENTERIMECONFIGMODE,
            VK_DBE_ENTERWORDREGISTERMODE, VK_DBE_FLUSHSTRING, VK_DBE_HIRAGANA, VK_DBE_KATAKANA,
            VK_DBE_NOCODEINPUT, VK_DBE_NOROMAN, VK_DBE_ROMAN, VK_DBE_SBCSCHAR, VK_DECIMAL,
            VK_DELETE, VK_DIVIDE, VK_DOWN, VK_E, VK_END, VK_EREOF, VK_ESCAPE, VK_EXECUTE, VK_EXSEL,
            VK_F, VK_F1, VK_F10, VK_F11, VK_F12, VK_F13, VK_F14, VK_F15, VK_F16, VK_F17, VK_F18,
            VK_F19, VK_F2, VK_F20, VK_F21, VK_F22, VK_F23, VK_F24, VK_F3, VK_F4, VK_F5, VK_F6,
            VK_F7, VK_F8, VK_F9, VK_FINAL, VK_FPARAM, VK_G, VK_GAMEPAD_A, VK_GAMEPAD_B,
            VK_GAMEPAD_DPAD_DOWN, VK_GAMEPAD_DPAD_LEFT, VK_GAMEPAD_DPAD_RIGHT, VK_GAMEPAD_DPAD_UP,
            VK_GAMEPAD_LEFT_SHOULDER, VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON,
            VK_GAMEPAD_LEFT_THUMBSTICK_DOWN, VK_GAMEPAD_LEFT_THUMBSTICK_LEFT,
            VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT, VK_GAMEPAD_LEFT_THUMBSTICK_UP,
            VK_GAMEPAD_LEFT_TRIGGER, VK_GAMEPAD_MENU, VK_GAMEPAD_RIGHT_SHOULDER,
            VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON, VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN,
            VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT, VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT,
            VK_GAMEPAD_RIGHT_THUMBSTICK_UP, VK_GAMEPAD_RIGHT_TRIGGER, VK_GAMEPAD_VIEW,
            VK_GAMEPAD_X, VK_GAMEPAD_Y, VK_H, VK_HANGEUL, VK_HANGUL, VK_HANJA, VK_HELP, VK_HOME,
            VK_I, VK_ICO_00, VK_ICO_CLEAR, VK_ICO_HELP, VK_IME_OFF, VK_IME_ON, VK_INSERT, VK_J,
            VK_JUNJA, VK_K, VK_KANA, VK_KANJI, VK_L, VK_LAUNCH_APP1, VK_LAUNCH_APP2,
            VK_LAUNCH_MAIL, VK_LAUNCH_MEDIA_SELECT, VK_LBUTTON, VK_LCONTROL, VK_LEFT, VK_LMENU,
            VK_LSHIFT, VK_LWIN, VK_M, VK_MBUTTON, VK_MEDIA_NEXT_TRACK, VK_MEDIA_PLAY_PAUSE,
            VK_MEDIA_PREV_TRACK, VK_MEDIA_STOP, VK_MENU, VK_MODECHANGE, VK_MULTIPLY, VK_N,
            VK_NAVIGATION_ACCEPT, VK_NAVIGATION_CANCEL, VK_NAVIGATION_DOWN, VK_NAVIGATION_LEFT,
            VK_NAVIGATION_MENU, VK_NAVIGATION_RIGHT, VK_NAVIGATION_UP, VK_NAVIGATION_VIEW, VK_NEXT,
            VK_NONAME, VK_NONCONVERT, VK_NUMLOCK, VK_NUMPAD0, VK_NUMPAD1, VK_NUMPAD2, VK_NUMPAD3,
            VK_NUMPAD4, VK_NUMPAD5, VK_NUMPAD6, VK_NUMPAD7, VK_NUMPAD8, VK_NUMPAD9, VK_O, VK_OEM_1,
            VK_OEM_102, VK_OEM_2, VK_OEM_3, VK_OEM_4, VK_OEM_5, VK_OEM_6, VK_OEM_7, VK_OEM_8,
            VK_OEM_ATTN, VK_OEM_AUTO, VK_OEM_AX, VK_OEM_BACKTAB, VK_OEM_CLEAR, VK_OEM_COMMA,
            VK_OEM_COPY, VK_OEM_CUSEL, VK_OEM_ENLW, VK_OEM_FINISH, VK_OEM_FJ_JISHO, VK_OEM_FJ_LOYA,
            VK_OEM_FJ_MASSHOU, VK_OEM_FJ_ROYA, VK_OEM_FJ_TOUROKU, VK_OEM_JUMP, VK_OEM_MINUS,
            VK_OEM_NEC_EQUAL, VK_OEM_PA1, VK_OEM_PA2, VK_OEM_PA3, VK_OEM_PERIOD, VK_OEM_PLUS,
            VK_OEM_RESET, VK_OEM_WSCTRL, VK_P, VK_PA1, VK_PACKET, VK_PAUSE, VK_PLAY, VK_PRINT,
            VK_PRIOR, VK_PROCESSKEY, VK_Q, VK_R, VK_RBUTTON, VK_RCONTROL, VK_RETURN, VK_RIGHT,
            VK_RMENU, VK_RSHIFT, VK_RWIN, VK_S, VK_SCROLL, VK_SELECT, VK_SEPARATOR, VK_SHIFT,
            VK_SLEEP, VK_SNAPSHOT, VK_SPACE, VK_SUBTRACT, VK_T, VK_TAB, VK_U, VK_UP, VK_V,
            VK_VOLUME_DOWN, VK_VOLUME_MUTE, VK_VOLUME_UP, VK_W, VK_X, VK_XBUTTON1, VK_XBUTTON2,
            VK_Y, VK_Z, VK_ZOOM,
        },
    },
    WindowsAndMessaging::{
        WM_CHAR, WM_DEADCHAR, WM_IME_CHAR, WM_IME_COMPOSITION, WM_IME_COMPOSITIONFULL,
        WM_IME_CONTROL, WM_IME_ENDCOMPOSITION, WM_IME_KEYDOWN, WM_IME_KEYLAST, WM_IME_KEYUP,
        WM_IME_NOTIFY, WM_IME_REQUEST, WM_IME_SELECT, WM_IME_SETCONTEXT, WM_IME_STARTCOMPOSITION,
        WM_KEYDOWN, WM_KEYUP, WM_MOUSEMOVE, WM_SYSCHAR, WM_SYSDEADCHAR, WM_SYSKEYDOWN, WM_SYSKEYUP,
    },
};
use windows::{
    core::imp::{heap_alloc, heap_free},
    Win32::UI::Input::KeyboardAndMouse::{
        GetAsyncKeyState, GetKeyNameTextW, GetKeyState, VIRTUAL_KEY,
    },
};

pub type VirtualKey = VIRTUAL_KEY;

/**
 * 获取一个键的按下状态（从上一次调用此函数开始计算），返回的第一个值表示按下过这个键并一直到现在都处于按下状态，第二个值表示是否再次按下过他。
 * `key` 虚拟键常亮。
 * */
pub fn get_async_key_state(key: VirtualKey) -> (bool, bool) {
    let state = unsafe { GetAsyncKeyState(key.0 as i32) };
    let (high, low) = (state >> 8, state & 0xff);
    (high != 0, low != 0)
}

/**
 * 获取某个键的当前状态，返回的第一个值表示当前是否按下，第二个值表示锁定状态（例如大小写锁定键）。
 * `key` 虚拟键常亮。
 * */
pub fn get_key_state(key: VirtualKey) -> (bool, bool) {
    let state = unsafe { GetKeyState(key.0 as i32) };
    let high = state >> 8;
    let low = state & 0xffi16;
    (high != 0, low != 0)
}

/// 模拟按键
pub fn send_key(key: VirtualKey) {
    unsafe {
        keybd_event(key.0 as u8, 0, KEYEVENTF_EXTENDEDKEY, 0);
        keybd_event(key.0 as u8, 0, KEYEVENTF_EXTENDEDKEY | KEYEVENTF_KEYUP, 0);
    }
}

/**
 * 获取一个键的名称。
 */
pub fn get_key_name(l_param: &LPARAM) -> String {
    unsafe {
        let mut text: [u16; 16] = [0; 16];
        let len = GetKeyNameTextW(l_param.0 as i32, &mut text);
        String::from_utf16_lossy(&text[..len as usize])
    }
}

/**
 * 此函数查询与指定窗口关联的输入上下文。
 * 在尝试访问上下文中的信息之前，应用程序应常规使用此函数来查询当前输入上下文。
 * `h_wnd` 要查询其输入上下文的窗口的句柄。
 * */
pub fn imm_get_context(h_wnd: HWND) -> HIMC {
    unsafe { ImmGetContext(h_wnd) }
}

/**
 * 此函数释放输入上下文并解锁上下文中关联的内存。应用程序必须为每次调用 imm_get_context 函数调用 imm_release_context。
 * `h_wnd` 先前查询其输入上下文的窗口的句柄。
 * `h_imc` 输入上下文的句柄。
 * */
pub fn imm_release_context(h_wnd: HWND, h_imc: HIMC) -> BOOL {
    unsafe { ImmReleaseContext(h_wnd, h_imc) }
}

/**
 * 查询候选列表的大小。
 * `h_imc` 输入上下文的句柄。
 */
pub fn imm_get_candidate_list_count(h_imc: HIMC) -> (u32, u32) {
    unsafe {
        let mut list_count = 0u32;
        let buffer_len = ImmGetCandidateListCountW(h_imc, &mut list_count);
        (buffer_len, list_count)
    }
}

/**
 * 此函数查询指定的候选列表，并将该列表复制到指定的缓冲区。
 * `h_imc` 输入上下文的句柄。
 * `index` 候选列表的从零开始的索引。
 * */
pub fn imm_get_candidate_list(h_imc: HIMC, index: u32) -> (CANDIDATELIST, Vec<String>) {
    unsafe {
        let len = ImmGetCandidateListW(h_imc, index, None, 0);
        let ptr = heap_alloc(len as usize).unwrap_or(std::ptr::null_mut());
        if ptr.is_null() {
            return (CANDIDATELIST::default(), vec![]);
        }
        let p_list = ptr as *mut CANDIDATELIST;
        ImmGetCandidateListW(h_imc, index, Some(p_list), len);
        let list = *p_list;
        let p1 = p_list as *const u32;
        let p2 = p_list as *const u8;
        let mut data = Vec::new();
        for i in 0..list.dwCount {
            // 因为list.dwOffset在rust编译器处理后，数组大小固定为1，所以不可以访问数组中的其他元素，这里需通过裸指针实现
            let offset = *p1.wrapping_add(6 + i as usize); // 6表示dwOffset在CANDIDATELIST结构中的第六个元素
                                                           // 获取候选字符串
            data.push(p2.wrapping_add(offset as usize).to_string_utf16());
        }
        heap_free(ptr);
        (list, data)
    }
}

/// 鼠标单击
pub fn click(x: i32, y: i32) {
    unsafe {
        SetCursorPos(x, y).expect("SetCursorPos failed");
        mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    }
}

/// 鼠标右键单击
pub fn right_click(x: i32, y: i32) {
    unsafe {
        SetCursorPos(x, y).expect("SetCursorPos failed");
        mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
        mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
    }
}

/// 获取鼠标当前坐标
pub fn get_cur_mouse_point() -> (i32, i32) {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        GetCursorPos(&mut point).expect("GetCursorPos failed");
        (point.x, point.y)
    }
}
