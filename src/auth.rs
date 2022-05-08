use std::io;

pub mod auth_services {
    use super::*;

    pub fn get_user_name() -> String {
        println!("Enter your name  ");
        let mut user_name = String::new();
        io::stdin().read_line(&mut user_name).unwrap();

        if user_name.trim().chars().count() < 2 {
            user_name.clear();
            get_user_name();
        }

        user_name.to_string()
    }
}
