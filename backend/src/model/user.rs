use diesel::{
    result::Error,
    Queryable,
    Insertable,
    QueryDsl,
    insert_into,
};
use serde::{Serialize, Deserialize};
use crate::db::util::{get_conn, DbPool};
use crate::db::schema::users;
use crate::db::schema::users::dsl::users as users_table;
use diesel_async::RunQueryDsl;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct User {
    pub uid: String,
    pub username: String,
    pub is_guest: bool,
}

impl User {
    pub async fn find_by_uid(pool: &DbPool, uid: &str) -> Result<User, Error> {
        let conn = &mut get_conn(pool).await?;
        users_table.find(uid).first::<Self>(conn).await
    }

    pub async fn insert(&self, pool: &DbPool) -> Result<(), Error> {
        let conn = &mut get_conn(pool).await?;
        insert_into(users_table)
            .values(self)
            .execute(conn).await?;
        Ok(())
    }
}