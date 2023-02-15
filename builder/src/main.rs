#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::sequence::preceded;
use nom::IResult;
// use nom::bytes::complete::take;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::map_parser;
use nom::sequence::separated_pair;
use nom::sequence::tuple;

use std::fs;

fn main() {
    let mut contents = String::from("");
    let file_data = fs::read_to_string("content/index.mdx").unwrap();
    let (input, title) = get_title(&file_data).unwrap();
    // println!("{}", title);
    contents.push_str("# ");
    contents.push_str(title);

    let (input, data) = get_content(input).unwrap();
    contents.push_str("\n");
    contents.push_str("\n");
    contents.push_str(data);

    output_file(&contents);
}

fn get_content(input: &str) -> IResult<&str, &str> {
    // let (input, spaces) = space0(input)?;
    // let (input, newline) = line_ending(input)?;
    // let (input, spaces) = space0(input)?;
    // let (input, newline) = line_ending(input)?;

    let (input, newline) = multispace1(input)?;
    let (input, header) = tag("---> CONTENT")(input)?;
    let (input, content) = take_until("--->")(input)?;
    Ok((input, content))
}

fn get_title(input: &str) -> IResult<&str, &str> {
    let (input, header) = tag("---> TITLE")(input)?;
    let (input, newline) = multispace1(input)?;
    let (input, title) = not_line_ending(input)?;
    Ok((input, title))
}

fn output_file(content: &str) {
    fs::write("../src/index.md", content).unwrap();
}
