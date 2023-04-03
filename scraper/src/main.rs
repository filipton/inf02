use anyhow::Result;
use html_parser::Element;
use std::collections::HashMap;

use crate::utils::{
    find_by_name, find_by_name_class, find_by_name_class_wo_style, find_by_name_id,
};

mod utils;

const QUESTION_PREFIX: &'static str = "pyt";

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();

    //let mut body = String::new();
    let mut ex_map: HashMap<String, String> = HashMap::new();
    let mut question_id = 1;

    for i in 1..=40 {
        ex_map.insert(format!("pyt{}", i), format!("{}", question_id));
        ex_map.insert(format!("orderodp{}", i), format!("1234"));

        //body += &format!("pyt{}={}&orderodp{}={}", i, question_id, i, "1234");

        question_id += 1;
    }

    let res = client
        .post("https://egzamin-informatyk.pl/odpowiedzi-inf02-ee08-sprzet-systemy-sieci/")
        .form(&ex_map)
        //.body(body)
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

    let mut questions: Vec<Question> = Vec::new();

    let mut tmp_inside = false;
    let mut tmp_question: String = String::new();
    let mut tmp_image: Option<String> = None;
    let mut tmp_anwsers: Vec<String> = Vec::new();
    let mut tmp_correct: usize = 5;

    for child in div.children {
        let elem = child.element().unwrap();
        let classes = &elem.classes;

        if classes == &vec!["trescE"] {
            tmp_inside = true;
            tmp_image = None;
            tmp_anwsers = vec![];
            tmp_correct = 5;

            tmp_question = elem
                .children
                .iter()
                .find_map(|t| t.text())
                .unwrap()
                .splitn(2, '.')
                .collect::<Vec<&str>>()[1]
                .trim()
                .to_owned();
        }

        if tmp_inside {
            if classes == &vec!["obrazek"] {
                tmp_image = elem
                    .children
                    .iter()
                    .find_map(|img| img.element())
                    .unwrap()
                    .attributes
                    .get("src")
                    .unwrap()
                    .to_owned();
            } else if classes.len() == 1 && classes[0].starts_with("odp") {
                let anwser_text = elem
                    .children
                    .iter()
                    .find_map(|t| t.text())
                    .unwrap()
                    .to_owned();

                tmp_anwsers.push(anwser_text);

                if classes[0].ends_with("good") {
                    tmp_correct = tmp_anwsers.len();
                }
            } else if classes == &vec!["sep"] {
                tmp_inside = false;

                questions.push(Question {
                    text: tmp_question.clone(),
                    image: tmp_image.clone(),
                    anwsers: tmp_anwsers.clone(),
                    correct: tmp_correct,
                });
            }
        }
    }

    println!("{:#?}", questions);

    Ok(())
}

#[derive(Debug)]
pub struct Question {
    pub text: String,
    pub image: Option<String>,
    pub anwsers: Vec<String>,
    pub correct: usize,
}
