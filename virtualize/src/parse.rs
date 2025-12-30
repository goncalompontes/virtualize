use proc_macro2::Ident;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};
use syn::{braced, parenthesized, Field, Token, Visibility};

pub struct Arg {
    pub name: Ident,
    pub colon: Token![:],
    pub ty: syn::Type,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let colon = input.parse()?;
        let ty = input.parse()?;
        Ok(Self { name, colon, ty })
    }
}

pub enum Receiver {
    Ref {
        ampersand: Token![&],
        lifetime: Option<syn::Lifetime>,
        self_token: Token![self],
    },
    Mut {
        ampersand: Token![&],
        lifetime: Option<syn::Lifetime>,
        mut_token: Token![mut],
        self_token: Token![self],
    },
}

impl Parse for Receiver {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ampersand = input.parse()?;
        let lifetime = input.parse().ok();
        if input.peek(Token![self]) {
            let self_token = input.parse()?;
            Ok(Self::Ref {
                ampersand,
                lifetime,
                self_token,
            })
        } else {
            let mut_token = input.parse()?;
            let self_token = input.parse()?;
            Ok(Self::Mut {
                ampersand,
                lifetime,
                mut_token,
                self_token,
            })
        }
    }
}

pub struct VirtualMethod {
    pub dyn_token: Token![dyn],
    pub fn_token: Token![fn],
    pub name: Ident,
    pub paren: Paren,
    pub receiver: Receiver,
    pub args: Option<(Token![,], Punctuated<Arg, Token![,]>)>,
    pub return_type: syn::ReturnType,
}

impl Parse for VirtualMethod {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let dyn_token = input.parse()?;
        let fn_token = input.parse()?;
        let name = input.parse()?;
        let content;
        let paren = parenthesized!(content in input);
        let receiver = content.parse()?;
        let args = if content.peek(Token![,]) {
            let colon_token = content.parse()?;
            let args = Punctuated::parse_terminated(&content)?;
            Some((colon_token, args))
        } else {
            None
        };
        let return_type = content.parse()?;
        Ok(Self {
            dyn_token,
            fn_token,
            name,
            paren,
            receiver,
            args,
            return_type,
        })
    }
}

pub struct FinalMethod {
    pub fn_token: Token![fn],
    pub name: Ident,
    pub paren: Paren,
    pub receiver: Receiver,
    pub args: Option<(Token![,], Punctuated<Arg, Token![,]>)>,
    pub return_type: syn::ReturnType,
}

impl Parse for FinalMethod {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fn_token = input.parse()?;
        let name = input.parse()?;
        let content;
        let paren = parenthesized!(content in input);
        let receiver = content.parse()?;
        let args = if content.peek(Token![,]) {
            let colon_token = content.parse()?;
            let args = Punctuated::parse_terminated(&content)?;
            Some((colon_token, args))
        } else {
            None
        };
        let return_type = content.parse()?;
        Ok(Self {
            fn_token,
            name,
            paren,
            receiver,
            args,
            return_type,
        })
    }
}

pub struct StaticFunction {
    pub fn_token: Token![fn],
    pub name: Ident,
    pub paren: Paren,
    pub args: Punctuated<Arg, Token![,]>,
    pub return_type: syn::ReturnType,
}

impl Parse for StaticFunction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fn_token = input.parse()?;
        let name = input.parse()?;
        let content;
        let paren = parenthesized!(content in input);
        let args = Punctuated::parse_terminated(&content)?;
        let return_type = content.parse()?;
        Ok(Self {
            fn_token,
            name,
            paren,
            args,
            return_type,
        })
    }
}

pub enum Function {
    Static(StaticFunction),
    Virtual(VirtualMethod),
    Final(FinalMethod),
}

impl Parse for Function {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![dyn]) {
            return Ok(Function::Virtual(input.parse()?));
        }

        let fork = input.fork();
        let maybe_final = fork.parse::<FinalMethod>().ok();
        match maybe_final {
            None => Ok(Function::Static(input.parse()?)),
            Some(final_method) => Ok(Function::Final(final_method)),
        }
    }
}

pub struct Impl {
    pub impl_token: Token![impl],
    pub name: Ident,
    pub generics: syn::Generics,
    pub where_clause: Option<syn::WhereClause>,
    pub brace: Brace,
    pub functions: Punctuated<Function, Token![;]>,
}

impl Parse for Impl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let impl_token = input.parse()?;
        let name = input.parse()?;
        let generics = input.parse()?;
        let where_clause = input.parse().ok();
        let content;
        let brace = braced!(content in input);
        let functions = Punctuated::parse_terminated(&content)?;
        Ok(Self {
            impl_token,
            name,
            generics,
            where_clause,
            brace,
            functions,
        })
    }
}

pub struct UseStatement {
    pub use_token: Token![use],
    pub path: syn::Path,
}

impl Parse for UseStatement {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let use_token = input.parse()?;
        let path = input.parse()?;
        Ok(Self { use_token, path })
    }
}

pub struct Fields {
    pub use_statements: Option<Punctuated<UseStatement, Token![;]>>,
    pub semi_token: Token![;],
    pub named: Punctuated<syn::Field, Token![,]>,
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let use_statements = Punctuated::parse_terminated(input).ok();
        let semi_token = input.parse()?;
        let named = Punctuated::parse_terminated_with(input, |input| Field::parse_named(input))?;
        Ok(Self {
            use_statements,
            semi_token,
            named,
        })
    }
}

pub struct Class {
    pub visibility: Option<Visibility>,
    pub struct_token: Token![struct],
    pub name: Ident,
    pub generics: syn::Generics,
    pub where_clause: Option<syn::WhereClause>,
    pub brace_token: Brace,
    pub fields: Fields,
}

impl Parse for Class {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility = input.parse().ok();
        let struct_token = input.parse()?;
        let name = input.parse()?;
        let generics = input.parse()?;
        let where_clause = input.parse().ok();
        let content;
        let brace_token = braced!(content in input);
        let fields = content.parse()?;
        Ok(Self {
            visibility,
            struct_token,
            name,
            generics,
            where_clause,
            brace_token,
            fields,
        })
    }
}

pub struct Context {
    pub classes: HashMap<Ident, (Class, Vec<Impl>)>,
}

impl Parse for Context {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut impls: HashMap<Ident, Vec<Impl>> = HashMap::new();
        let mut classes: HashMap<Ident, Class> = HashMap::new();

        while !input.is_empty() {
            if input.peek(Token![impl]) {
                let impl_block = input.parse::<Impl>()?;
                let class_name = impl_block.name.clone();
                impls.entry(class_name).or_default().push(impl_block);
            } else {
                let class = input.parse::<Class>()?;
                classes.insert(class.name.clone(), class);
            }
        }

        for impl_name in impls.keys() {
            if !classes.contains_key(impl_name) {
                return Err(syn::Error::new(
                    impl_name.span(),
                    format!("impl block for `{}` has no corresponding struct definition", impl_name)
                ));
            }
        }

        // Join classes with their impls
        let classes = classes.into_iter()
            .map(|(name, class)| {
                let impls = impls.remove(&name).unwrap_or_default();
                (name, (class, impls))
            })
            .collect();

        Ok(Self { classes })
    }
}
