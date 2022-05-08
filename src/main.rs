mod auth;
mod logic;
use auth::auth_services::{self};
use logic::functionality;

fn main() {
    let welcome_message = format!("Welcome to  Word Chaser");
    println!("                 {}", welcome_message);
    let user_name = auth_services::get_user_name();
    println!("Welcome >>> {}", user_name);
    functionality::play();
    println!("Welcome again {}", user_name)
}
