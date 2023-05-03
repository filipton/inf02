use crate::scraper::scrape_40;
use anyhow::Result;
use scraper::Question;
use std::{io::Write, path::PathBuf};

mod scraper;
mod utils;

const BASE_URL: &'static str = "https://egzamin-informatyk.pl";

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().nth(1).unwrap_or_default() == "add" {
        adder().await?;
        return Ok(());
    }

    let client = reqwest::Client::new();

    let images_dir = PathBuf::from("./images");
    _ = std::fs::create_dir(&images_dir);

    let mut questions_combined: Vec<Question> = Vec::new();

    let max = 2227;
    let jumps = max / 40;

    for i in 0..jumps {
        let start_id = (i * 40) + 1;

        let mut tmp_questions = scrape_40(&client, start_id).await?;
        questions_combined.append(&mut tmp_questions);
    }

    // if not divisible by 40 run last scrape
    let start_id = max - 40 + 1;
    let skip = 40 - (max - (jumps * 40));

    let mut tmp_questions = scrape_40(&client, start_id).await?;
    tmp_questions = tmp_questions.into_iter().skip(skip as usize).collect();
    questions_combined.append(&mut tmp_questions);

    println!("Downloading images...");
    for i in 0..questions_combined.len() {
        let question = questions_combined.get(i).unwrap();

        if question.image.is_some() {
            let image = question.image.clone().unwrap();
            let image_bytes = client
                .get(format!("{}{}", BASE_URL, &image.trim_start_matches("..")))
                .send()
                .await?
                .bytes()
                .await?;

            let image_name = format!("{}.png", i);
            questions_combined[i].image = Some(image_name.clone());

            let image_path = images_dir.join(image_name);
            std::fs::write(image_path, image_bytes)?;
        }
    }

    let base_json = serde_json::to_string_pretty(&questions_combined)?;
    std::fs::write(PathBuf::from("base.json"), base_json)?;

    Ok(())
}

async fn adder() -> Result<()> {
    let client = reqwest::Client::new();

    let path = PathBuf::from("base.json");
    let images_dir = PathBuf::from("./images");
    _ = std::fs::create_dir(&images_dir);

    let mut base_questions: Vec<Question> = serde_json::from_str(&std::fs::read_to_string(&path)?)?;

    loop {
        std::io::stdout().write_all(b"Enter question (Enter to close and save): ")?;
        std::io::stdout().flush()?;

        let mut question = String::new();
        std::io::stdin().read_line(&mut question)?;

        if question.trim() != "" {
            std::io::stdout().write_all(b"Enter image url (Enter if none): ")?;
            std::io::stdout().flush()?;
            let mut image = String::new();
            std::io::stdin().read_line(&mut image)?;

            let mut image_opt = None;
            if image.trim() != "" {
                let image_bytes = client.get(image).send().await?.bytes().await?;
                let image_name = format!("{}.png", base_questions.len());
                let image_path = images_dir.join(&image_name);
                std::fs::write(image_path, image_bytes)?;

                image_opt = Some(image_name);
            }

            let mut anwsers: Vec<String> = Vec::new();
            loop {
                std::io::stdout().write_all(b"Enter anwser (Enter if none): ")?;
                std::io::stdout().flush()?;
                let mut anwser = String::new();
                std::io::stdin().read_line(&mut anwser)?;

                if anwser.trim() == "" {
                    break;
                }

                anwsers.push(anwser.trim().to_string());
            }

            std::io::stdout().write_all(b"Enter correct anwser number (1..X): ")?;
            std::io::stdout().flush()?;
            let mut correct = String::new();
            std::io::stdin().read_line(&mut correct)?;

            let question = Question {
                id: base_questions.len() as i32,
                text: question.trim().to_string(),
                image: image_opt,
                anwsers,
                correct: correct.trim().parse()?,
            };

            base_questions.push(question);

            let base_json = serde_json::to_string_pretty(&base_questions)?;
            std::fs::write(&path, base_json)?;
        } else {
            break;
        }
    }

    Ok(())
}
