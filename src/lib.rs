pub mod engine;

#[cfg(test)]
mod tests {
    use crate::engine::data::DataType;

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
        let schema = vec![
            ("name", DataType::String),
            ("value", DataType::Number),
            ("nested", DataType::Object(vec![
                ("name", DataType::String),
                ("value", DataType::Number),
            ])),
        ];

        let block = DataType::Block(Box::new(DataType::Object(schema)));

        let result = block.parse()(EXAMPLE_CODE);
        println!("{:?}", result);
    }
}
