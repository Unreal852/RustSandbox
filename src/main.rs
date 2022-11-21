mod user;

use std::io::{stdin, stdout, Write};
use uuid::Uuid;
use crate::user::User;

fn main()
{
    let mut user = String::new();

    ask_input(&mut user, String::from("Enter your username: "));

    let user = User{uuid: Uuid::new_v4(), username: user};

    println!("Welcome {}", user);

    let mut first_number = String::new();
    let mut second_number = String::new();

    ask_input(&mut first_number, String::from("Enter number: "));
    ask_input(&mut second_number, String::from("Enter number: "));

    let first_number = first_number.trim().parse::<i32>().unwrap();
    let second_number = second_number.trim().parse::<i32>().unwrap();

    let result = add(first_number, second_number);

    println!("The result of {} + {} is {}", first_number, second_number, result);
}

fn ask_input(_buf: &mut String, _msg: String)
{
    print(_msg);
    stdin().read_line(_buf).expect("Invalid input");
}

fn print(_str: String)
{
    print!("{}", _str);
    stdout().flush().unwrap();
}

fn add(left: i32, right: i32) -> i32 {
    return left + right;
}
