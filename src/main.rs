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

pub mod handler;
pub mod models;

/*--------------------------------------------------------------------*/

/* Main function for retrieving info regarding what the admin wishes 
   to do. First thing that happens is that we create a connection pool
   to our database pillow_db. Then it is only one long match case. */

/*--------------------------------------------------------------------*/


fn main() 
{

    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("set URL");
    let conn = &MysqlConnection::establish(&url).unwrap();


    println!("\nWelcome back Admin\n");
    loop
    {
        handler::display_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(&handler::err_msg());

        match &*input.trim() 
        {
            "1" => {
                handler::add_new_user(&conn);
            },
            "2" => {
                handler::show_user(&conn);
            },
            "3" => {
                handler::show_all_users(&conn);
            },
            "4" => {
            },
            "6" => {
                break;
            },
            "7" => {
                break;
            },
            "8" => {
                break;
            },
            "9" => {
                break;
            },
            "10" => {
                break;
            },
            "11" => {
                break;
            },
            "12" => {
                break;
            },
            "13" => {
                break;
            },
            "14" => {
                break;
            },
            "15" => {
                break;
            },
            "16" => {
                break;
            },
            "17" => {
                break;
            },
            _ => {
                println!("{}", &handler::err_msg())
            }
        }
        input.clear();
    }
}

