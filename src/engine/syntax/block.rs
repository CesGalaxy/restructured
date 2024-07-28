use nom::{
    bytes::complete::tag, character::complete::{alpha1, space0}, IResult
};

use crate::engine::data::DataValue;

use super::object::parse_object;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Block<'a> {
    name: &'a str,
    properties: Vec<(&'a str, DataValue<'a>)>,
}

impl<'a> Block<'a> {
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, name) = Self::parse_name(input)?;

        let (input, _) = space0(input)?;

        let (input, properties) = parse_object(input)?;

        Ok((input, Self { name, properties }))
    }

    pub fn parse_name(input: &'a str) -> IResult<&'a str, &'a str> {
        let (input, _) = tag("@")(input)?;

        let (input, name) = alpha1(input)?;

        Ok((input, name))
    }
}