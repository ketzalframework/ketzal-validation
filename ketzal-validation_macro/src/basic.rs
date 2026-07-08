use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, parse_macro_input};

use crate::utils::is_option_type;

pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let mut validators = Vec::new();

    let Data::Struct(data) = &input.data else {
        return syn::Error::new_spanned(&input.ident, "KetzalBasicM only supports structs")
            .to_compile_error()
            .into();
    };

    let Fields::Named(fields) = &data.fields else {
        return syn::Error::new_spanned(&input.ident, "KetzalBasicM requires named fields")
            .to_compile_error()
            .into();
    };

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_str = field_name.to_string();
        let is_option = is_option_type(&field.ty);

        for attr in &field.attrs {
            if !attr.path().is_ident("rule") {
                continue;
            }

            let rule = match attr.parse_args::<LitStr>() {
                Ok(v) => v.value(),
                Err(e) => return e.to_compile_error().into(),
            };

            for part in rule.split('|') {
                let mut pieces = part.split(':');
                let rule_name = pieces.next().unwrap();
                let args: Vec<String> = pieces
                    .next()
                    .map(|rest| rest.split(',').map(|v| v.to_string()).collect())
                    .unwrap_or_default();
                let args_tokens = args.iter().map(|v| quote! { #v.to_string() });

                if rule_name == "confirmed" {
                    let confirmation_name = Ident::new(
                        &format!("{}_confirmation", field_name_str),
                        field_name.span(),
                    );
                    validators.push(quote! {
                        if self.#field_name != self.#confirmation_name {
                            errors.push(
                                #field_name_str,
                                "confirmed",
                                ketzal_validation::i18n::t(
                                    "validator.confirmed.mismatch",
                                    &[("field", #field_name_str)],
                                ),
                            );
                        }
                    });
                } else if is_option {
                    validators.push(quote! {
                        if let Some(ref val) = self.#field_name {
                            if let Some(validator) = registry.get(#rule_name) {
                                if let Err(err) = validator.validate(
                                    #field_name_str,
                                    val,
                                    &[#(#args_tokens),*],
                                ) {
                                    errors.extend(err);
                                }
                            }
                        }
                    });
                } else {
                    validators.push(quote! {
                        if let Some(validator) = registry.get(#rule_name) {
                            if let Err(err) = validator.validate(
                                #field_name_str,
                                &self.#field_name,
                                &[#(#args_tokens),*],
                            ) {
                                errors.extend(err);
                            }
                        }
                    });
                }
            }
        }
    }

    let expanded = quote! {
        impl #struct_name {
            pub fn validate(
                &self,
            ) -> Result<(), ketzal_validation::errors::ValidationErrors> {
                let registry =
                    ketzal_validation::registry::ValidatorRegistry::global();

                let mut errors =
                    ketzal_validation::errors::ValidationErrors::new();

                #(#validators)*

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    expanded.into()
}
