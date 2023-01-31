use windows::{
    core::{
        Error,
        InParam, PCWSTR,
    },
    Win32::{
        Foundation::{
            HANDLE,
            LUID,
        },
        Security::{
            TOKEN_ACCESS_MASK,
            LookupPrivilegeValueW,
        },
        System::Threading::{
            GetCurrentProcess,
            OpenProcess,
            OpenProcessToken,
        },
    },
};

pub struct Process {
    pub handle: HANDLE,
}

impl Process {
    unsafe fn get_debug_privileges() {
        let self_process_hanlde = GetCurrentProcess();

        let mut token_handle = HANDLE::default();

        // TODO: check if returned bool is true or not
        OpenProcessToken(
            self_process_hanlde,
            TOKEN_ACCESS_MASK(0x1 | 0x2 | 0x4 | 0x8 | 0x10 | 0x20 | 0x40 | 0x80 | 0xF0000),
            &mut token_handle,
        );

        let mut wanted_luid = LUID::default();
        let thing: Vec<u16> = "SeDebugPrivilege".encode_utf16().collect();
        let wanted_privilege = PCWSTR::from_raw(thing.as_ptr());

        LookupPrivilegeValueW(
            None,
            wanted_privilege,
            &mut wanted_luid
        );
    }

    pub fn from_id(process_id: u32) -> Result<Self, Error> {
        // TODO: get debug privilege

        // TODO: is this unsafe code safe
        unsafe {
            let process_handle = OpenProcess(
                // I don't remember what flags these are
                windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS(
                    0xF0000 | 0x100000 | 0xFFFF,
                ),
                false,
                process_id,
            );

            process_handle.map(|handle| Process { handle })
        }
    }
}
