use nom::{branch::alt, bytes::complete::{escaped, tag}, character::complete::{multispace0, none_of}, sequence::delimited, IResult};

use super::syntax::{block::Block, object::parse_object};

#[derive(Debug, Clone)]
pub enum DataValue<'a> {
    String(String),
    Number(u64),
    Block(Block<'a>),
    Map(Vec<(&'a str, DataValue<'a>)>),
}

impl<'a> DataValue<'a> {
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, _) = multispace0(input)?;

        let (input, value) = alt((
            DataValue::parse_string,
            DataValue::parse_number,
            DataValue::parse_block,
            DataValue::parse_map,
        ))(input)?;

        let (input, _) = multispace0(input)?;

        Ok((input, value))
    }

    pub fn parse_string(input: &'a str) -> IResult<&'a str, Self> {
        let esc = escaped(none_of("\\\'"), '\\', tag("'"));
        let esc_or_empty = alt((esc, tag("")));
        let (input, res) = delimited(tag("'"), esc_or_empty, tag("'"))(input)?;

        Ok((input, Self::String(res.to_string())))
    }

    pub fn parse_number(input: &'a str) -> IResult<&'a str, Self> {
        let (input, res) = nom::character::complete::digit1(input)?;

        Ok((input, Self::Number(res.parse().unwrap())))
    }

    pub fn parse_block(input: &'a str) -> IResult<&'a str, Self> {
        let (input, block) = Block::<'a>::parse(input)?;

        Ok((input, Self::Block(block)))
    }

    pub fn parse_map(input: &'a str) -> IResult<&'a str, Self> {
        let (input, props) = parse_object(input)?;

        Ok((input, Self::Map(props)))
    }
}