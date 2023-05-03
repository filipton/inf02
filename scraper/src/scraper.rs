use crate::utils::{
    find_by_name, find_by_name_class, find_by_name_class_wo_style, find_by_name_id,
    get_element_string,
};
use anyhow::Result;
use html_parser::{Element, Node};
use reqwest::Client;
use std::{collections::HashMap, ops::RangeInclusive, path::PathBuf};

pub const BASE_URL: &'static str = "https://egzamin-informatyk.pl";
pub const Q_MAX: i32 = 2227;

pub struct QuestionsContainer {
    pub questions: Vec<Question>,
    pub client: Client,
    pub image_dir: PathBuf,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Question {
    pub id: i32,
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,

    pub anwsers: Vec<String>,
    pub correct: i8,
}

impl QuestionsContainer {
    pub fn new(image_dir: PathBuf) -> Self {
        Self {
            questions: Vec::new(),
            client: Client::new(),
            image_dir,
        }
    }

    /// Reads questions from json file
    /// If file does not exist it doesn't return error
    pub fn read_from_file(path: &PathBuf, image_dir: &PathBuf) -> Result<Self> {
        let json = std::fs::read_to_string(path).unwrap_or("[]".into());
        let questions: Vec<Question> = serde_json::from_str(&json)?;

        Ok(Self {
            questions,
            client: Client::new(),
            image_dir: image_dir.clone(),
        })
    }

    pub fn write_to_file(&self, path: &PathBuf) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.questions)?;
        std::fs::write(path, json)?;

        Ok(())
    }

    pub fn add_question(&mut self, question: &Question) -> Result<()> {
        let mut q1 = question.clone();
        q1.anwsers.sort();

        if self.questions.iter().any(|q| {
            let mut q2 = q.clone();
            q2.anwsers.sort();

            return q.text == question.text && q.anwsers == question.anwsers;
        }) {
            println!("[Duplication protection] Question already exists");
            if question.image.is_some() {
                std::fs::remove_file(self.image_dir.join(&question.image.as_ref().unwrap()))?;
            }

            return Ok(());
        }

        self.questions.push(question.clone());
        Ok(())
    }

    pub async fn scrape_full(&mut self, from: i32, to: Option<i32>) -> Result<()> {
        let to = to.unwrap_or(Q_MAX);

        let qi = ((to - from + 1) / 40) + 1;
        for i in 0..qi {
            println!("Scraping {}/{}", i + 1, qi);
            let i = (i * 40) + 1;
            self.scrape(i..=(i + 40)).await?;
        }

        Ok(())
    }

    /// Scrape based on 40 questions test
    /// Range must sum to 40 and it consists of question ids from 1 to Q_MAX
    async fn scrape(&mut self, range: RangeInclusive<i32>) -> Result<()> {
        if range.end() - range.start() != 40 {
            return Err(anyhow::anyhow!("Range must sum to 40"));
        }

        let mut question_id = *range.start();
        let mut ex_map: HashMap<String, String> = HashMap::new();
        for i in 1..=40 {
            ex_map.insert(format!("pyt{}", i), format!("{}", question_id));
            ex_map.insert(format!("orderodp{}", i), format!("1234"));

            if question_id >= Q_MAX {
                question_id = 1;
            }
            question_id += 1;
        }

        self.scrape_40(ex_map).await?;
        Ok(())
    }

    async fn scrape_40(&mut self, body: HashMap<String, String>) -> Result<()> {
        let mut tmp_question: Question = Question {
            id: self.questions.len() as i32,
            text: String::new(),
            image: None,
            anwsers: Vec::new(),
            correct: -1,
        };

        let mut tmp_inside = false;
        for child in self.get_raw_questions(body).await? {
            let elem = child.element().unwrap();
            let classes = &elem.classes;

            if classes == &vec!["trescE"] {
                tmp_inside = true;
                tmp_question = Question {
                    id: self.questions.len() as i32,
                    text: String::from(get_element_string(&elem, false)),
                    image: None,
                    anwsers: Vec::new(),
                    correct: -1,
                };
            }

            if tmp_inside {
                if classes == &vec!["obrazek"] {
                    tmp_question.image = self.process_image(&elem).await?;
                } else if classes.len() == 1 && classes[0].starts_with("odp") {
                    tmp_inside = self.process_anwser(&elem, classes, &mut tmp_question)?;
                } else if classes == &vec!["sep"] {
                    self.add_question(&tmp_question)?;
                    tmp_inside = false;
                }
            }
        }

        Ok(())
    }

    fn process_anwser(
        &mut self,
        elem: &Element,
        classes: &Vec<String>,
        tmp_question: &mut Question,
    ) -> Result<bool> {
        let anwser_text = elem
            .children
            .iter()
            .find_map(|t| t.text())
            .unwrap()
            .to_owned();

        tmp_question.anwsers.push(anwser_text);
        if classes[0].ends_with("good") {
            tmp_question.correct = tmp_question.anwsers.len() as i8;
        }

        if tmp_question.anwsers.len() == 4 {
            self.add_question(&tmp_question)?;
            return Ok(false);
        }

        Ok(true)
    }

    async fn process_image(&mut self, elem: &Element) -> Result<Option<String>> {
        let image = elem
            .children
            .iter()
            .find_map(|img| img.element())
            .unwrap()
            .attributes
            .get("src")
            .unwrap()
            .to_owned()
            .unwrap();

        self.download_image(&format!("{}{}", BASE_URL, &image.trim_start_matches("..")))
            .await
    }

    pub async fn download_image(&mut self, image: &str) -> Result<Option<String>> {
        let image_bytes = self.client.get(image).send().await?.bytes().await?;

        let image_name = format!("{}.png", self.questions.len());
        let image_path = self.image_dir.join(&image_name);
        std::fs::write(image_path, image_bytes)?;

        Ok(Some(image_name))
    }

    async fn get_raw_questions(&mut self, body: HashMap<String, String>) -> Result<Vec<Node>> {
        let res = self
            .client
            .post(format!(
                "{}/odpowiedzi-inf02-ee08-sprzet-systemy-sieci/",
                BASE_URL
            ))
            .form(&body)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        let res_html = res.text().await?;
        let html = html_parser::Dom::parse(&res_html)?;

        let html = html
            .children
            .iter()
            .find(|h| h.element().unwrap().name == "html")
            .unwrap()
            .element()
            .unwrap();

        let body = find_by_name(html, "body");
        let main = find_by_name(&body, "main");
        let section = find_by_name_id(&main, "section", "portfolio");
        let div = find_by_name_class(&section, "div", vec!["container", "inner", "light-bg"]);
        let div = find_by_name_class_wo_style(&div, "div", vec!["row"]);
        let div = find_by_name_class(&div, "div", vec!["col-md-9"]);

        Ok(div.children)
    }
}
