use convert_case::{Case, Casing};
use syn::{Lit, Lit::Str};

pub enum EnvNameAttrs {
    Flatten,
    RenameAbs(Lit),
    Rename(Lit),
    Empty,
}

impl EnvNameAttrs {
    pub fn name(&self, struct_name: &str, field_name: &str) -> String {
        match self {
            EnvNameAttrs::Flatten => field_name.to_string(),
            EnvNameAttrs::RenameAbs(Str(str)) => str.value().to_string(),
            EnvNameAttrs::Rename(Str(str)) => {
                format!("{}_{}", struct_name.to_string(), str.value())
            }
            _ => format!("{}_{}", struct_name.to_string(), field_name.to_string()),
        }
        .to_case(Case::UpperSnake)
    }
}
