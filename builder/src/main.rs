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
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Result;
use serde_json::Value;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct LineSet {
    heading: String,
    visible: Vec<i32>,
    active: Vec<i32>,
    fades: Vec<i32>,
    overrides: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    raw_text: String,
    raw_json: Option<String>,
    line_set: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Page {
    title: Option<String>,
    content: Option<String>,
    source: Option<String>,
    raw_text: String,
    examples: Vec<Example>,
}

fn main() {
    let mut page = Page {
        raw_text: fs::read_to_string("content/index.mdx").unwrap(),
        title: None,
        content: None,
        source: None,
        examples: vec![],
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

    dbg!(page);

    // match get_examples(&mut page) {
    //     Ok(_) => println!("It worked"),
    //     Err(_) => println!("It borked"),
    // }

    // match get_raw_jsons(&mut page) {
    //     Ok(_) => println!("It worked"),
    //     Err(_) => println!("It borked"),
    // }

    // get_line_set(&mut page);

    // match get_line_set(&mut page) {
    //     Ok(_) => println!("It worked"),
    //     Err(e) => println!("{:?}", e),
    // }

    // dbg!(page);
}

// fn get_raw_jsons(page: &mut Page) -> IResult<&str, &str> {
//     for example in page.examples.iter_mut() {
//         let (input, _) = tag("details:")(example.raw_text.as_str())?;
//         let (input, raw_json) = take_until("note:")(input)?;
//         // dbg!(raw_json.to_string());
//         example.raw_json = Some(raw_json.trim().to_string());
//         // example.line_set = serde_json::from_str(raw_json)?;
//     }
//     Ok(("", ""))
// }

fn get_line_set(page: &mut Page) -> Result<&str> {
    for example in page.examples.iter_mut() {
        // This works
        example.line_set = serde_json::from_str("{\"a\":\"b\"}").unwrap();
        //dbg!(&example.line_set.as_ref().unwrap());

        //
        example.line_set = serde_json::from_str("{\"a\":\"b\"}").unwrap();
        //dbg!(&example.line_set.as_ref().unwrap());

        // dbg!(&example.raw_json.as_ref().unwrap());
        //
        // //////////////////////////////////////////////
        // Things that don't work
        // serde_json::from_str(&example.raw_json.as_ref().unwrap())?;
        // serde_json::from_str(&example.raw_json.unwrap_or("{}".to_string()));

        ////////////////////////////////////////
        // This doesn't work. It loads the data, but it's a String
        // dbg!(json!(&example.raw_json.as_ref().unwrap()));
        // example.line_set = Some((json!(&example.raw_json.as_ref().unwrap())));
        // dbg!(&example.line_set.as_ref()["heading"]);

        ///////////////////////////////////////////////////
        //// This fails
        //dbg!(&example.raw_json);
        //let v: Value = serde_json::from_str(&example.raw_json.as_ref().unwrap().as_str())?;
        //dbg!(v);

        ///////////////////////////////////////////////////
        //// This works now that the data is cleaned up
        //// dbg!(&example.raw_json);
        //let v: Value = serde_json::from_str(&example.raw_json.as_ref().unwrap().as_str()).unwrap();
        //dbg!(v);

        /////////////////////////////////////////////////////
        //// This works with less whatever
        //let v: Value = serde_json::from_str(&example.raw_json.as_ref().unwrap())?;
        //dbg!(v);

        ///////////////////////////////////////////////////
        // This works with less whatever
        example.line_set = serde_json::from_str(&example.raw_json.as_ref().unwrap())?;
        dbg!(&example.line_set.as_ref().unwrap()["heading"]
            .as_str()
            .unwrap());

        println!(
            "{}",
            &example.line_set.as_ref().unwrap()["heading"]
                .as_str()
                .unwrap()
        );

        ///////////////////////////////////////////////
        //
        //
        //
        //
        //
        //
        //
        // dbg!(serde_json::from_str(&example.raw_json.as_ref().unwrap())?);

        // example.line_set = Some(serde_json::from_str(example.raw_json.unwrap())?).ok_or("asdf")?;
        // let v: Value = example.raw_json.unwrap().into();
        // dbg!(example.raw_json.as_ref().unwrap());
        // dbg!(serde_json::from_str(&example.raw_json.unwrap().as_str()));
        //dbg!(serde_json::from_str(
        //   example.raw_json.as_ref().unwrap().as_str()
        // )?);
        //example.line_set = Some(serde_json::from_str(example.raw_json.as_ref().unwrap())?);
        // dbg!(serde_json::from_str(Some(&example.raw_json).unwrap()));
        // dbg!(serde_json::from_str(example.raw_json).unwrap().to_str());
        // example.line_set = serde_json::from_str(example.raw_json))?;
    }
    // dbg!(page);
    Ok("")
}

// fn get_examples(page: &mut Page) -> IResult<&str, &str> {
//     let (input, _) = take_until("---> EXAMPLE")(page.raw_text.as_str())?;
//     let (input, _) = tag("---> EXAMPLE")(input)?;
//     let (input, mut lines) =
//         separated_list1(tag("---> EXAMPLE"), take_until("---> EXAMPLE"))(input)?;
//     let (_, tmp_line) = rest(input)?;
//     let (_, mut line) = preceded(tag("---> EXAMPLE"), rest)(input)?;
//     lines.push(line);
//     for line in lines.iter() {
//         let mut example = Example {
//             raw_text: line.trim().to_string(),
//             raw_json: None,
//             line_set: None,
//         };
//         page.examples.push(example);
//     }
//     Ok(("", ""))
// }

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
