use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, Ident, LitStr, Token,
    parse::{Parse, ParseStream},
};

use crate::utils::is_option_type;

struct RuleAttr {
    rules: String,
    inline_msgs: Vec<(String, String)>,
}

impl Parse for RuleAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let rules = input.parse::<LitStr>()?.value();

        let mut inline_msgs = Vec::new();
        while input.parse::<Token![,]>().is_ok() {
            let msg = input.parse::<LitStr>()?.value();
            if let Some((rule, text)) = msg.split_once(':') {
                inline_msgs.push((rule.to_string(), text.to_string()));
            }
        }

        Ok(Self { rules, inline_msgs })
    }
}

struct MessagesAttr {
    messages: Vec<(String, String)>,
}

impl Parse for MessagesAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut messages = Vec::new();
        while !input.is_empty() {
            let rule: syn::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let text: LitStr = input.parse()?;
            messages.push((rule.to_string(), text.value()));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(Self { messages })
    }
}

pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
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

        let mut field_messages: Vec<(String, String)> = Vec::new();

        for attr in &field.attrs {
            if attr.path().is_ident("message")
                && let Ok(msgs) = attr.parse_args::<MessagesAttr>()
            {
                field_messages.extend(msgs.messages);
            }
        }

        for attr in &field.attrs {
            if !attr.path().is_ident("rule") {
                continue;
            }

            let rule_attr = match attr.parse_args::<RuleAttr>() {
                Ok(v) => v,
                Err(e) => return e.to_compile_error().into(),
            };

            field_messages.extend(rule_attr.inline_msgs);

            for part in rule_attr.rules.split('|') {
                let mut pieces = part.split(':');
                let rule_name = pieces.next().unwrap();
                let args: Vec<String> = pieces
                    .next()
                    .map(|rest| rest.split(',').map(|v| v.to_string()).collect())
                    .unwrap_or_default();
                let args_tokens = args.iter().map(|v| quote! { #v.to_string() });

                let custom_msg = field_messages
                    .iter()
                    .find(|(r, _)| r == rule_name)
                    .map(|(_, m)| m.clone());

                if rule_name == "confirmed" {
                    let confirmation_name = Ident::new(
                        &format!("{}_confirmation", field_name_str),
                        field_name.span(),
                    );
                    if let Some(ref msg) = custom_msg {
                        validators.push(quote! {
                            if self.#field_name != self.#confirmation_name {
                                let msg = {
                                    let mut m = #msg.to_string();
                                    m = m.replace(":attribute", #field_name_str);
                                    m
                                };
                                errors.push(
                                    #field_name_str,
                                    "confirmed",
                                    msg,
                                );
                            }
                        });
                    } else {
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
                    }
                } else if let Some(ref msg) = custom_msg {
                    let msg_tokens = if args.is_empty() {
                        quote! {{
                            let mut msg = #msg.to_string();
                            msg = msg.replace(":attribute", #field_name_str);
                            msg
                        }}
                    } else {
                        let rule_name_str = rule_name.to_string();
                        quote! {{
                            let mut msg = #msg.to_string();
                            msg = msg.replace(":attribute", #field_name_str);
                            msg = msg.replace(&format!(":{}", #rule_name_str), &args[0]);
                            msg
                        }}
                    };

                    if is_option {
                        validators.push(quote! {
                            if let Some(ref val) = self.#field_name {
                                if let Some(validator) = registry.get(#rule_name) {
                                    let args = [#(#args_tokens),*];
                                    if let Err(mut err) = validator.validate(
                                        #field_name_str, val, &args,
                                    ) {
                                        for e in &mut err.errors {
                                            if e.rule == #rule_name {
                                                e.message = #msg_tokens;
                                            }
                                        }
                                        errors.extend(err);
                                    }
                                }
                            }
                        });
                    } else {
                        validators.push(quote! {
                            if let Some(validator) = registry.get(#rule_name) {
                                let args = [#(#args_tokens),*];
                                if let Err(mut err) = validator.validate(
                                    #field_name_str, &self.#field_name, &args,
                                ) {
                                    for e in &mut err.errors {
                                        if e.rule == #rule_name {
                                            e.message = #msg_tokens;
                                        }
                                    }
                                    errors.extend(err);
                                }
                            }
                        });
                    }
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
