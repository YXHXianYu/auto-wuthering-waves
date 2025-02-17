
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