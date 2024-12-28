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

    pub fn genres() -> Vec<String> {
        use super::schema::artists::dsl::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;

        artists
            .filter(genre.is_not_null())
            .distinct()
            .select(genre)
            .order(genre.asc())
            .load::<Option<String>>(conn)
            .map(|v| v.into_iter().flatten().collect())
            .unwrap_or_else(|_| vec![])
    }
}
