use sqlx::PgPool;

use crate::domain::block::Block;

pub struct BlockRepository {
    pool: PgPool
}

impl BlockRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn get_all(&self) -> anyhow::Result<Vec<Block>> {
        let sql = r#"
            SELECT 
                plantation,
                block,
                reserve,
                "year planted",
                "block area (ha)",
                productive
            FROM block_register
            LIMIT 200;
        "#;

        let blocks = sqlx::query_as(sql)
            .fetch_all(&self.pool)
            .await?;

        Ok(blocks)
    }
}
