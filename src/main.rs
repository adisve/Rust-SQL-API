#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use std::io;
use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenv::dotenv;
use crate::models::User;
use std::env;

mod schema;
mod models;

fn main() 
{
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("set URL");
    let conn = &MysqlConnection::establish(&url).unwrap();
    println!("Welcome back Admin\n");
    loop
    {
        display_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Faulty input");

        match &*input.trim() 
        {
            "1" => {
                add_new_user(&conn);
            },

            "2" => {
                show_user(&conn);
            },

            "3" => {
                show_all_users(&conn);
            }
            "10" => {
                break;
            }

            _ => {
                println!("Something went wrong.")
            }
        }
        input.clear();
    }
}

fn err_msg() -> String
{
    return String::from("Something went wrong.\nMake sure your input is correct.");
}

fn display_menu()
{
    println!("----------\n");
    println!("1. Insert new user\t2. Show a user\n");
    println!("3. Show all users\t4. Delete a user\n");
}

fn add_new_user(conn: &&MysqlConnection)
{
    let email = rprompt::prompt_reply_stdout("\nEmail: ").unwrap();
    let pass = rprompt::prompt_reply_stdout("\nPassword: ").unwrap();
    let name = rprompt::prompt_reply_stdout("\nName: ").unwrap();

    let user = models::NewUser {
        email : email,
        password : pass,
        name : name,
    };
    if models::User::insert(user, &conn){
        println!("Successful insertion");
    }else{
        println!("Failed to insert user");
    }
                
}

fn show_all_users(conn: &&MysqlConnection)
{
    let users = models::User::all(&conn);
    for user in users{
        println!("User id : {}", user.user_id);
        println!("\t");
        println!("Name : {}", user.name);
        println!("\t");
        println!("Email : {}", user.email);
        println!("\t");
        println!("Password : {}", user.password);
        println!("\t");
        println!("-----------------\n");
    }
}

fn show_user(conn: &&MysqlConnection)
{
    let input = rprompt::prompt_reply_stdout("\nUser id: ").unwrap();
    let id: i32  = input.parse().unwrap();
    let users = models::User::show(id, &conn);
    if !users.is_empty(){
        for user in users 
        {
            println!("\nUser id : {}", user.user_id);
            println!("\nName : {}", user.name);
            println!("\nEmail : {}", user.email);
            println!("\nPassword : {}", user.password);
        }
    }else{
        println!("\nerror parsing output");
    }
}