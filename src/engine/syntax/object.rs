use nom::{bytes::complete::tag, character::complete::{alpha1, multispace0}, sequence::tuple, IResult};

use crate::engine::data::DataValue;

pub fn parse_container(input: &str, clasifier: impl Fn(&DataValue) -> &'static str) -> IResult<&str, Vec<(&str, DataValue)>> {
    let (input, _) = tag("[")(input)?;

    let (input, mut props) = parse_map_properties(input)?;
    // TODO: Make this 2 lines with an alt
    let (input, elements) = nom::multi::many0(tuple((
        DataValue::parse,
        multispace0,
    )))(input)?;

    let elements = elements.iter().map(|(element, _)| element).cloned().collect::<Vec<_>>();

    for element in elements {
        let key = clasifier(&element);
        props.push((key, element));
    }

    let (input, _) = tag("]")(input)?;

    Ok((input, props))
}

pub fn parse_object(input: &str) -> IResult<&str, Vec<(&str, DataValue)>> {
    let (input, _) = tag("{")(input)?;

    let (input, _) = multispace0(input)?;

    let (input, props) = parse_map_properties(input)?;

    let (input, _) = multispace0(input)?;

    let (input, _) = tag("}")(input)?;

    Ok((input, props))
}

pub fn parse_map_properties(input: &str) -> IResult<&str, Vec<(&str, DataValue)>> {
    let (input, props) = nom::multi::many0(tuple((
        parse_map_property,
        multispace0,
    )))(input)?;

    let props = props.iter().map(|(prop, _)| prop).cloned().collect();

    Ok((input, props))
}

pub fn parse_map_property(input: &str) -> IResult<&str, (&str, DataValue)> {
    let (input, key) = alpha1(input)?;

    let (input, _) = multispace0(input)?;

    let (input, _) = tag(":")(input)?;

    let (input, _) = multispace0(input)?;

    let (input, value) = DataValue::parse(input)?;

    Ok((input, (key, value)))
}