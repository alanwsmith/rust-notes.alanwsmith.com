#![allow(warnings)]

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::fs;

#[derive(Debug)]
struct Page {
    title: Option<String>,
    content: Option<String>,
    source: Option<String>,
    raw_text: String,
}

fn main() {
    let mut page = Page {
        raw_text: fs::read_to_string("content/index.mdx").unwrap(),
        title: None,
        content: None,
        source: None,
    };

    match get_title(&mut page) {
        Ok(_) => println!("It worked"),
        Err(_) => println!("It borked"),
    }

    match get_content(&mut page) {
        Ok(_) => println!("It worked"),
        Err(_) => println!("It borked"),
    }

    match get_source(&mut page) {
        Ok(_) => println!("It worked"),
        Err(_) => println!("It borked"),
    }

    match get_lines(&mut page) {
        Ok(_) => println!("It worked"),
        Err(_) => println!("It borked"),
    }

    dbg!(page.source);
}

fn get_lines(page: &mut Page) -> IResult<&str, &str> {
    let (input, _) = take_until("---> LINES")(page.raw_text.as_str())?;
    let (input, mut lines) = separated_list1(tag("---> LINES"), take_until("---> LINES"))(input)?;
    let (_, tmp_line) = rest(input)?;
    let (_, mut line) = preceded(tag("---> LINES"), rest)(input)?;
    lines.push(line);
    dbg!(lines);
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

// use serde::{Deserialize, Serialize};
// use serde_json::Result;
// use serde_json::Value;

// #[derive(Serialize, Deserialize)]
// struct LineSet {
//     lines: Vec<i32>,
//     highlights: Vec<i32>,
//     heading: String,
//     note: String,
// }

// #[derive(Serialize, Deserialize)]
// struct Example {
//     set: Vec<LineSet>,
// }

// fn main() {
//     let mut contents = String::from("");
//     let file_data = fs::read_to_string("content/index.mdx").unwrap();
//     // Get the page title
//     let (input, title) = get_title(&file_data).unwrap();
//     contents.push_str("# ");
//     contents.push_str(title);
//     // Get the main body of contents
//     let (input, data) = get_content(input).unwrap();
//     contents.push_str("\n");
//     contents.push_str("\n");
//     contents.push_str(data);
//     // Get the source code.
//     let (input, source) = get_source(input).unwrap();
//     contents.push_str("\n");
//     contents.push_str("```rust, editable");
//     contents.push_str("\n");
//     contents.push_str(source.trim());
//     contents.push_str("\n");
//     contents.push_str("```");
//     let the_lines: Vec<String> = source.trim().lines().map(str::to_string).collect();
//     println!("{:?}", the_lines);
//     // // Get the JSON
//     // let (input, json_data) = get_json(input).unwrap();
//     // let input = make_json(json_data).unwrap();
//     // println!("{:?}", input);
//     // // let e: Example = serde_json::from_str(json_data).unwrap();
//     // Output the file
//     output_file(&contents);
// }

// fn get_content(input: &str) -> IResult<&str, &str> {
//     let (input, newline) = multispace1(input)?;
//     let (input, header) = tag("---> CONTENT")(input)?;
//     let (input, data) = take_until("--->")(input)?;
//     Ok((input, data))
// }

// fn make_json(input: &str) -> Result<Value> {
//     // let data = r#"
//     //     {
//     //         "name": "John Doe",
//     //         "age": 43,
//     //         "phones": [
//     //             "+44 1234567",
//     //             "+44 2345678"
//     //         ]
//     //     }"#;
//     println!("{}", input);
//     let v: Value = serde_json::from_str(input)?;
//     Ok(v)
// }

// fn get_json(input: &str) -> IResult<&str, &str> {
//     let (input, data) = tag("---> JSON")(input)?;
//     let (input, data) = rest(input)?;
//     Ok((input, data))
// }

// fn get_source(input: &str) -> IResult<&str, &str> {
//     let (input, newline) = multispace0(input)?;
//     let (input, header) = tag("---> SOURCE")(input)?;
//     let (input, data) = multispace0(input)?;
//     let (input, data) = take_until("--->")(input)?;
//     Ok((input, data))
// }

// fn get_title(input: &str) -> IResult<&str, &str> {
//     let (input, header) = tag("---> TITLE")(input)?;
//     let (input, newline) = multispace1(input)?;
//     let (input, title) = not_line_ending(input)?;
//     Ok((input, title))
// }

// fn output_file(content: &str) {
//     fs::write("../src/index.md", content).unwrap();
// }
