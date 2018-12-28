extern crate winapi;
use winapi::shared::minwindef::DWORD;
use winapi::um::winnt::{HANDLE, HRESULT};
use winapi::um::wincon::COORD;

type HPCON = *mut winapi::ctypes::c_void;
extern "system" {
    fn CreatePseudoConsole(
        size: COORD,
        hInput: HANDLE,
        hOutput: HANDLE,
        dwFlags: DWORD,
        phPC: *mut HPCON,
    ) -> HRESULT;
    fn ResizePseudoConsole(hPC: HPCON, size: COORD) -> HRESULT;
    fn ClosePseudoConsole(hPC: HPCON);
}

fn main() {
    unsafe {
        const ZERO_HPCON_VALUE: HPCON = 0isize as HPCON;
        ClosePseudoConsole(ZERO_HPCON_VALUE);
    }

    println!("done");
}
