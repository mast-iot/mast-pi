table! {
    device (id) {
        id -> Integer,
        name -> Text,
        desc -> Nullable<Text>,
        device_type -> Text,
        icon -> Nullable<Text>,
        room_id -> Integer,
        group_id -> Nullable<Integer>,
    }
}

table! {
    group (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    param (id) {
        id -> Integer,
        param_type -> Text,
        value_type -> Text,
        key -> Text,
        desc -> Nullable<Text>,
        options -> Text,
        value -> Text,
        usage -> Text,
        device_id -> Integer,
        in_id-> Nullable<Integer>,
        out_id-> Nullable<Integer>,
    }
}

table! {
    room (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        desc -> Nullable<Text>,
    }
}

table! {
    user(id){
        id -> Integer,
        name -> Text,
        mobile -> Text,
        image -> Nullable<Text>,
        password -> Text,
    }
}

table! {
    input(id){
        id->Integer,
        address->Integer,
    }
}

table! {
    output(id){
        id-> Integer,
        address->Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    device,
    group,
    param,
    room,
    user,
);
