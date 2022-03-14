#![allow(dead_code)]
#![allow(unused_imports)]
table! {
    bluetooth_module (module_id, pillow_id) {
        module_id -> Integer,
        mac -> Nullable<Varchar>,
        uuid -> Nullable<Varchar>,
        pillow_id -> Integer,
    }
}

table! {
    phone (imei) {
        imei -> Integer,
        mac -> Nullable<Varchar>,
        uuid -> Nullable<Varchar>,
        brand -> Nullable<Varchar>,
        manufacturer -> Nullable<Varchar>,
        model -> Nullable<Varchar>,
        user_id -> Integer,
    }
}

table! {
    pillow (pillow_id) {
        pillow_id -> Integer,
        model -> Nullable<Varchar>,
        product_number -> Nullable<Integer>,
        user_id -> Integer,
    }
}

table! {
    schedule (schedule_id) {
        schedule_id -> Integer,
        alarm_date -> Nullable<Varchar>,
        schedule_name -> Nullable<Varchar>,
        user_id -> Integer,
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
