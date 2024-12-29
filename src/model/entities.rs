use crate::model::ModelManager;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub struct EntitiesBmc;

impl EntitiesBmc {
    pub fn bands() -> Vec<String> {
        use super::schema::artists::dsl::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;

        artists
            .select(name)
            .order(name.asc())
            .load::<String>(conn)
            .unwrap_or_else(|_| vec![])
    }
}
