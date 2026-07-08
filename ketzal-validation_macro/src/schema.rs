use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitBool, LitStr, parse_macro_input};

use crate::utils::is_option_type;

pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input.ident, "KetzalSchema only supports structs")
            .to_compile_error()
            .into();
    };

    let Fields::Named(fields) = &data.fields else {
        return syn::Error::new_spanned(&input.ident, "KetzalSchema requires named fields")
            .to_compile_error()
            .into();
    };

    let mut schema_entries = Vec::new();

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();

        let mut title: Option<String> = None;
        let mut description: Option<String> = None;
        let mut read_only = false;
        let mut write_only = false;
        let mut default: Option<String> = None;
        let mut deprecated = false;

        for attr in &field.attrs {
            if !attr.path().is_ident("schema") {
                continue;
            }

            if let Err(e) = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("title") {
                    let s: LitStr = meta.value()?.parse()?;
                    title = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("description") {
                    let s: LitStr = meta.value()?.parse()?;
                    description = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("read_only") {
                    let b: LitBool = meta.value()?.parse()?;
                    read_only = b.value;
                    Ok(())
                } else if meta.path.is_ident("write_only") {
                    let b: LitBool = meta.value()?.parse()?;
                    write_only = b.value;
                    Ok(())
                } else if meta.path.is_ident("default") {
                    let s: LitStr = meta.value()?.parse()?;
                    default = Some(s.value());
                    Ok(())
                } else if meta.path.is_ident("deprecated") {
                    let b: LitBool = meta.value()?.parse()?;
                    deprecated = b.value;
                    Ok(())
                } else {
                    Err(meta.error(format!(
                        "unknown schema attribute `{}`",
                        meta.path.require_ident()?,
                    )))
                }
            }) {
                return e.to_compile_error().into();
            }
        }

        let is_required = !is_option_type(&field.ty);

        let title_expr = title
            .map(|t| quote! { Some(#t.to_string()) })
            .unwrap_or(quote! { None });
        let desc_expr = description
            .map(|d| quote! { Some(#d.to_string()) })
            .unwrap_or(quote! { None });
        let default_expr = default
            .map(|d| quote! { Some(serde_json::Value::String(#d.to_string())) })
            .unwrap_or(quote! { None });
        let required_expr = if is_required {
            quote! { vec![#field_name_str.to_string()] }
        } else {
            quote! { vec![] }
        };

        schema_entries.push(quote! {
            map.insert(
                #field_name_str.to_string(),
                ketzal_validation::openapi::schema::OpenApiSchema {
                    name: #field_name_str.to_string(),
                    title: #title_expr,
                    description: #desc_expr,
                    example: None,
                    read_only: #read_only,
                    write_only: #write_only,
                    default: #default_expr,
                    required: #required_expr,
                    deprecated: #deprecated,
                },
            );
        });
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn schema() -> std::collections::HashMap<
                String,
                ketzal_validation::openapi::schema::OpenApiSchema,
            > {
                let mut map = std::collections::HashMap::new();
                #(#schema_entries)*
                map
            }
        }
    };

    expanded.into()
}
