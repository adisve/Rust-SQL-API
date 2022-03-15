use std::io;
use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenv::dotenv;

/*--------------------------------------------------------------------*/

/* These are the functions that act as a gap between our ORM and the
   user input. The user enters one of several option in main.rs and
   ends up executing one of these functions. */

/*--------------------------------------------------------------------*/

pub fn err_msg() -> String
{
    return String::from("Something went wrong.\nMake sure your input is correct.");
}

pub fn display_menu()
{
    println!("---------------------------------------------\n");
    println!("1. Insert new user\t2. Show a user\n");
    println!("3. Show all users\t4. Delete a user\n");
    println!("5. Update user\t       6. Show all users by name\n");
    println!("---------------------------------------------\n");
    println!("7. Show phone by IMEI(15 digits)\t8. Show all phones\n");
    println!("9. Update phone by IMEI\t        10. Insert new phone\n");
    println!("11. Delete phone by IMEI\t12. Show all phones by brand\n");
    println!("---------------------------------------------\n");
    println!("13. Show schedule by id\t    14. Show all schedules\n");
    println!("15. Update by schedule id\t16. Insert schedule\n");
    println!("17. Delete by schedule id\t\n");
    println!("---------------------------------------------\n");

}

pub fn add_new_user(conn: &&MysqlConnection)
{
    let email = rprompt::prompt_reply_stdout("\nEmail: ").unwrap();
    let pass = rprompt::prompt_reply_stdout("\nPassword: ").unwrap();
    let name = rprompt::prompt_reply_stdout("\nName: ").unwrap();

    let user = crate::models::NewUser {
        email : email,
        password : pass,
        name : name,
    };
    if crate::models::User::insert(user, &conn){
        println!("Successful insertion");
    }else{
        println!("Failed to insert user");
    }
                
}

pub fn show_all_users(conn: &&MysqlConnection)
{
    let users = crate::models::User::all(&conn);
    for user in users{
        println!("User id : {}", user.user_id);
        println!("\t");
        println!("Name : {}", user.name);
        println!("\t");
        println!("Email : {}", user.email);
        println!("\t");
        println!("Password : {}", user.password);
        println!("\t");
        println!("-------------------------\n");
    }
}

pub fn show_user(conn: &&MysqlConnection)
{
    let input = rprompt::prompt_reply_stdout("\nUser id: ").unwrap();
    let id: i32  = input.parse().unwrap();
    let users = crate::models::User::show(id, &conn);
    if !users.is_empty(){
        for user in users 
        {
            println!("\nUser id : {}", user.user_id);
            println!("\nName : {}", user.name);
            println!("\nEmail : {}", user.email);
            println!("\nPassword : {}", user.password);
        }
    }else{
        println!("\nEmpty set");
    }
}

pub fn insert_phone(conn: &&MysqlConnection)
{
    let imei = rprompt::prompt_reply_stdout("\nIMEI : ").unwrap();
    let uuid = rprompt::prompt_reply_stdout("\nUUID : ").unwrap();
    let mac = rprompt::prompt_reply_stdout("\nMAC : ").unwrap();
    let brand = rprompt::prompt_reply_stdout("\nBrand : ").unwrap();
    let model = rprompt::prompt_reply_stdout("\nModel : ").unwrap();
    let manufacturer = rprompt::prompt_reply_stdout("\nManufacturer : ").unwrap();
    let user_id = rprompt::prompt_reply_stdout("\nUser id : ").unwrap();
    
    let phone = crate::models::NewPhone {
        imei : imei,
        uuid : uuid,
        mac : mac,
        brand : brand,
        model : model,
        manufacturer : manufacturer,
        user_id : user_id.parse::<i32>().unwrap()
    };

    if crate::models::Phone::insert(phone, &conn){
        println!("Successful insertion");
    }else{
        println!("Failed to insert phone");
    }
}