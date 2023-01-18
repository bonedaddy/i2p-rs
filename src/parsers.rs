use nom::bytes::complete::{tag, take_till};
use nom::multi::separated_list0;
use nom::{
	character::complete::{alphanumeric1 as alphanumeric, space1 as space},
	combinator::{recognize, value},
	sequence::{pair, tuple},
	IResult,
};

use nom::branch::alt;

pub fn is_space(chr: char) -> bool {
	chr == ' ' || chr == '\t'
}

pub fn is_next_line(chr: char) -> bool {
	chr == '\n'
}

pub fn is_space_or_next_line(chr: char) -> bool {
	is_space(chr) || is_next_line(chr)
}

pub fn is_double_quote(chr: char) -> bool {
	chr == '\"'
}

pub fn quoted_value(input: &str) -> IResult<&str, &str> {
	value(
		input,
		tuple((tag("\""), take_till(is_double_quote), tag("\""))),
	)(input)
}

// named nom_value to avoid collision
pub fn nom_value(input: &str) -> IResult<&str, &str> {
	value(input, take_till(is_space_or_next_line))(input)
}

pub fn key_value(input: &str) -> IResult<&str, &str> {
	value(
		input,
		tuple((
			alphanumeric,
			tag("="),
			alt((quoted_value, nom_value))
		))
	)(input)
}

pub fn keys_and_values(input: &str) -> Result<(&str, Vec<&str>), nom::Err<nom::error::Error<&str>>> {
		separated_list0(space, key_value)(input)
}

pub fn sam_hello(input: &str) -> IResult<&str, &str> {
	value(
		input,
		tuple((
			tag("HELLO REPLY "),
			keys_and_values,
			tag("\n"),
			// keys_and_values (is this called here???)
		))
	)(input)
}

pub fn sam_session_status(input: &str) -> IResult<&str, &str> {
	value(
		input,
		tuple((
			tag("SESSION STATUS "),
			keys_and_values,
			tag("\n")
			// keys_and_values (does this go here?)
		))
	)(input)
}

pub fn sam_stream_status(input: &str) -> IResult<&str, &str> {
	value(
		input,
		tuple((
			tag("STREAM STATUS "),
			keys_and_values,
			tag("\n")
			// keys_and_values (does this go here?)
		))
	)(input)
}

pub fn sam_naming_reply(input: &str) -> IResult<&str, &str> {
	value(
		input,
		tuple((
			tag("NAMING REPLY "),
			keys_and_values,
			tag("\n")
			// keys_and_values (does this go here?)
		))
	)(input)
}


pub fn sam_dest_reply<'a>(input: &str) -> Result<(&str, &str), nom::Err<nom::error::Error<&str>>>{
	value(
		input,
		tuple((
			tag("DEST REPLY "),
			keys_and_values,
			tag("\n"),
			// keys_and_values
		))
	)(input)
}

#[cfg(test)]
mod tests {
	use nom::error::ErrorKind;

	#[test]
	fn hello() {
		use crate::parsers::sam_hello;
		let res = sam_hello("HELLO REPLY RESULT=OK VERSION=3.1\n").unwrap();
		println!("{:#?}", res);
	}
}
