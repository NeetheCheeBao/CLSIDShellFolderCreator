use std::ffi::c_void;
use std::mem::zeroed;

use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, SIZE, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    BeginPaint, CreateFontW, CreateSolidBrush, DeleteObject, DrawTextW, EndPaint, FillRect,
    FrameRect, GetDC,
    GetTextExtentPoint32W, ReleaseDC, SelectObject, SetBkMode, SetTextColor, CLIP_DEFAULT_PRECIS,
    CLEARTYPE_QUALITY, DEFAULT_CHARSET, DT_LEFT, DT_SINGLELINE, DT_VCENTER, FF_DONTCARE, HDC,
    HFONT, InvalidateRect, OUT_DEFAULT_PRECIS, PAINTSTRUCT, TRANSPARENT,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, GetClientRect, GetCursorPos, GetSystemMetrics, GetWindowTextW,
    LoadCursorW, RegisterClassW, SetWindowPos, SetWindowTextW, ShowWindow,
    CS_HREDRAW, CS_VREDRAW, HWND_TOPMOST, IDC_ARROW, SM_CXCURSOR, SM_CXSCREEN, SM_CYCURSOR,
    SM_CYSCREEN, SWP_NOACTIVATE,
    SWP_SHOWWINDOW, SW_HIDE, WNDCLASSW, WS_EX_NOACTIVATE, WS_EX_TOOLWINDOW, WS_EX_TOPMOST, WS_POPUP,
};

static mut TIP_FONT: HFONT = std::ptr::null_mut();

// 独立的置顶弹出窗，可以超出主窗口范围
pub struct Tip {
    hwnd: HWND,
    font: HFONT,
}

impl Tip {
    pub fn new() -> Option<Tip> {
        unsafe {
            let hinst = GetModuleHandleW(std::ptr::null());
            let class = wide("ClsidTipWindow");
            let wc = WNDCLASSW {
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(wndproc),
                hInstance: hinst,
                hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
                lpszClassName: class.as_ptr(),
                ..zeroed()
            };
            RegisterClassW(&wc);

            let empty = wide("");
            let hwnd = CreateWindowExW(
                WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE,
                class.as_ptr(),
                empty.as_ptr(),
                WS_POPUP,
                0,
                0,
                10,
                10,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                hinst,
                std::ptr::null(),
            );
            if hwnd.is_null() {
                return None;
            }

            let face = wide("Microsoft YaHei UI");
            let font = CreateFontW(
                -12,
                0,
                0,
                0,
                400,
                0,
                0,
                0,
                DEFAULT_CHARSET as u32,
                OUT_DEFAULT_PRECIS as u32,
                CLIP_DEFAULT_PRECIS as u32,
                CLEARTYPE_QUALITY as u32,
                FF_DONTCARE as u32,
                face.as_ptr(),
            );
            std::ptr::addr_of_mut!(TIP_FONT).write(font);
            Some(Tip { hwnd, font })
        }
    }

    pub fn show(&self, text: &str) {
        unsafe {
            let w = wide(text);
            SetWindowTextW(self.hwnd, w.as_ptr());

            let hdc = GetDC(self.hwnd);
            let old = SelectObject(hdc, self.font);
            let mut size = SIZE { cx: 0, cy: 0 };
            GetTextExtentPoint32W(hdc, w.as_ptr(), w.len() as i32 - 1, &mut size);
            SelectObject(hdc, old);
            ReleaseDC(self.hwnd, hdc);

            let (bw, bh) = (size.cx + 16, size.cy + 8);
            let mut pt = POINT { x: 0, y: 0 };
            GetCursorPos(&mut pt);
            let (sw, sh) = (
                GetSystemMetrics(SM_CXSCREEN),
                GetSystemMetrics(SM_CYSCREEN),
            );
            // 按系统指针尺寸外推，确保整个指针矩形都不压到气泡
            let cx = GetSystemMetrics(SM_CXCURSOR).max(16);
            let cy = GetSystemMetrics(SM_CYCURSOR).max(16);
            let mut x = pt.x + cx + 6;
            let mut y = pt.y + 6;
            // 右侧放不下就翻到指针左边，下方放不下就上移
            if x + bw > sw {
                x = pt.x - bw - cx - 6;
            }
            if y + bh > sh {
                y = pt.y - bh - cy - 6;
            }
            SetWindowPos(
                self.hwnd,
                HWND_TOPMOST,
                x,
                y,
                bw,
                bh,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );
            InvalidateRect(self.hwnd, std::ptr::null(), 1);
        }
    }

    pub fn hide(&self) {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
        }
    }
}

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    match msg {
        windows_sys::Win32::UI::WindowsAndMessaging::WM_PAINT => {
            let mut ps: PAINTSTRUCT = zeroed();
            let hdc: HDC = BeginPaint(hwnd, &mut ps);
            let mut rc: RECT = zeroed();
            GetClientRect(hwnd, &mut rc);

            let brush = CreateSolidBrush(0x00FF_FFFF);
            FillRect(hdc, &rc, brush);
            DeleteObject(brush as *mut c_void);

            let border = CreateSolidBrush(0x00C0_C0C0);
            FrameRect(hdc, &rc, border);
            DeleteObject(border as *mut c_void);

            SetBkMode(hdc, TRANSPARENT as i32);
            SetTextColor(hdc, 0x0000_0000);
            let font = std::ptr::addr_of!(TIP_FONT).read();
            let old = SelectObject(hdc, font);

            let mut buf = [0u16; 512];
            let n = GetWindowTextW(hwnd, buf.as_mut_ptr(), buf.len() as i32);
            let mut trc = RECT {
                left: rc.left + 8,
                top: rc.top,
                right: rc.right - 8,
                bottom: rc.bottom,
            };
            DrawTextW(
                hdc,
                buf.as_ptr(),
                n,
                &mut trc,
                DT_LEFT | DT_VCENTER | DT_SINGLELINE,
            );

            SelectObject(hdc, old);
            EndPaint(hwnd, &ps);
            0
        }
        windows_sys::Win32::UI::WindowsAndMessaging::WM_ERASEBKGND => 1,
        _ => DefWindowProcW(hwnd, msg, wp, lp),
    }
}
