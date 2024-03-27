use serde::Serialize;

#[derive(Serialize)]
pub struct Pagination<T> {
    pub items: Vec<T>,
    pub total: i64,
}
