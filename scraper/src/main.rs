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

    for child in div.children {
        let elem = child.element().unwrap();
        let classes = &elem.classes;

        if classes == &vec!["trescE"] {
            println!("{:?}", elem.children.iter().find_map(|t| t.text()).unwrap());
        }
    }

    Ok(())
}
