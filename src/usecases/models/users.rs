use time::OffsetDateTime;

// #[derive(sqlx::FromRow, Debug)]         rename できぬ！
// pub struct Users {
//     #[sqlx(rename = "kaiin_id")]
//     pub id: i64,
//     #[sqlx(rename = "adana")]
//     pub name: String,
//     #[sqlx(rename = "mail_address")]
//     pub email: String,
//     pub password: String,
//     pub created_at: OffsetDateTime,
//     pub updated_at: OffsetDateTime,
// }

#[derive(sqlx::FromRow, Debug)]
pub struct Users {
    pub kaiin_id: i64,
    pub adana: String,
    pub mail_address: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}
