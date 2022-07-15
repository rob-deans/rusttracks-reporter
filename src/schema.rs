table! {
    location (tst, tid) {
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
