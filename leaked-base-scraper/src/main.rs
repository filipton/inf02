use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.html")?;
    let dom = tl::parse(&input, tl::ParserOptions::default())?;
    let parser = dom.parser();

    let articles = dom.query_selector("article").unwrap();
    for article in articles {
        let children = article.get(parser).unwrap().children().unwrap();
        let children = children.all(parser);

        println!("\n\n============\n\n");

        for child in children {
            println!("{}", child.as_tag().unwrap().);
        }
    }

    Ok(())
}
