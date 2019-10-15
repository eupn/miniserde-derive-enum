use crate::{attr, bound};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse_quote, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed, Ident, Result, Variant};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Enum(enumeration) => derive_enum(&input, enumeration),
        _ => Err(Error::new(
            Span::call_site(),
            "only enums are supported",
        )),
    }
}

fn variant_fragment(enum_ident: &Ident, variant: &Variant) -> TokenStream {
    let stream_name = Ident::new(
        &format!("__{}_{}_VARIANT_STREAM", enum_ident, variant.ident),
                 Span::call_site());

    let struct_name = Ident::new(
        &format!("__{}_{}_VARIANT_STRUCT", enum_ident, variant.ident),
        Span::call_site());

    let data_struct_fields = variant_fields_pattern(false, &variant);

    quote! {
        miniserde::ser::Fragment::Map(Box::new(#stream_name {
            data: #struct_name {
                #data_struct_fields
            },
            state: 0,
        }))
    }
}

fn variant_body_impl(enum_ident: &Ident, variant: &Variant) -> TokenStream {
    let variant_name = &variant.ident.to_string();
    let stream_name = Ident::new(
        &format!("__{}_{}_VARIANT_STREAM", enum_ident, variant.ident),
        Span::call_site());
    let struct_name = Ident::new(
        &format!("__{}_{}_VARIANT_STRUCT", enum_ident, variant.ident),
        Span::call_site());

    let is_unit = variant.fields.iter().count() == 0;

    let (data, data_struct, data_struct_lifetime) = if is_unit {
        let s = quote! {
            struct #struct_name;
        };

        (quote!(&()), s, TokenStream::new())
    } else {
        let field_names = variant.fields
            .iter()
            .map(|f| f.ident.as_ref())
            .flatten()
            .collect::<Vec<_>>();

        let field_types = variant.fields
            .iter()
            .map(|f| f.ty.clone())
            .collect::<Vec<_>>();

        let s = quote! {
            #[derive(Serialize)]
            struct #struct_name<'a> {
                #(
                    #field_names: &'a #field_types,
                )*
            }
        };

        let lifetime = quote! {
            <'__a>
        };

        (quote!(&self.data), s, lifetime)
    };

    quote! {
        #data_struct

        struct #stream_name #data_struct_lifetime {
            data: #struct_name #data_struct_lifetime,
            state: usize,
        }

        impl #data_struct_lifetime miniserde::ser::Map for #stream_name #data_struct_lifetime {
            fn next(&mut self) -> miniserde::export::Option<(miniserde::export::Cow<miniserde::export::str>, &dyn miniserde::Serialize)> {
                let __state = self.state;
                self.state = __state + 1;
                match __state {
                    0usize => {
                        Some((miniserde::export::Cow::Borrowed(#variant_name), #data))
                    },

                    _ => miniserde::export::None,
                }
            }
        }
    }
}

fn variant_fields_pattern(by_ref: bool, variant: &Variant) -> TokenStream {
    let is_unit = variant.fields.iter().count() == 0;
    if is_unit {
        return TokenStream::new()
    }

    let fields = variant.fields
        .iter()
        .map(|f| f.ident.as_ref())
        .flatten()
        .collect::<Vec<_>>();

    if by_ref {
        quote! {
            #(
                ref #fields,
            )*
        }
    } else {
        quote! {
            #(
                #fields,
            )*
        }
    }
}

fn derive_enum(input: &DeriveInput, enumeration: &DataEnum) -> Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let dummy = Ident::new(
        &format!("_IMPL_MINISERIALIZE_FOR_{}", ident),
        Span::call_site(),
    );

    let index = 0usize..;

    let wrapper_generics = bound::with_lifetime_bound(&input.generics, "'__a");
    let (wrapper_impl_generics, wrapper_ty_generics, _) = wrapper_generics.split_for_impl();
    let bound = parse_quote!(miniserde::Serialize);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    let var_idents = enumeration
        .variants
        .iter()
        .map(|variant| &variant.ident)
        .collect::<Vec<_>>();

    let names = enumeration
        .variants
        .iter()
        .map(attr::name_of_variant)
        .collect::<Result<Vec<_>>>()?;

    let variant_impl = enumeration
        .variants
        .iter()
        .map(|variant| variant_fragment(ident, variant))
        .collect::<Vec<_>>();

    let variant_streams = enumeration
        .variants
        .iter()
        .map(|variant| variant_body_impl(ident, variant))
        .collect::<Vec<_>>();

    let variant_pattern_fields = enumeration
        .variants
        .iter()
        .map(|variant| variant_fields_pattern(true, variant))
        .collect::<Vec<_>>();

    let tokens = quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            #(
                #variant_streams
            )*

            impl miniserde::Serialize for #ident {
                fn begin(&self) -> miniserde::ser::Fragment {
                    match self {
                        #(
                            #ident::#var_idents { #variant_pattern_fields } => {
                                #variant_impl
                            }
                        )*
                    }
                }
            }
        };
    };

    Ok(tokens)
}
