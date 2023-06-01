use anyhow::Result;

fn main() -> Result<()> {
    let mut questions: Vec<Question> = vec![];

    let input = std::fs::read_to_string("input.html")?;
    let dom = tl::parse(&input, tl::ParserOptions::default())?;
    let parser = dom.parser();

    let articles = dom.query_selector("article").unwrap();
    for article in articles {
        let p_tags = article
            .get(parser)
            .unwrap()
            .as_tag()
            .unwrap()
            .query_selector(parser, "p")
            .unwrap();

        let mut tmp_question = Question {
            id: questions.len(),
            text: "".to_string(),
            image: None,
            video: None,
            anwsers: vec![],
            correct: 0,
        };
        let mut tmp_children = vec![];

        for p_tag in p_tags {
            let inner_html = p_tag.get(parser).unwrap().inner_html(parser);

            if inner_html.contains("video") {
                let re = regex::Regex::new(r#"src="(([^"])*.(JPG|PNG|jpg|png|mp4))""#).unwrap();
                let captures = re.captures(&inner_html).unwrap();

                let url = captures.get(1).unwrap().as_str();
                tmp_question.video = Some(url.to_string());
                continue;
            } else if inner_html.contains("img") {
                let re = regex::Regex::new(r#"src="(([^"])*.(JPG|PNG|jpg|png|mp4))""#).unwrap();
                let captures = re.captures(&inner_html).unwrap();

                let url = captures.get(1).unwrap().as_str();
                tmp_question.image = Some(url.to_string());
                continue;
            }

            tmp_children.push(inner_html);
        }

        // Add anwsers
        tmp_question
            .anwsers
            .push(tmp_children[tmp_children.len() - 2].to_string());
        tmp_question
            .anwsers
            .push(tmp_children[tmp_children.len() - 3].to_string());
        tmp_question
            .anwsers
            .push(tmp_children[tmp_children.len() - 4].to_string());
        tmp_question
            .anwsers
            .push(tmp_children[tmp_children.len() - 5].to_string());

        // Add question text
        tmp_question.text = tmp_children[0].to_string();

        // Add correct anwser
        tmp_question.correct = tmp_question
            .anwsers
            .iter()
            .position(|a| a == &tmp_children[tmp_children.len() - 1])
            .unwrap()
            + 1;

        // Remove prefix from anwsers
        tmp_question.anwsers = tmp_question
            .anwsers
            .into_iter()
            .map(|a| remove_answer_prefix(&a))
            .collect::<Vec<_>>();

        questions.push(tmp_question);
    }

    let json = serde_json::to_string_pretty(&questions)?;
    std::fs::write("output.json", json)?;

    Ok(())
}

fn remove_answer_prefix(text: &str) -> String {
    let re = regex::Regex::new(r"(^[A-D]. )").unwrap();
    re.replace(text, "").to_string()
}

#[derive(Debug, serde::Serialize)]
struct Question {
    id: usize,
    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<String>,

    anwsers: Vec<String>,
    correct: usize,
}
