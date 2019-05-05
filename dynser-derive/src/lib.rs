#![recursion_limit = "192"]

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::Error;
use syn::spanned::Spanned;
use syn::{Data, Field, PathSegment, Type};
use synstructure::decl_derive;

decl_derive!([Object, attributes(default, primitive, any)] => derive_object);

#[allow(clippy::needless_pass_by_value)]
fn derive_object(s: synstructure::Structure) -> TokenStream {
    match derive_object_inner(&s) {
        Err(err) => err.to_compile_error(),
        Ok(tokens) => tokens,
    }
}

#[allow(clippy::needless_pass_by_value)]
fn derive_object_inner(s: &synstructure::Structure) -> Result<TokenStream, Error> {
    let ds = match s.ast().data {
        Data::Struct(ref ds) => ds,
        _ => return Err(Error::new(s.ast().span(), "can only derive on structs")),
    };

    let mut create_body = TokenStream::new();
    let mut default_field = None;
    for field in ds.fields.iter() {
        let ident = field.ident.as_ref().ok_or_else(|| {
            Error::new(
                field.ident.span(),
                "can only derive on structs with named fields",
            )
        })?;
        let field_name = field_name(field);

        let is_default = field.attrs.iter().any(|attr| attr.path.is_ident("default"));

        if is_default {
            if default_field.is_some() {
                return Err(Error::new(
                    field.ident.span(),
                    "can only have one #[default] field",
                ));
            }
            default_field = Some(ident);
            continue;
        }

        let is_primitive = field
            .attrs
            .iter()
            .any(|attr| attr.path.is_ident("primitive"));
        let is_any = field.attrs.iter().any(|attr| attr.path.is_ident("any"));

        let is_vec = is_wrapped_by(&field.ty, "Vec");
        let is_option = is_wrapped_by(&field.ty, "Option");

        // Which FieldMutReflection:: variant to use.
        let tag = if is_vec {
            quote!(List)
        } else if is_primitive {
            quote!(Primitive)
        } else if is_any {
            quote!(Any)
        } else {
            quote!(Object)
        };

        if is_option {
            let span = ident.span();
            create_body.extend(quote_spanned!(span =>
              #field_name => {
                match self.#ident {
                  Some(ref mut v) => FieldMutReflection::#tag(v),
                  None => {
                    self.#ident = Some(Default::default());
                    FieldMutReflection::#tag(self.#ident.as_mut().unwrap())
                  }
                }
              }
            ));
        } else {
            let span = ident.span();
            create_body.extend(quote_spanned!(span =>
              #field_name => FieldMutReflection::#tag(&mut self.#ident),
            ));
        }
    }

    let default_match = default_field
        .map(|ident| {
            let span = ident.span();
            quote_spanned!(span => name => { return self.#ident.create(name) })
        })
        .unwrap_or_else(|| {
            quote! {
                _ => { return Err(ReflectionError::InvalidField { name: field_name.to_string() }) }
            }
        });

    Ok(s.gen_impl(quote! {
        use crate::reflection::{Object, FieldMutReflection, ReflectionError};

        #[allow(unreachable_code)]
        gen impl Object for @Self {
            fn create(&mut self, field_name: &str) -> Result<FieldMutReflection, ReflectionError> {
                Ok(match field_name {
                    #create_body
                    #default_match
                })
            }
        }
    }))
}

fn field_name(field: &Field) -> String {
    let ident = field.ident.as_ref().expect("field name expected");
    ident.to_string().trim_start_matches("r#").to_owned()
}

fn only_segment(ty: &Type) -> Option<&PathSegment> {
    match ty {
        Type::Path(tp) if tp.path.segments.len() == 1 => {
            Some(tp.path.segments.first().as_ref().unwrap().value())
        }
        _ => None,
    }
}

fn is_wrapped_by(ty: &Type, type_name: &str) -> bool {
    only_segment(ty).map_or(false, |segment| segment.ident == type_name)
}
