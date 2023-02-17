#![allow(warnings)]

use anyhow::Result as AResult;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use serde_yaml::Value as YAMLValue;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Example {
    raw_text: String,
    data: Option<YAMLValue>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Page {
    title: Option<String>,
    content: Option<String>,
    source: Option<String>,
    raw_text: String,
    examples: Vec<Example>,
    output: String,
}

fn main() {
    let mut page = Page {
        raw_text: fs::read_to_string("content/index.mdx").unwrap(),
        title: None,
        content: None,
        source: None,
        examples: vec![],
        output: String::from(""),
    };

    match get_title(&mut page) {
        Ok(_) => println!("Got: Title"),
        Err(e) => println!("Error Getting Title: {}", e),
    }

    match get_content(&mut page) {
        Ok(_) => println!("Got: Content"),
        Err(e) => println!("Error Getting Content: {}", e),
    }

    match get_source(&mut page) {
        Ok(_) => println!("Got: Source"),
        Err(e) => println!("Error Getting Source: {}", e),
    }

    match get_examples(&mut page) {
        Ok((a, b)) => println!("Got: Examples"),
        Err(e) => println!("Error Getting Examples: {}", e),
    }

    match get_yaml(&mut page) {
        Ok(()) => println!("Got: YAML"),
        Err(e) => println!("Error Getting YAML: {}", e),
    }

    build_output(&mut page);
    write_output(&page.output);

    // dbg!(&page);
}

fn write_output(text: &String) -> std::io::Result<()> {
    fs::write(
        "/Users/alan/workshop/rust-notes.alanwsmith.com/src/index.md",
        text,
    )?;
    Ok(())
}

fn build_output(page: &mut Page) {
    page.output.push_str("# ");
    page.output.push_str(page.title.as_ref().unwrap().as_str());
    page.output.push_str("\n\n");
    page.output
        .push_str(page.content.as_ref().unwrap().as_str());
    page.output.push_str("\n\n");

    // Pull the source code into a Vec.
    let raw_lines: Vec<&str> = page.source.as_ref().unwrap().split("\n").collect();

    for example in page.examples.iter() {
        //dbg!(example);

        page.output.push_str("### ");
        page.output.push_str(
            example
                .data
                .as_ref()
                .unwrap()
                .get("heading")
                .unwrap()
                .as_str()
                .unwrap(),
        );
        page.output.push_str("\n");
        page.output.push_str("```rust, editable\n");

        // Make a vec to hold the local source code lines
        let mut local_lines = vec![""];
        local_lines.resize(raw_lines.len(), "");

        for visible_line_number in example
            .data
            .as_ref()
            .unwrap()
            .get("visible")
            .unwrap()
            .as_sequence()
            .unwrap()
        {
            let active_index = visible_line_number.as_u64().unwrap() as usize - 1;
            local_lines[active_index] = raw_lines[active_index];
        }

        for override_value in example
            .data
            .as_ref()
            .unwrap()
            .get("overrides")
            .unwrap()
            .as_sequence()
            .unwrap()
        {
            let override_index = override_value["line"].as_u64().unwrap() as usize - 1;
            let override_text = override_value["text"].as_str().unwrap();
            local_lines[override_index] = override_text;
            dbg!(override_index);
            dbg!(override_text);

            //dbg!(&override_value["line"]);
        }

        // output the local lines
        page.output.push_str(&local_lines.join("\n"));
        page.output.push_str("\n```\n\n");
        page.output.push_str(
            example
                .data
                .as_ref()
                .unwrap()
                .get("note")
                .unwrap()
                .as_str()
                .unwrap(),
        );

        page.output.push_str("\n\n");
    }
}

fn get_yaml(page: &mut Page) -> AResult<()> {
    for example in page.examples.iter_mut() {
        example.data = serde_yaml::from_str(&example.raw_text)?;
    }
    Ok(())
}

fn get_examples(page: &mut Page) -> IResult<&str, &str> {
    let (input, _) = take_until("---> EXAMPLE")(page.raw_text.as_str())?;
    let (input, _) = tag("---> EXAMPLE")(input)?;
    let (input, mut lines) =
        separated_list1(tag("---> EXAMPLE"), take_until("---> EXAMPLE"))(input)?;
    let (_, tmp_line) = rest(input)?;
    let (_, mut line) = preceded(tag("---> EXAMPLE"), rest)(input)?;
    lines.push(line);
    for line in lines.iter() {
        let mut example = Example {
            raw_text: line.trim().to_string(),
            data: None,
        };
        page.examples.push(example);
    }
    Ok(("", ""))
}

fn get_content(page: &mut Page) -> IResult<&str, &str> {
    let (input, _) = take_until("---> CONTENT")(page.raw_text.as_str())?;
    let (input, _) = tag("---> CONTENT")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, content) = take_until("--->")(input)?;
    page.content = Some(content.trim().to_string());
    Ok(("", ""))
}

fn get_source(page: &mut Page) -> IResult<&str, &str> {
    let (input, _) = take_until("---> SOURCE")(page.raw_text.as_str())?;
    let (input, _) = tag("---> SOURCE")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, source) = take_until("--->")(input)?;
    page.source = Some(source.trim().to_string());
    Ok(("", ""))
}

fn get_title(page: &mut Page) -> IResult<&str, &str> {
    let (input, _) = tag("---> TITLE")(page.raw_text.as_str())?;
    let (input, _) = multispace1(input)?;
    let (_input, title) = not_line_ending(input)?;
    page.title = Some(title.to_string());
    Ok(("", ""))
}
