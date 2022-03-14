#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenv::dotenv;
use std::env;

mod schema;
mod models;

fn main() {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("set URL");
    let conn = &MysqlConnection::establish(&url).unwrap();

    let user = models::NewUser {
        email : String::from("adis.veletanlic@gmail.com"),
        password : String::from("P@sSw0r_D"),
        name : String::from("Adis Veletanlic"),
    };

    if models::User::insert(user, &conn){
        println!("Successful insertion");
    }else{
        println!("Failed to insert user");
    }
}