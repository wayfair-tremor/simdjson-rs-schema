use super::error;
use super::scope;
use simd_json::value::Value as ValueTrait;

use std::fmt;

/*
macro_rules! nonstrict_process {
    ($val:expr, $path:ident) => {{
        let maybe_val = $val;
        if maybe_val.is_none() {
            return $crate::validators::ValidationState::new();
        }

        maybe_val.unwrap();
    }};
}
*/

macro_rules! val_error {
    ($err:expr) => {
        $crate::validators::ValidationState {
            errors: vec![Box::new($err)],
            missing: vec![],
        }
    };
}

pub use self::ref_::Ref;
pub use self::required::Required;

pub mod formats;
pub mod ref_;
pub mod required;

#[derive(Debug)]
pub struct ValidationState {
    pub errors: super::error::SimdjsonSchemaErrors,
    pub missing: Vec<url::Url>,
}

impl ValidationState {
    pub fn new() -> ValidationState {
        ValidationState {
            errors: vec![],
            missing: vec![],
        }
    }

    pub fn append(&mut self, second: ValidationState) {
        self.errors.extend(second.errors);
        self.missing.extend(second.missing);
    }
}

pub trait Validator<V>
where
    V: ValueTrait,
{
    fn validate(&self, item: &V, _: &str, _: &scope::Scope<V>) -> ValidationState
    where
        <V as ValueTrait>::Key: std::borrow::Borrow<str> + std::hash::Hash + Eq;
}

impl<V> fmt::Debug for dyn Validator<V> + 'static + Send + Sync
where
    V: ValueTrait,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("<validator>")
    }
}

pub type BoxedValidator<V> = Box<dyn Validator<V> + 'static + Send + Sync>;
pub type Validators<V> = Vec<BoxedValidator<V>>;
