pub fn get_database_url() -> String {
    match std::env::var("DATABASE_URL") {
        Ok(val) => {
            if !val.starts_with("postgresql://") {
                panic!("DATABASE_URL must start with 'postgresql://'");
            }

            val
        }
        Err(_) => {
            panic!("DATABASE_URL environment variable is not set.")
        }
    }
}

pub fn main() {
    std::env::set_var("DATABASE_URL", "postgresql://localhost");

    let db_url = get_database_url();
    println!("Database URL: {}", db_url);

    std::env::remove_var("DATABASE_URL");
    get_database_url();

    std::env::set_var("DATABASE_URL", "mysql://localhost");
    get_database_url();
}
