pub static SESSION: &str = "XDG_SESSION_TYPE";
pub static ENV_CONFIG: &str = "PAPERPASS_CONFIG";
pub fn set_env(path: &str) {
    unsafe {
        std::env::set_var(ENV_CONFIG, path);
    }
}
