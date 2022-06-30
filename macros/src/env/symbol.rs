use proc_macro2::{Group, Span, TokenStream, TokenTree};
use syn::{
    parse::{self, Parse},
    Meta::List,
};

#[derive(Clone)]
pub struct Symbol(&'static str);

impl PartialEq<Symbol> for syn::Path {
    fn eq(&self, word: &Symbol) -> bool {
        self.is_ident(word.0)
    }
}

pub const ENV: Symbol = Symbol("env");
pub const RENAME: Symbol = Symbol("rename");
pub const FLATTEN: Symbol = Symbol("flatten");
pub const RENAME_ABS: Symbol = Symbol("rename_abs");
pub const DEFAULT: Symbol = Symbol("default");
pub const DEFAULT_PATH: Symbol = Symbol("default_path");

pub fn get_env_meta_items(attr: &syn::Attribute) -> Vec<syn::NestedMeta> {
    if attr.path != ENV {
        return Vec::new();
    }

    match attr.parse_meta() {
        Ok(List(meta)) => meta.nested.into_iter().collect(),
        _ => panic!("Wrong meta"),
    }
}

pub fn get_lit_str<'a>(lit: &'a syn::Lit) -> &'a syn::LitStr {
    match lit {
        syn::Lit::Str(lit) => lit,
        _ => panic!("expected env attribute to be a string"),
    }
}

pub fn parse_lit_into_expr_path(lit: &syn::Lit) -> syn::ExprPath {
    let string = get_lit_str(lit);
    parse_lit_str(string).expect("Some error")
}

fn parse_lit_str<T>(s: &syn::LitStr) -> parse::Result<T>
where
    T: Parse,
{
    let tokens = spanned_tokens(s)?;
    syn::parse2(tokens)
}

fn spanned_tokens(s: &syn::LitStr) -> parse::Result<TokenStream> {
    let stream = syn::parse_str(&s.value())?;
    Ok(respan(stream, s.span()))
}

fn respan(stream: TokenStream, span: Span) -> TokenStream {
    stream.into_iter().map(|token| respan_token(token, span)).collect()
}

fn respan_token(mut token: TokenTree, span: Span) -> TokenTree {
    if let TokenTree::Group(g) = &mut token {
        *g = Group::new(g.delimiter(), respan(g.stream(), span));
    }
    token.set_span(span);
    token
}
