// Place represents a cool location.
#[derive(Queryable)]
pub struct Place {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: std::time,
    pub updated_at: std::time,
}