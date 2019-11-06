table! {
    events (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        cover_img -> Nullable<Varchar>,
        address -> Varchar,
        loc_name -> Nullable<Varchar>,
        lat -> Float8,
        long -> Float8,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        profile_img -> Nullable<Varchar>,
        cover_img -> Nullable<Varchar>,
    }
}

table! {
    memberships (id) {
        id -> Int4,
        user_id -> Int4,
        group_id -> Int4,
        membership_role -> Varchar,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        body -> Varchar,
        created_at -> Timestamp,
        user_id -> Int4,
        group_id -> Int4,
        event_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        fullname -> Varchar,
        email -> Varchar,
        profile_img -> Nullable<Varchar>,
    }
}

joinable!(memberships -> groups (group_id));
joinable!(memberships -> users (user_id));
joinable!(posts -> events (event_id));
joinable!(posts -> groups (group_id));
joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    groups,
    memberships,
    posts,
    users,
);
