extern crate winapi;
use std::ptr::null_mut;
use winapi::shared::minwindef::DWORD;
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::namedpipeapi::CreatePipe;
use winapi::um::wincon::COORD;
use winapi::um::winnt::{HANDLE, HRESULT};

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

//TODO: build a real abstraction around pipes
fn create_pipe_or_panic(read_pipe: &mut HANDLE, write_pipe: &mut HANDLE) {
    unsafe {
        let rc = CreatePipe(read_pipe, write_pipe, null_mut(), 0);
        if rc != 1 {
            panic!("CreatePipe: {}", rc);
        }
    }
}

fn close_pipe(pipe: &mut HANDLE) {
    unsafe {
        if *pipe != INVALID_HANDLE_VALUE {
            CloseHandle(*pipe);
            *pipe = INVALID_HANDLE_VALUE;
        }
    }
}

fn main() {
    let size = COORD { X: 80, Y: 24 };
    let mut pipe_pty_in = INVALID_HANDLE_VALUE;
    let mut pipe_out = INVALID_HANDLE_VALUE;
    create_pipe_or_panic(&mut pipe_pty_in, &mut pipe_out);
    let mut pipe_in = INVALID_HANDLE_VALUE;
    let mut pipe_pty_out = INVALID_HANDLE_VALUE;
    create_pipe_or_panic(&mut pipe_in, &mut pipe_pty_out);
    unsafe {
        let mut hpc: HPCON = null_mut();
        let rc = CreatePseudoConsole(size, pipe_pty_in, pipe_pty_out, 0, &mut hpc);
        if rc != 0 {
            panic!("CreatePseudoConsole: {}", rc);
        }

        close_pipe(&mut pipe_pty_in);
        close_pipe(&mut pipe_pty_out);

        let rc = ResizePseudoConsole(hpc, size);
        if rc != 0 {
            panic!("CreatePseudoConsole: {}", rc);
        }

        ClosePseudoConsole(hpc);

        close_pipe(&mut pipe_in);
        close_pipe(&mut pipe_out);
    }

    println!("done");
}
