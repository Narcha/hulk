use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, MetaList, MetaNameValue, Result,
};

#[proc_macro_derive(Partial, attributes(partial, partial_name))]
#[proc_macro_error]
pub fn partial(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_partial(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn derive_partial(input: DeriveInput) -> Result<TokenStream> {
    let DeriveInput {
        attrs, ident, data, ..
    } = input;

    let input_fields = match data {
        Data::Struct(data) => data.fields,
        Data::Enum(data) => abort!(
            data.enum_token,
            "`Partial` can only be derived for `struct`",
        ),
        Data::Union(data) => abort!(
            data.union_token,
            "`Partial` can only be derived for `struct`",
        ),
    };

    let fields = input_fields.iter().map(|field| {
        let ty = &field.ty;
        let ident = field.ident.as_ref().unwrap();
        let attrs = field
            .attrs
            .iter()
            .filter_map(|Attribute { meta, .. }| match meta {
                syn::Meta::List(MetaList { path, tokens, .. }) => path
                    .get_ident()
                    .map(Ident::to_string)
                    .is_some_and(|path| path == "partial")
                    .then_some(quote! {
                        #[#tokens]
                    }),
                _ => None,
            });

        let partial_ty = quote!(<#ty as partial::Partial>::Partial);

        quote! {
            #(#attrs)*
            #ident: Option<#partial_ty>,
        }
    });

    let where_clauses = input_fields.iter().map(|field| {
        let ty = &field.ty;

        quote!(#ty: partial::Partial)
    });

    let partial_attrs = attrs
        .iter()
        .filter_map(|Attribute { meta, .. }| match meta {
            syn::Meta::List(MetaList { path, tokens, .. }) => path
                .get_ident()
                .map(Ident::to_string)
                .is_some_and(|path| path == "partial")
                .then_some(quote! {
                    #[#tokens]
                }),
            _ => None,
        });

    let partial_names = attrs
        .iter()
        .filter_map(|Attribute { meta, .. }| match meta {
            syn::Meta::NameValue(MetaNameValue { path, value, .. }) => path
                .get_ident()
                .is_some_and(|path| path == "partial_name")
                .then_some(value),
            _ => None,
        })
        .collect::<Vec<_>>();

    let partial_ident = match partial_names.as_slice() {
        [] => {
            format_ident!("Partial{ident}")
        }
        [partial_name] => match syn::parse2(partial_name.to_token_stream()) {
            Ok(partial_ident) => partial_ident,
            Err(error) => abort!(partial_name, error),
        },
        [_, second_occurrence, ..] => {
            abort!(
                second_occurrence,
                "`partial_name` attruíbute cannot be used multiple times"
            )
        }
    };

    let apply_partial = generate_apply_partial(&input_fields);

    Ok(quote! {
        #(#partial_attrs)*
        struct #partial_ident where #(#where_clauses),* {
            #(#fields)*
        }

        impl partial::Partial for #ident {
            type Partial = #partial_ident;

            #apply_partial
        }
    })
}

fn generate_apply_partial(fields: &Fields) -> TokenStream {
    let lines = fields.iter().map(|field| {
        let ident = &field.ident;
        quote! {
            if let Some(#ident) = partial.#ident {
                self.#ident.apply_partial(#ident);
            }
        }
    });

    quote! {
        fn apply_partial(&mut self, partial: Self::Partial) {
            #(#lines)*
        }
    }
}
