#[derive(Debug)]
pub struct AppConfig {
    pub theme: String,
    pub notifications_enabled: bool,
    pub max_users: u32,
    pub auto_save: bool,
    pub cache_size_mb: u32,
    pub log_level: String,
    pub retry_attempts: u32,
    pub timeout_seconds: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "Light".to_string(),
            notifications_enabled: true,
            max_users: 100,
            auto_save: true,
            cache_size_mb: 512,
            log_level: "INFO".to_string(),
            retry_attempts: 3,
            timeout_seconds: 30,
        }
    }
}

// Example usage
pub fn main() {
    // Create a default configuration
    let default_config = AppConfig::default();
    println!("Default Config: {:?}", default_config);

    // Create a custom configuration using ..Default::default()
    let custom_config = AppConfig {
        theme: String::from("Dark"),
        ..Default::default()
    };
    println!("Custom Config: {:?}", custom_config);
}
