
extern crate winapi;

type HPCON = *mut winapi::ctypes::c_void;
extern "system" {
    fn ClosePseudoConsole(hPC : HPCON);
}

fn main() {
    unsafe {
        const ZERO_HPCON_VALUE: HPCON = 0isize as HPCON;
        ClosePseudoConsole(ZERO_HPCON_VALUE);
    }

    println!("done");
}
