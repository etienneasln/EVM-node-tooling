diesel::table!{
    blueprints (id) {
        id -> Integer,
        payload -> Binary,
        timestamp -> Integer,
    }
}

