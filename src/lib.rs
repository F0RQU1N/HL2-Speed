use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::{libloaderapi::GetModuleHandleA, processthreadsapi::CreateThread}
};
use std::ptr::null_mut as NULL;

static mut aGetLocalPlayer: DWORD = 0;
const m_flMaxspeed: u32 = 0xDAC;

unsafe extern "system" fn StartRoutine(__: *mut winapi::ctypes::c_void) -> DWORD {
    loop {
        let mut pLocalPlayer: DWORD = 0;

        std::arch::asm!(
            "call {}",
            "mov {}, eax",
            "pop ebx",
            in(reg) aGetLocalPlayer,
            out(reg) pLocalPlayer,
        );

        if pLocalPlayer != 0 {

            *((pLocalPlayer + m_flMaxspeed) as *mut f32) = 7777f32;
        }

        winapi::um::synchapi::Sleep(3000);
    }
    return 0;
}

#[no_mangle]
pub extern "system" fn DllMain(instance: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    if reason == 0x1 {
        unsafe {
            aGetLocalPlayer = GetModuleHandleA( b"server.dll\0".as_ptr().cast() ) as DWORD + 0x26D5F0;

            CreateThread(NULL(), 0, Some(StartRoutine), NULL(), 0, NULL());
        }
    }
    
    return TRUE;
}
