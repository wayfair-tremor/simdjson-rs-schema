use hashbrown::HashMap;
use simd_json::value::{BorrowedValue as Value, Value as ValueTrait};

use super::schema;
use super::validators;

pub type FormatBuilders<V> = HashMap<String, Box<dyn super::Keyword<V> + 'static + Send + Sync>>;

fn default_formats<V>() -> FormatBuilders<V>
where
    V: ValueTrait,
{
    let mut map: FormatBuilders<V> = HashMap::new();

    let date_time_builder = Box::new(|_def: &Value, _ctx: &schema::WalkContext<'_>| {
        Ok(Some(
            Box::new(validators::formats::DateTime) as validators::BoxedValidator<V>
        ))
    });
    map.insert("date-time".to_string(), date_time_builder);

    map
}

pub struct Format<V> {
    pub formats: FormatBuilders<V>,
}

impl<V> Format<V>
where
    V: ValueTrait,
{
    pub fn new() -> Format<V> {
        Format {
            formats: default_formats(),
        }
    }

    pub fn with<F>(build_formats: F) -> Format<V>
    where
        F: FnOnce(&mut FormatBuilders<V>),
    {
        let mut formats = default_formats();
        build_formats(&mut formats);
        Format { formats }
    }
}

impl<V> super::Keyword<V> for Format<V>
where
    V: ValueTrait + 'static,
{
    fn compile(&self, def: &Value, ctx: &schema::WalkContext<'_>) -> super::KeywordResult<V>
    where
        <V as ValueTrait>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq,
    {
        let format = keyword_key_exists!(def, "format");

        if format.as_str().is_some() {
            let format = format.as_str().unwrap();
            match self.formats.get(format) {
                Some(keyword) => keyword.compile(def, ctx),
                None => Ok(None),
            }
        } else {
            Err(schema::SchemaError::Malformed {
                path: ctx.fragment.join("/"),
                detail: "The value of format must be a string".to_string(),
            })
        }
    }
}
