use babyrs::establish_connection;
use diesel::prelude::*;

#[test]
fn test_establish_connection() {
    std::env::set_var("DATABASE_URL", ":memory:");

    let connection = &mut establish_connection();

    assert!(diesel::sql_query("SELECT 1").execute(connection).is_ok());
}
