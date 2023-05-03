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
        "pdf" => {
            pdf(container, base_path).await?;
        }
        _ => {
            println!(
                "Usage: {} [add|scrape|pdf]",
                std::env::args().nth(0).unwrap()
            );
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
                container.download_image(image.trim()).await?
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

async fn pdf(mut container: QuestionsContainer, base_path: PathBuf) -> Result<()> {
    println!("Copy pdf to input.pdf and press enter to continue");
    println!("Note: use print to pdf in chrome!");
    std::io::stdin().read_line(&mut String::new())?;

    let bytes = std::fs::read("input.pdf").unwrap();
    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    // delete empty lines
    let out = out
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let mut questions = out.split("Zadanie").collect::<Vec<_>>();
    questions.remove(0);

    for q in questions {
        let split = q.splitn(2, "A. ").collect::<Vec<_>>();

        let question_text = split[0].lines().skip(1).collect::<Vec<_>>().join("\n");
        let question_text = question_text.trim();

        let anwsers = split[1]
            .lines()
            .map(|l| {
                let splitted = l.splitn(2, ". ").collect::<Vec<_>>();

                if splitted.len() == 2 {
                    splitted[1].trim()
                } else {
                    splitted[0].trim()
                }
            })
            .collect::<Vec<_>>();

        let anwsers = anwsers.iter().take(4).collect::<Vec<_>>();

        println!("=======================================");
        println!("Question: {}", question_text);
        println!("Answer: {:#?}", anwsers);
        println!("---------------------------------------");

        std::io::stdout()
            .write_all(b"Enter correct anwser number (1..X) (Enter to skip): ")
            .unwrap();
        std::io::stdout().flush().unwrap();
        let mut correct = String::new();
        std::io::stdin().read_line(&mut correct).unwrap();
        if correct.trim() == "" {
            continue;
        }

        std::io::stdout()
            .write_all(b"Enter image url (Enter if none): ")
            .unwrap();
        std::io::stdout().flush().unwrap();
        let mut image = String::new();
        std::io::stdin().read_line(&mut image).unwrap();

        let image = if image.trim() == "" {
            None
        } else {
            container.download_image(image.trim()).await?
        };

        let question = Question {
            id: container.questions.len() as i32,
            text: question_text.to_string(),
            image,
            anwsers: anwsers
                .iter()
                .map(|s| s.trim_end_matches(".").to_string())
                .collect(),
            correct: correct.trim().parse()?,
        };

        container.add_question(&question)?;
        container.write_to_file(&base_path)?;
    }

    Ok(())
}
