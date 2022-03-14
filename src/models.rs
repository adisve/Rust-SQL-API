#![allow(dead_code)]
#![allow(unused_imports)]
use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;

use std::string::String;
use crate::schema::user;
use crate::schema::user::dsl::user as all_users;
use crate::schema::phone;
use crate::schema::phone::dsl::phone as all_phones;
use crate::schema::schedule;
use crate::schema::schedule::dsl::schedule as all_schedules;

/*--------------------------------------------------------------------*/

/* Difference between these structs and 'New' structs is that these structs query
   existing tables from the Database and those annotated with 'New' are meant to allow us
   to insert new entities */

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
    pub schedule_name : String,
    pub alarm_date : String,
    pub user_id : i32
}

/*--------------------------------------------------------------------*/

/* The Queryable annotation will generate all of the code needed
   to load a Post struct from a SQL query. Naturally the below
   annotation 'Insertable' does the same but for Inserts. */

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
    pub imei : i32,
    pub uuid : String,
    pub mac : String,
    pub brand : String,
    pub model : String,
    pub manufacturer : String,
    pub user_id : i32
}

#[derive(Insertable)]
#[table_name = "schedule"]
pub struct NewSchedule{
    pub alarm_date : String,
    pub user_id : i32,
    pub schedule_name : String
}


/*--------------------------------------------------------------------*/

/* These are the methods our API uses to interact with the database */

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
        use crate::schema::user::dsl::{email as e, password as p, name as n};
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