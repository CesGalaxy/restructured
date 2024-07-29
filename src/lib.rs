pub mod engine;

#[cfg(test)]
mod tests {
    use crate::engine::data::DataType;

    static EXAMPLE_CODE: &str = r#"
        @block {
            name: 'example'
            value: null
            nested: [
                name: 'nested'
                value: 24
                1 3 5 7 9
                11 13 15 17 19
                8401
            ]
        }
    "#;

    #[test]
    fn test_parsing() {
        let schema = vec![
            ("name", DataType::String),
            ("value", DataType::Nullable(Box::new(DataType::Number))),
            ("nested", DataType::Container("numbers", Box::new(DataType::Number), vec![
                ("name", DataType::String),
                ("value", DataType::Number),
            ])),
        ];

        let block = DataType::Block(Box::new(DataType::Object(schema)));

        let result = block.parse()(EXAMPLE_CODE);
        println!("{:?}", result);
    }
}
