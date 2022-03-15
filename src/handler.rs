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
    println!("17. Delete by schedule id\t0. EXIT\n");
    println!("---------------------------------------------\n");

}

pub fn show_user_by_id(conn: &&MysqlConnection)
{
    let input = rprompt::prompt_reply_stdout("\nUser id: ").unwrap();
    let id: i32  = input.parse().unwrap();
    let users = crate::models::User::show(id, &conn);
    if !users.is_empty(){
        println!("\nUser id : {}", users[0].user_id);
        println!("\nName : {}", users[0].name);
        println!("\nEmail : {}", users[0].email);
        println!("\nPassword : {}", users[0].password);
    
    }else{
        println!("\nEmpty set");
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

pub fn update_user_by_id(conn: &&MysqlConnection)
{
    let user_id = rprompt::prompt_reply_stdout("\nUser id : ").unwrap();
    let email = rprompt::prompt_reply_stdout("\nEmail: ").unwrap();
    let pass = rprompt::prompt_reply_stdout("\nPassword: ").unwrap();
    let name = rprompt::prompt_reply_stdout("\nName: ").unwrap();

    let user = crate::models::NewUser {
        email : email,
        password : pass,
        name : name,
    };
    if crate::models::User::update(user_id.parse::<i32>().unwrap(), &conn, user){
        println!("Successful update");
    }else{
        println!("Failed to insert user");
    }

}

pub fn insert_new_user(conn: &&MysqlConnection)
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

pub fn delete_user_by_id(conn: &&MysqlConnection)
{
    let user_id = rprompt::prompt_reply_stdout("\nUser id : ").unwrap();
    if crate::models::User::delete(user_id.parse::<i32>().unwrap(), &conn)
    {
        println!("Successful deletion");
    }else{
        println!("Failed to insert phone");
    }
}

pub fn show_all_users_by_name(conn: &&MysqlConnection)
{
    let name = rprompt::prompt_reply_stdout("\nName : ").unwrap();
    let users = crate::models::User::all_by_name(name, &conn);
}


/*-----------------------------------------------------------------------------------*/


pub fn show_phone_by_imei(conn: &&MysqlConnection)
{
    let imei = rprompt::prompt_reply_stdout("\nIMEI : ").unwrap();
    let phones = crate::models::Phone::show(imei, &conn);
    if !phones.is_empty(){
        
        println!("\nIMEI : {}", phones[0].imei);
        println!("\nUUID : {}", phones[0].uuid);
        println!("\nMAC : {}", phones[0].mac);
        println!("\nBrand : {}", phones[0].brand);
        println!("\nModel : {}", phones[0].model);
        println!("\nManufacturer : {}", phones[0].manufacturer);
        println!("\nUser Id : {}", phones[0].user_id);
    
    }else{
        println!("\nEmpty set");
    }
}

pub fn show_all_phones(conn: &&MysqlConnection)
{
    let phones = crate::models::Phone::all(&conn);
    if !phones.is_empty(){
        for phone in phones 
        {
            println!("\nIMEI : {}", phone.imei);
            println!("\nUUID : {}", phone.uuid);
            println!("\nMAC : {}", phone.mac);
            println!("\nBrand : {}", phone.brand);
            println!("\nModel : {}", phone.model);
            println!("\nManufacturer : {}", phone.manufacturer);
            println!("\nUser Id : {}", phone.user_id);
            println!("-----------------------------------------");
        }
    }else{
        println!("\nEmpty set");
    }
}

pub fn update_phone_by_imei(conn: &&MysqlConnection)
{
    /* Only time you would update a phone table row is 
       when the user gets a new phone, so everything
       has to change except for the user_id */
    let search_imei = rprompt::prompt_reply_stdout("\nIMEI Key : ").unwrap();

    let imei = rprompt::prompt_reply_stdout("\nNew IMEI : ").unwrap();
    let uuid = rprompt::prompt_reply_stdout("\nNew UUID : ").unwrap();
    let mac = rprompt::prompt_reply_stdout("\nNew MAC : ").unwrap();
    let brand = rprompt::prompt_reply_stdout("\nNew Brand : ").unwrap();
    let model = rprompt::prompt_reply_stdout("\nNew Model : ").unwrap();
    let manufacturer = rprompt::prompt_reply_stdout("\nNew Manufacturer : ").unwrap();

    let phone = crate::models::UpdatePhone {
        imei : imei,
        uuid : uuid,
        mac : mac,
        brand : brand,
        model : model,
        manufacturer : manufacturer,
    };
    if crate::models::Phone::update(search_imei, &conn, phone){
        println!("Successful update");
    }else{
        println!("Failed to update phone");
    }
}

pub fn insert_new_phone(conn: &&MysqlConnection)
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

pub fn delete_phone_by_imei(conn: &&MysqlConnection)
{
    let imei = rprompt::prompt_reply_stdout("\nIMEI : ").unwrap();
    if crate::models::Phone::delete(imei, &conn){
        println!("Successful deletion");
    }else{
        println!("Failed to delete phone");
    }
}

pub fn show_all_phones_by_brand(conn: &&MysqlConnection)
{
    let brand = rprompt::prompt_reply_stdout("\nBrand : ").unwrap();
    let phones = crate::models::Phone::all_by_brand(brand, &conn);
    if !phones.is_empty()
    {
        for phone in phones
        {
            println!("\nIMEI : {}", phone.imei);
            println!("\nUUID : {}", phone.uuid);
            println!("\nMAC : {}", phone.mac);
            println!("\nBrand : {}", phone.brand);
            println!("\nModel : {}", phone.model);
            println!("\nManufacturer : {}", phone.manufacturer);
            println!("\nUser Id : {}", phone.user_id);
            println!("-----------------------------------------");
        }
    }else
    {
        println!("Empty set");
    }
}

pub fn show_schedule_by_id(conn: &&MysqlConnection)
{
    let schedule_id = rprompt::prompt_reply_stdout("\nSchedule Id : ").unwrap();
    let schedules = crate::models::Schedule::show(schedule_id.parse::<i32>().unwrap(), &conn);
    if !schedules.is_empty()
    {
        println!("\nSchedule id : {}", schedules[0].schedule_id);
        println!("\nAlarm date : {:?}", schedules[0].alarm_date);
        println!("\nUser Id : {:?}", schedules[0].user_id);
        println!("\nSchedule name : {:?}", schedules[0].schedule_name);

    }else
    {
        println!("Empty set");
    }
}

pub fn show_all_schedules(conn: &&MysqlConnection)
{
    let schedules = crate::models::Schedule::all(&conn);
    if !schedules.is_empty()
    {
        println!("\nSchedule id : {}", schedules[0].schedule_id);
        println!("\nAlarm date : {:?}", schedules[0].alarm_date);
        println!("\nUser Id : {:?}", schedules[0].user_id);
        println!("\nSchedule name : {:?}", schedules[0].schedule_name);
    }else
    {
        println!("Empty set");
    }
}

pub fn update_schedule_by_id(conn: &&MysqlConnection)
{
    let schedule_id = rprompt::prompt_reply_stdout("\nSchedule Id : ").unwrap();

    let alarm_date = rprompt::prompt_reply_stdout(
        "\nNew alarm date (Format -> DAY MON DD YYYY HH:MM:SS GMT+0000) : ").unwrap();
    let schedule_name = rprompt::prompt_reply_stdout("\nNew schedule name : ").unwrap();

    let new_schedule = crate::models::UpdateSchedule {
        schedule_name : Some(schedule_name),
        alarm_date : Some(alarm_date)
    };
    if crate::models::Schedule::update(schedule_id.parse::<i32>().unwrap(), &conn, new_schedule){
        println!("Successful update");
    }else
    {
        println!("\nSomething went wrong");
    }
}

pub fn insert_new_schedule(conn: &&MysqlConnection)
{
    let schedule_name = rprompt::prompt_reply_stdout("\nSchedule name : ").unwrap();
    let alarm_date = rprompt::prompt_reply_stdout(
        "\nAlarm date (Format -> DAY MON DD YYYY HH:MM:SS GMT+0000) : ").unwrap();
    let user_id = rprompt::prompt_reply_stdout("\nUser Id : ").unwrap();

    let schedule = crate::models::NewSchedule {
        schedule_name : Some(schedule_name),
        alarm_date : Some(alarm_date),
        user_id : user_id.parse::<i32>().unwrap(),
    };
    if crate::models::Schedule::insert(schedule, &conn){
        println!("Successful insertion");
    }else{
        println!("Failed to insert schedule");
    }
}

pub fn delete_schedule_by_id(conn: &&MysqlConnection)
{
    let schedule_id = rprompt::prompt_reply_stdout("\nSchedule id : ").unwrap();
    if crate::models::Schedule::delete(schedule_id.parse::<i32>().unwrap(), &conn)
    {
        println!("Successful deletion");
    }else{
        println!("Failed to insert schedule");
    }
}