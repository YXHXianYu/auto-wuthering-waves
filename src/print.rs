// command <=> yellow
#[macro_export]
macro_rules! command_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<y>[Command]      </y>: {}", format!($($arg)*))
    }};
}

// adb <=> blue
#[macro_export]
macro_rules! controller_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<b>[Controller]   </b>: {}", format!($($arg)*))
    }};
}

// task <=> green
#[macro_export]
macro_rules! task_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<g>[Task]         </g>: {}", format!($($arg)*))
    }};
}

// welcome <=> magenta
#[macro_export]
macro_rules! welcome_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<m>[Welcome]      </m>: {}", format!($($arg)*))
    }};
}

// error <=> red
#[macro_export]
macro_rules! error_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<r>[Error]        </r>: {}", format!($($arg)*))
    }};
}

// warning <=> orange
#[macro_export]
macro_rules! warning_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<o>[Warning]      </o>: {}", format!($($arg)*))
    }};
}
