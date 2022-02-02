mod default;
mod name;

pub use default::EnvDefaultAttrs;
pub use name::EnvNameAttrs;

pub struct Field {
    pub field_name: EnvNameAttrs,
    pub default_type: EnvDefaultAttrs,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            field_name: EnvNameAttrs::Empty,
            default_type: EnvDefaultAttrs::Empty,
        }
    }
}
