use web_sys::console;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn console_log(msg: &str) {
    console::log_1(&msg.into());
}
