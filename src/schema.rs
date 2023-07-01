// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Integer,
        dt -> Timestamp,
        urine -> Bool,
        stool -> Bool,
        skin2skin -> Integer,
        breastfeed -> Integer,
        breastmilk -> Integer,
        formula -> Integer,
        pump -> Integer,
    }
}
