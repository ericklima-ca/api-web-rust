use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Score {
    pub user: String,
    pub points: i64,
}
