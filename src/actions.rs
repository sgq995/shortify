use diesel::{expression_methods::ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use crate::models;

pub fn select_url(conn: &mut SqliteConnection, hash: String) -> diesel::QueryResult<models::Url> {
    use crate::schema::urls::dsl;

    dsl::urls
        .filter(dsl::hash.eq(&hash))
        .first::<models::Url>(conn)
}

pub fn insert_url(
    conn: &mut SqliteConnection,
    hash: String,
    url: String,
) -> diesel::QueryResult<models::Url> {
    use crate::schema::urls::dsl;

    let created_url = match select_url(conn, hash.clone()) {
        Ok(result) => result,
        _ => {
            let new_url = models::CreateUrl {
                hash: &hash,
                url: &url,
            };

            diesel::insert_into(dsl::urls)
                .values(new_url)
                .execute(conn)
                .expect("Error inserting url");

            models::Url {
                id: 0,
                hash: hash,
                url: url,
            }
        }
    };

    Ok(created_url)
}
