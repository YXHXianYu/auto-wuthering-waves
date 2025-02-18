use crate::command::run_command_async;


pub fn is_admin() -> bool {
    #[cfg(windows)] {
        use windows::Win32::UI::Shell::IsUserAnAdmin;
        unsafe { IsUserAnAdmin().as_bool() }
    }

    #[cfg(not(windows))]
    {
        use crate::warning_println;
        warning_println!("is_admin is not implemented for non-Windows platforms. Skip admin check.");
        false
    }
}

pub fn run_myself_as_admin() -> bool {
    #[cfg(windows)] {
        let myself_path = std::env::current_exe().unwrap();
        let myself_path = format!("\"{}\"", myself_path.to_str().unwrap());
        run_command_async(vec![
            "powershell",
            "-Command",
            "Start-Process",
            "-FilePath",
            myself_path.as_str(),
            "-Verb",
            "runAs",
        ]);

        true
    }

    #[cfg(not(windows))]
    {
        use crate::warning_println;
        warning_println!("run_myself_as_admin is not implemented for non-Windows platforms. Skip admin check.");
        false
    }
}