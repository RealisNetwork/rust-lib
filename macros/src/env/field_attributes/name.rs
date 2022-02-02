use convert_case::{Case, Casing};
use syn::{Lit, Lit::Str};

pub enum EnvNameAttrs {
    Flatten,
    RenameAbs(Lit),
    Rename(Lit),
    Empty,
}

impl EnvNameAttrs {
    pub fn name(&self, struct_name: String, field_name: String) -> String {
        match self {
            EnvNameAttrs::Flatten => format!("{}", field_name),
            EnvNameAttrs::RenameAbs(Str(str)) => format!("{}", str.value()),
            EnvNameAttrs::Rename(Str(str)) => format!("{}_{}", struct_name.to_string(), str.value()),
            _ => format!("{}_{}", struct_name.to_string(), field_name.to_string()),
        }
        .to_case(Case::UpperSnake)
    }
}
