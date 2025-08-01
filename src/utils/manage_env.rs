pub static ENV_CONFIG: &str = "PAPERPASS_CONFIG";
pub fn set_env(path: &str) {
    unsafe {
        std::env::set_var(ENV_CONFIG, path);
    }
}
