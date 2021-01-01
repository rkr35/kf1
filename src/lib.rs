use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE},
    um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
    um::{consoleapi, libloaderapi, processthreadsapi, wincon},
};

mod logger;

fn idle() {
    use std::io::Read;

    println!("Idling.");

    let mut buffer = [0; 1];
    let _ = std::io::stdin().read_exact(&mut buffer);
}

unsafe extern "system" fn my_thread(dll: LPVOID) -> DWORD {
    consoleapi::AllocConsole();

    if let Err(e) = logger::initialize() {
        eprintln!("Unable to initialize logger: {}", e);
    } else {
        log::info!("Initialized logger.");
    }

    idle();

    wincon::FreeConsole();

    const EXIT_SUCCESS: u32 = 0;
    libloaderapi::FreeLibraryAndExitThread(dll.cast(), EXIT_SUCCESS);
    EXIT_SUCCESS
}

#[no_mangle]
unsafe extern "system" fn DllMain(dll: HINSTANCE, reason: DWORD, _: LPVOID) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        libloaderapi::DisableThreadLibraryCalls(dll);

        processthreadsapi::CreateThread(
            std::ptr::null_mut(), // uninheritable, default security descriptor
            0,                    // use default stack size
            Some(my_thread),      // function for thread to execute
            dll.cast(),           // pass handle to dll for thread to deallocate
            0,                    // run function immediately
            std::ptr::null_mut(), // don't return created thread id
        );
    } else if reason == DLL_PROCESS_DETACH {
        log::logger().flush();
    }

    TRUE
}
