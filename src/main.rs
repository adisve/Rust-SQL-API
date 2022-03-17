#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv;

use std::io;
use std::env;
use std::process;
use regex::Regex;
use dotenv::dotenv;
use diesel::prelude::*;
use crate::models::User;
use diesel::MysqlConnection;

pub mod handler;
pub mod models;

/*--------------------------------------------------------------------*/

/* Main function for retrieving info regarding what the admin wishes 
   to do. First thing that happens is that we create a connection pool
   to our database pillow_db. Then it is only one long match case.    */

/*--------------------------------------------------------------------*/


fn main() 
{
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect(
        "A set URL is required. You can import it as '.env' inside the root package");
    let conn = &MysqlConnection::establish(&url).unwrap();
    let regex = Regex::new("[-+]?[ 0-9]*\\.?[0-9]+$").unwrap();
    println!("\nWelcome back Admin\n");
    loop
    {
        handler::display_menu(); //
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect(&handler::err_msg());

        if regex.is_match(&input.trim()){
            match &*input.trim() 
            {
                "0" => {
                    process::exit(1);
                },
                "1" => {
                    handler::insert_new_user(&conn); //
                },
                "2" => {
                    handler::show_user_by_id(&conn); //
                },
                "3" => {
                    handler::show_all_users(&conn); //
                },
                "4" => {
                    handler::delete_user_by_id(&conn); //
                },  
                "5" => {
                    handler::update_user_by_id(&conn); // 
                },
                "6" => {
                    handler::show_all_users_by_name(&conn); //
                },
                "7" => {
                    handler::show_phone_by_imei(&conn); //
                },
                "8" => {
                    handler::show_all_phones(&conn); //
                },
                "9" => {
                    handler::update_phone_by_imei(&conn); //
                },
                "10" => {
                    handler::insert_new_phone(&conn); //
                },
                "11" => {
                    handler::delete_phone_by_imei(&conn); //
                },
                "12" => {
                    handler::show_all_phones_by_brand(&conn); //
                },
                "13" => {
                    handler::show_schedule_by_id(&conn); //
                },
                "14" => {
                    handler::show_all_schedules(&conn); //
                },
                "15" => {
                    handler::update_schedule_by_id(&conn); //
                },
                "16" => {
                    handler::insert_new_schedule(&conn); //
                },
                "17" => {
                    handler::delete_schedule_by_id(&conn); //
                },
                _ => {
                    println!("{}", &handler::err_msg())
                }
            }
        }
        input.clear();
    }
}

