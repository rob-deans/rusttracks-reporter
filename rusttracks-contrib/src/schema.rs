table! {
    location (tst, tid) {
        tst -> BigInt,
        lat -> Double,
        lon -> Double,
        acc -> Nullable<Integer>,
        alt -> Nullable<Integer>,
        vac -> Nullable<Integer>,
        batt -> Nullable<Integer>,
        tid -> Text,
        vel -> Nullable<Integer>,
        created_at -> Integer,
    }
}

table! {
    location_old (tst, tid) {
        tst -> Integer,
        lat -> Double,
        lon -> Double,
        acc -> Nullable<Integer>,
        alt -> Nullable<Integer>,
        vac -> Nullable<Integer>,
        batt -> Nullable<Integer>,
        tid -> Text,
        vel -> Nullable<Integer>,
        created_at -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    location,
    location_old,
);
