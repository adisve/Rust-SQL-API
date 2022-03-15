/*--------------------------------------------------------------------*/

/* These are the table macros for our ORM, they represent the database
   schemas tables and rows.                                           */

/*--------------------------------------------------------------------*/

table! {
    bluetooth_module (module_id, pillow_id) {
        mac -> Varchar,
        uuid -> Nullable<Varchar>,
        module_id -> Integer,
        pillow_id -> Integer,
    }
}

table! {
    phone (imei) {
        imei -> Char,
        uuid -> Char,
        mac -> Char,
        brand -> Varchar,
        model -> Varchar,
        manufacturer -> Varchar,
        user_id -> Integer,
    }
}

table! {
    pillow (pillow_id) {
        pillow_id -> Integer,
        model -> Nullable<Varchar>,
        user_id -> Integer,
        version_number -> Nullable<Float>,
    }
}

table! {
    schedule (schedule_id) {
        schedule_id -> Integer,
        alarm_date -> Nullable<Varchar>,
        user_id -> Integer,
        schedule_name -> Nullable<Varchar>,
    }
}

table! {
    theme (theme_id, schedule_id) {
        theme_id -> Integer,
        color -> Nullable<Varchar>,
        pattern -> Nullable<Varchar>,
        movement -> Nullable<Varchar>,
        speed -> Nullable<Integer>,
        schedule_id -> Integer,
    }
}

table! {
    user (user_id) {
        user_id -> Integer,
        email -> Varchar,
        password -> Varchar,
        name -> Varchar,
    }
}

joinable!(bluetooth_module -> pillow (pillow_id));
joinable!(phone -> user (user_id));
joinable!(pillow -> user (user_id));
joinable!(schedule -> user (user_id));
joinable!(theme -> schedule (schedule_id));

allow_tables_to_appear_in_same_query!(
    bluetooth_module,
    phone,
    pillow,
    schedule,
    theme,
    user,
);
