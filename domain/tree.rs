use sqlx::prelude::FromRow;

#[derive(Debug, FromRow)]
pub struct TreeData {
    pub plantation: String,
    pub area_type: String,
    #[sqlx(rename="plot")]
    pub plot_id: String,
    pub tree_id: Option<f64>,
    #[sqlx(rename="dbh_cm")]
    pub dbh: Option<f64>,
    #[sqlx(rename="height_m")]
    pub height: Option<f64>,
    #[sqlx(rename="monitoring year")]
    pub measurement_year: f64,
}
