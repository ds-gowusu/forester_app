use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct Block {
    pub plantation: String,
    #[sqlx(rename="block")]
    pub block_name: String,
    pub reserve: String,
    #[sqlx(rename="year planted")]
    pub year_planted: i64,
    #[sqlx(rename="block area (ha)")]
    pub block_area: f64,
    pub productive: Option<f64>
}
