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
        value_tpye -> Text,
        key -> Text,
        desc -> Nullable<Text>,
        options -> Text,
        value -> Text,
        usage -> Text,
        device_id -> Integer,
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

allow_tables_to_appear_in_same_query!(
    device,
    group,
    param,
    room,
);
