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
pub mod query_routes;

fn main() {
    dotenv().ok();

    let url = env::var("DATABASE_URL").expect("set URL");
    let conn = &MysqlConnection::establish(&url).unwrap();

    let result = models::User::show(1, &conn);

    for user in result{
        println!("{}", user.name);
        println!("----------\n");
        println!("{}", user.email);
        println!("----------\n");
        println!("{}", user.password);
    }


    let user = models::NewUser {
        email : String::from("adis.veletanlic@gmail.com"),
        password : String::from("P@sSw0r_D"),
        name : String::from("Adis Veletanlic"),
    };

    if query_routes::User::insert(user, &conn){
        println!("Successful insertion");
    }else{
        println!("Failed to insert user");
    }
}