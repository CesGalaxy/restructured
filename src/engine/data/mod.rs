use nom::{branch::alt, bytes::complete::{escaped, tag}, character::complete::{multispace0, none_of}, sequence::{delimited, tuple}, IResult};

use super::syntax::{block::Block, map::parse_map_properties};

#[derive(Debug, Clone)]
pub enum DataValue<'a> {
    String(String),
    Number(u64),
    Block(Box<Block<'a>>),
    Map(Vec<(&'a str, DataValue<'a>)>),
    List(Vec<DataValue<'a>>)
}

pub type MapPropertyType<'a> = (&'a str, DataType<'a>);
pub type MapSchema<'a> = Vec<MapPropertyType<'a>>;

pub enum DataType<'a> {
    Container((&'a str, &'a DataType<'a>),  MapSchema<'a>),
    Object (MapSchema<'a>),
    String,
    Number,
    Block(Box<DataType<'a>>),
}

impl<'a> DataType<'a> {
    pub fn parse(&'a self) -> impl Fn(&'a str) -> IResult<&'a str, DataValue<'a>> {
        move |input: &'a str| {
            let (input, _) = multispace0(input)?;

            // TODO: WHY ARE THERE Fn() and fn()???
            let (input, res) = match self {
                DataType::String => Self::parse_string(input),
                DataType::Number => Self::parse_number(input),
                DataType::Block(schema) => Self::parse_block(schema)(input),
                DataType::Object(schema) => Self::parse_object(schema)(input),
                DataType::Container(collection, schema) => Self::parse_container(collection, schema)(input),
            }?;

            Ok((input, res))
        }
    }

    pub fn parse_string(input: &'a str) -> IResult<&'a str, DataValue> {
        let esc = escaped(none_of("\\\'"), '\\', tag("'"));
        let esc_or_empty = alt((esc, tag("")));
        let (input, res) = delimited(tag("'"), esc_or_empty, tag("'"))(input)?;

        Ok((input, DataValue::String(res.to_string())))
    }

    pub fn parse_number(input: &'a str) -> IResult<&'a str, DataValue> {
        let (input, res) = nom::character::complete::digit1(input)?;

        Ok((input, DataValue::Number(res.parse().unwrap())))
    }

    pub fn parse_block(schema: &'a DataType<'a>) -> impl Fn(&'a str) -> IResult<&'a str, DataValue> {
        move |input: &'a str| {
            let (input, block) = Block::<'a>::parse(schema)(input)?;

            Ok((input, DataValue::Block(Box::new(block))))
        }
    }

    pub fn parse_object(schema: &'a MapSchema<'a>) -> impl Fn(&'a str) -> IResult<&'a str, DataValue<'a>> {
        move |input: &'a str| {
            let (input, _) = tag("{")(input)?;
            let (input, _) = multispace0(input)?;
            let (input, props) = parse_map_properties(&schema)(input)?;
            let (input, _) = multispace0(input)?;
            let (input, _) = tag("}")(input)?;

            Ok((input, DataValue::Map(props)))
        }
    }

    pub fn parse_container(collection: &'a (&'a str, &'a DataType<'a>), schema: &'a MapSchema<'a>) -> impl Fn(&'a str) -> IResult<&'a str, DataValue<'a>> {
        move |input: &'a str| {
            let (input, _) = tag("[")(input)?;
            let (input, _) = multispace0(input)?;
            let (input, mut props) = parse_map_properties(&schema)(input)?;
            let (input, _) = multispace0(input)?;

            // TODO: Make this 2 lines with an alt
            let (input, elements) = nom::multi::many0(tuple((
                collection.1.parse(),
                multispace0,
            )))(input)?;

            let elements: Vec<DataValue> = elements.iter().map(|(element, _)| element).cloned().collect();

            props.push((collection.0, DataValue::List(elements)));

            let (input, _) = tag("]")(input)?;

            Ok((input, DataValue::Map(props)))
        }
    }
}