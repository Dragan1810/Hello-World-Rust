table! {
    factories (id) {
        id -> Uuid,
        level -> Int4,
        gold_per_day -> Int4,
        price -> Int4,
        name -> Varchar,
    }
}

table! {
    invitations (id) {
        id -> Uuid,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    player_factories (id) {
        id -> Uuid,
        user_id -> Uuid,
        factory_id -> Uuid,
        amount -> Int4,
    }
}

table! {
    players_data (id) {
        energy -> Int4,
        gold -> Int4,
        exp -> Int4,
        id -> Uuid,
        last_updated -> Timestamp,
        gold_acc -> Int4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password -> Varchar,
        created_on -> Timestamp,
        player_data_id -> Uuid,
    }
}

joinable!(player_factories -> factories (factory_id));
joinable!(player_factories -> users (user_id));
joinable!(users -> players_data (player_data_id));

allow_tables_to_appear_in_same_query!(
    factories,
    invitations,
    player_factories,
    players_data,
    users,
);
