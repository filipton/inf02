use anyhow::Result;
use scraper::{Question, QuestionsContainer};
use std::{io::Write, path::PathBuf};

mod scraper;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::args().nth(1).unwrap_or_default() == "add" {
        adder().await?;
        return Ok(());
    }

    let images_dir = PathBuf::from("./images");
    _ = std::fs::create_dir(&images_dir);

    let mut container = QuestionsContainer::read_from_file(PathBuf::from("base.json"), images_dir)?;
    container.scrape_full(1, None).await?;
    container.write_to_file(PathBuf::from("base.json"))?;

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
