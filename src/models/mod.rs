/*--------------------------------------------------------------------*/

/* This is an ORM made to retrieve entities from the pillow_db database
   and map them into usable Structs defined in the program.           */

/*--------------------------------------------------------------------*/

#![allow(dead_code)]
#![allow(unused_imports)]
use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;

use std::string::String;
use schema::user;
use schema::user::dsl::user as all_users;
use schema::phone;
use schema::phone::dsl::phone as all_phones;
use schema::schedule;
use schema::schedule::dsl::schedule as all_schedules;

mod schema;

/*--------------------------------------------------------------------*/

/* Difference between these structs and 'New' structs is that these structs query
   existing tables from the Database and those annotated with 'New' are meant to allow us
   to insert new entities                                             */

/*--------------------------------------------------------------------*/

#[derive(Queryable)]
pub struct User{
    pub user_id : i32,
    pub email : String,
    pub password : String,
    pub name : String
}

#[derive(Queryable)]
pub struct Phone{
    pub imei : String,
    pub uuid : String,
    pub mac : String,
    pub brand : String,
    pub model : String,
    pub manufacturer : String,
    pub user_id : i32
}

#[derive(Queryable)]
pub struct Schedule{
    pub schedule_id : i32,
    pub alarm_date : Option<String>,
    pub user_id : i32,
    pub schedule_name : Option<String>
}

/*--------------------------------------------------------------------*/

/* The Queryable annotation will generate all of the code needed
   to load a Post struct from a SQL query. Naturally the below
   annotation 'Insertable' does the same but for Inserts.             */

/*--------------------------------------------------------------------*/

/* Insertable trait tells  */
#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser{
    pub email : String,
    pub password : String,
    pub name : String
}

#[derive(Insertable)]
#[table_name = "phone"]
pub struct NewPhone{
    pub imei : String,
    pub mac : String,
    pub uuid : String,
    pub brand : String,
    pub manufacturer : String,
    pub model : String,
    pub user_id : i32
}

#[derive(Insertable)]
#[table_name = "schedule"]
pub struct NewSchedule{
    pub schedule_id : i32,
    pub alarm_date : Option<String>,
    pub user_id : i32,
    pub schedule_name : Option<String>
}

/*--------------------------------------------------------------------*/

/* These are the methods our API uses to interact with the database   */

/*--------------------------------------------------------------------*/

impl User {
    pub fn show(user_id: i32, conn: &MysqlConnection
    ) -> Vec<User> {
        /* We perform a few functions on our all_users item brougth in
           from our schema. First we run the find() method with the user's
           name as a parameter, then the load() function to cast it into the
           previously created User struct and finally throw an error message
           if it is not found. */
        all_users
            .find(user_id)
            .load::<User>(conn)
            .expect("Error loading user")
    }

    pub fn all(conn: &MysqlConnection
    ) -> Vec<User>{
        all_users
            .order(user::user_id.desc())
            .load::<User>(conn)
            .expect("Error loading users")
    }

    pub fn update_by_name(user_id: i32, conn: MysqlConnection, user: NewUser)
     -> bool {
        use crate::models::schema::user::dsl::{email as e, password as p, name as n};
        let NewUser {
            email,
            password,
            name,
        } = user;

        diesel::update(all_users.find(user_id))
            .set((e.eq(email), p.eq(password), n.eq(name)))
            .execute(&conn)
            .is_ok()
    }

    pub fn insert(user: NewUser, conn: &MysqlConnection)
     -> bool {
        diesel::insert_into(user::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_user_id(user_id: i32, conn: &MysqlConnection)
     -> bool {
        if User::show(user_id, conn).is_empty() {
            return false;
        }
        diesel::delete(all_users.find(user_id)).execute(conn).is_ok()
    }

    pub fn all_by_name(name: String, conn: &MysqlConnection)
     -> Vec<User> {
        all_users
            .filter(user::name.eq(name))
            .load::<User>(conn)
            .expect("Error loading users")
    }
}


impl Phone {
    pub fn show(imei: String, conn: &MysqlConnection
    ) -> Vec<Phone> {
        all_phones
            .find(imei)
            .load::<Phone>(conn)
            .expect("Error loading phone")
    }

    pub fn all(conn: &MysqlConnection
    ) -> Vec<Phone>{
        all_phones
            .order(phone::imei.desc())
            .load::<Phone>(conn)
            .expect("Error loading phones")
    }

    pub fn update_by_imei(input_imei: String, conn: MysqlConnection, phone: NewPhone)
     -> bool {
        use crate::models::schema::phone::dsl::{
            imei as i, uuid as u,
                mac as m, brand as b, model as mo, 
                    manufacturer as man, user_id as usd};
        let NewPhone {
            imei,
            uuid,
            mac,
            brand,
            model,
            manufacturer,
            user_id
        } = phone;

        diesel::update(all_phones.find(input_imei))
            .set((i.eq(imei), u.eq(uuid), m.eq(mac), b.eq(brand), mo.eq(model),
                man.eq(manufacturer), usd.eq(user_id)))
            .execute(&conn)
            .is_ok()
    }

    pub fn insert(phone: NewPhone, conn: &MysqlConnection)
     -> bool {
        diesel::insert_into(phone::table)
            .values(&phone)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_imei(imei: String, conn: &MysqlConnection)
     -> bool {
        if Phone::show(imei.to_string(), conn).is_empty() {
            return false;
        }
        diesel::delete(all_phones.find(imei.to_string())).execute(conn).is_ok()
    }

    pub fn all_by_brand(brand: String, conn: &MysqlConnection)
     -> Vec<Phone> {
        all_phones
            .filter(phone::brand.eq(brand))
            .load::<Phone>(conn)
            .expect("Error loading users")
    }
}

impl Schedule {
    pub fn show(schedule_id: i32, conn: &MysqlConnection
    ) -> Vec<Schedule> {
        all_schedules
            .find(schedule_id)
            .load::<Schedule>(conn)
            .expect("Error loading schedule")
    }

    pub fn all(conn: &MysqlConnection
    ) -> Vec<Schedule>{
        all_schedules
            .order(schedule::schedule_id.desc())
            .load::<Schedule>(conn)
            .expect("Error loading schedules")
    }

    pub fn update_by_schedule_id(schedule_id: i32, conn: MysqlConnection, schedule: NewSchedule)
     -> bool {
        use crate::models::schema::schedule::dsl::{alarm_date as a, schedule_name as s};
        let NewSchedule {
            schedule_id,
            alarm_date,
            user_id,
            schedule_name
        } = schedule;

        diesel::update(all_schedules.find(schedule_id))
            .set((a.eq(alarm_date), s.eq(schedule_name)))
            .execute(&conn)
            .is_ok()
    }

    pub fn insert(schedule: NewSchedule, conn: &MysqlConnection)
     -> bool {
        diesel::insert_into(schedule::table)
            .values(&schedule)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_schedule_id(schedule_id: i32, conn: &MysqlConnection)
     -> bool {
        if Schedule::show(schedule_id, conn).is_empty() {
            return false;
        }
        diesel::delete(all_schedules.find(schedule_id)).execute(conn).is_ok()
    }
}
