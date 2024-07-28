pub trait Schema {
    fn by_tag(tag: &str) -> Option<bool>;

    fn properties(&self) -> SchemaData;
}

pub enum SchemaData<'a> {
    Container(&'a str),
    Object(Vec<(&'a str, SchemaData<'a>)>),
}