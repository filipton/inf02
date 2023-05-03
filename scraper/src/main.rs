use anyhow::Result;
use scraper::{Question, QuestionsContainer};
use std::{io::Write, path::PathBuf};

mod scraper;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let images_dir = PathBuf::from("./images");
    let base_path = PathBuf::from("base.json");
    _ = std::fs::create_dir(&images_dir);

    let mut container = QuestionsContainer::read_from_file(&base_path, &images_dir)?;

    let arg = std::env::args().nth(1).unwrap_or_default();
    match arg.as_str() {
        "add" => {
            adder(container, base_path).await?;
        }
        "scrape" => {
            container.scrape_full(1, None).await?;
            container.write_to_file(&base_path)?;
        }
        _ => {
            println!("Usage: {} [add|scrape]", std::env::args().nth(0).unwrap());
        }
    }

    Ok(())
}

async fn adder(mut container: QuestionsContainer, base_path: PathBuf) -> Result<()> {
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

            let image = if image.trim() == "" {
                None
            } else {
                container.download_image(image.trim().to_string()).await?
            };

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
                id: container.questions.len() as i32,
                text: question.trim().to_string(),
                image,
                anwsers,
                correct: correct.trim().parse()?,
            };

            container.add_question(&question)?;
            container.write_to_file(&base_path)?;
        } else {
            break;
        }
    }

    Ok(())
}
