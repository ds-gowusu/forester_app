use sqlx::PgPool;

use crate::domain::tree::TreeData;

pub struct TreeRepository {
    pool: PgPool
}

impl TreeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<TreeData>> {
        let sql = r#"
            SELECT 
                plantation,
                area_type,
                "monitoring year",
                plot,
                tree_id,
                height_m,
                dbh_cm
            FROM tree_data
            LIMIT 200;
        "#;

        let trees = sqlx::query_as(sql)
            .fetch_all(&self.pool)
            .await?;

        Ok(trees)
    }
}
