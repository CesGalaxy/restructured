use nom::{bytes::complete::tag, character::complete::{alpha1, multispace0}, sequence::tuple, IResult};

use crate::engine::data::{DataType, DataValue, MapSchema};

pub fn parse_map_properties<'a>(schema: &'a MapSchema<'a>) -> impl Fn(&'a str) -> IResult<&'a str, Vec<(&'a str, DataValue)>> {
    move |input: &'a str| {
        let (input, props) = nom::multi::many0(tuple((
            parse_map_property(schema),
            multispace0,
        )))(input)?;

        let props: Vec<(&'a str, DataValue)> = props.iter().map(|(prop, _)| prop).cloned().collect();

        let schema_satisfied = schema.iter().all(|(s_key, s_type)| {
            props.iter().any(|(p_key, _)| p_key == s_key) || matches!(s_type, DataType::Nullable(_))
        });

        assert!(schema_satisfied, "schema not satisfied");

        Ok((input, props))
    }
}

pub fn parse_map_property<'a>(schema: &'a MapSchema<'a>) -> impl Fn(&'a str) -> IResult<&'a str, (&'a str, DataValue)> {
    move |input: &'a str| {
        let (input, key) = alpha1(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = multispace0(input)?;

        let prop_type = &schema.iter().find(|(prop_key, _)| prop_key == &key).expect("property should be listed in schema").1;

        let (input, value) = prop_type.parse()(input)?;

        Ok((input, (key, value)))
    }
}