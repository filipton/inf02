use anyhow::Result;
use std::collections::HashMap;

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

    println!("{:?}", ex_map);

    let res = client
        .post("https://egzamin-informatyk.pl/odpowiedzi-inf02-ee08-sprzet-systemy-sieci/")
        .form(&ex_map)
        //.body(body)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .await?;

    println!("{:?}", res.text().await?);

    Ok(())
}
