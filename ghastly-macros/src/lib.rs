// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttrStyle, Attribute, Expr, ItemFn, Lit, Meta, MetaNameValue};

fn extract_doc_comment(attrs: &[Attribute]) -> impl Iterator<Item = String> + '_ {
    attrs
        .iter()
        .filter(|attr| matches!(attr.style, AttrStyle::Outer))
        .filter(|attr| attr.path().is_ident("doc"))
        .filter_map(|attr| match &attr.meta {
            Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(syn::ExprLit {
                        lit: Lit::Str(s), ..
                    }),
                ..
            }) => Some(s.value()),
            _ => None,
        })
        .flat_map(|s| {
            let lines = s
                .split('\n')
                .map(|s| s.strip_prefix(' ').unwrap_or(s).to_owned())
                .collect::<Vec<_>>();
            lines.into_iter()
        })
}

#[proc_macro_attribute]
pub fn policy(_args: TokenStream, input: TokenStream) -> TokenStream {
    let item_fn = parse_macro_input!(input as ItemFn);
    let doc: String = extract_doc_comment(&item_fn.attrs).fold(String::new(), |a, b| a + &b + "\n");
    let ident = item_fn.sig.ident.clone();
    let policy_name = ident.to_token_stream().to_string();

    quote!(
        #item_fn

        inventory::submit! {
            if #doc.is_empty() {
               crate::Policy::new(#policy_name, #ident)
            } else {
               crate::Policy::new(#policy_name, #ident).with_doc(#doc)
            }
        }
    )
    .into()
}
