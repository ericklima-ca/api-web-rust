use crate::models::score_model::Score;
use actix_web::{get, web, Responder, Result};

#[get("/")]
async fn get_score() -> Result<impl Responder> {
    let obj = gen_mock_scores();
    Ok(web::Json(obj))
}

fn gen_mock_scores() -> Vec<Score> {
    vec![
        Score {
            user: "Erick".to_string(),
            points: 1000,
        },
        Score {
            user: "Amorim".to_string(),
            points: 150,
        },
        Score {
            user: "Lima".to_string(),
            points: 200,
        },
    ]
}
