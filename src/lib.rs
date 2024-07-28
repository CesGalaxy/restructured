pub mod engine;

#[cfg(test)]
mod tests {
    use crate::engine::data::DataValue;

    static EXAMPLE_CODE: &str = r#"
        @block {
            name: 'example'
            value: 42
            nested: {
                name: 'nested'
                value: 24
            }
        }
    "#;

    #[test]
    fn test_parsing() {
        let result = DataValue::parse(EXAMPLE_CODE);
        println!("{:?}", result);
    }
}
