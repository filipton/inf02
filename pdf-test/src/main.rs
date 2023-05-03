fn main() {
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

    questions.iter().for_each(|q| {
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

        println!("Question: {}", question_text);
        println!("Answer: {:#?}", anwsers);
    });
}
