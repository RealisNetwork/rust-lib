use syn::Lit;

pub enum EnvRenameAttrs {
    Flatten(String),
    RenameAbs(Lit),
    Rename(Lit),
    Empty,
}

pub enum EnvDefaultAttrs {
    Default,
    DefaultPath(Lit),
    Empty,
}

pub struct Field {
    pub field_type: EnvRenameAttrs,
    pub default_type: EnvDefaultAttrs,
}

impl Default for Field {
    fn default() -> Self {
        Self {
            field_type: EnvRenameAttrs::Empty,
            default_type: EnvDefaultAttrs::Empty,
        }
    }
}
