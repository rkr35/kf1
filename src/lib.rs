use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::libloaderapi,
    um::processthreadsapi,
    um::winnt::DLL_PROCESS_ATTACH,
};

unsafe extern "system" fn my_thread(dll: LPVOID) -> DWORD {
    0
}

#[no_mangle]
unsafe extern "system" fn DllMain(dll: HINSTANCE, reason: DWORD, _: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        libloaderapi::DisableThreadLibraryCalls(dll);

        processthreadsapi::CreateThread(
            std::ptr::null_mut(),   // uninheritable, default security descriptor
            0,                      // use default stack size
            Some(my_thread),        // function for thread to execute
            dll.cast(),             // pass handle to dll for thread to deallocate
            0,                      // run function immediately
            std::ptr::null_mut(),   // don't return created thread id
        );
    }

    TRUE
}
