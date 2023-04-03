use crate::scraper::scrape_40;
use anyhow::Result;
use scraper::Question;
use std::path::PathBuf;

mod scraper;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let mut questions_combined: Vec<Question> = Vec::new();

    let max = 2227;
    let jumps = max / 40;

    for i in 0..jumps {
        let start_id = (i * 40) + 1;

        let mut tmp_questions = scrape_40(&client, start_id).await?;
        questions_combined.append(&mut tmp_questions);
    }

    // if not divisible by 40 run last scrape
    let start_id = max - 40;
    let skip = 40 - (max - (jumps * 40));

    let mut tmp_questions = scrape_40(&client, start_id).await?;
    tmp_questions = tmp_questions.into_iter().skip(skip as usize).collect();
    questions_combined.append(&mut tmp_questions);

    println!("max: {}", max);
    println!("len: {}", questions_combined.len());

    let base_json = serde_json::to_string_pretty(&questions_combined)?;
    std::fs::write(PathBuf::from("/tmp/baza.json"), base_json)?;

    Ok(())
}
