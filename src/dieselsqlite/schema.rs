//GENERATED AUTOMATICALLY BY DIESEL CLI

diesel::table! {
    block_storage_mode (rowid) {
        rowid -> Integer,
        legacy -> Integer,
    }
}

diesel::table! {
    delayed_transactions (rowid) {
        rowid -> Integer,
        injected_before -> Integer,
        hash -> Binary,
        payload -> Binary,
    }
}

diesel::table! {
    irmin_chunks (rowid) {
        rowid -> Integer,
        level -> Integer,
        timestamp -> Integer,
    }
}

diesel::table! {
    kernel_upgrades (rowid) {
        rowid -> Integer,
        injected_before -> Integer,
        root_hash -> Binary,
        activation_timestamp -> Integer,
        applied_before -> Nullable<Integer>,
    }
}

diesel::table! {
    l1_l2_finalized_levels (l1_level) {
        l1_level -> Integer,
        start_l2_level -> Integer,
        end_l2_level -> Integer,
    }
}

diesel::table! {
    l1_l2_levels_relationships (latest_l2_level) {
        latest_l2_level -> Integer,
        l1_level -> Integer,
    }
}

diesel::table! {
    metadata (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    sequencer_upgrades (rowid) {
        rowid -> Integer,
        injected_before -> Integer,
        sequencer -> Binary,
        pool_address -> Binary,
        activation_timestamp -> Integer,
        applied_before -> Nullable<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    block_storage_mode,
    delayed_transactions,
    irmin_chunks,
    kernel_upgrades,
    l1_l2_finalized_levels,
    metadata,
    sequencer_upgrades,
);

//WRITTEN MANUALLY

diesel::table! {
    blocks (level) {
        level -> Integer,
        hash -> Binary,
        block-> Binary,
    }
}

diesel::table! {
    blueprints (id) {
        id -> Integer,
        payload -> Binary,
        timestamp -> Integer,
    }
}

diesel::table! {
    context_hashes (id) {
        id -> Integer,
        context_hash -> Binary,
    }
}

diesel::table! {
    migrations (id){
        id->Integer,
        name->Nullable<Text>,
    }
}

diesel::table! {
    pending_confirmations (level){
        level->Integer,
        hash->Binary,
    }
}

diesel::table! {
    transactions (hash){
        block_hash ->Binary ,
        block_number->Integer,
        index_ ->Integer ,
        hash ->Binary,
        from_ ->Binary,
        to_ ->Nullable<Binary>,
        receipt_fields ->Binary,
        object_fields-> Binary,
    }
}

diesel::table! {
    sqlite_schema (row_id){
        row_id->Integer,
        #[sql_name = "type"]
        schema_type->Text,
        name->Text,
        tbl_name->Text,
        rootpage->Integer,
        sql->Text,
    }
}
