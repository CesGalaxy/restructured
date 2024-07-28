use nom::{
    bytes::complete::tag, character::complete::{alpha1, space0}, IResult
};

use crate::engine::data::{DataType, DataValue};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Block<'a> {
    name: &'a str,
    properties: DataValue<'a>,
}

impl<'a> Block<'a> {
    pub fn parse(schema: &'a DataType<'a>) -> impl Fn(&'a str) -> IResult<&'a str, Self> {
        move |input: &'a str| {
            let (input, name) = Self::parse_name(input)?;

            let (input, _) = space0(input)?;

            let (input, properties) = schema.parse()(input)?;

            Ok((input, Self { name, properties: properties }))
        }
    }

    pub fn parse_name(input: &'a str) -> IResult<&'a str, &'a str> {
        let (input, _) = tag("@")(input)?;

        let (input, name) = alpha1(input)?;

        Ok((input, name))
    }
}