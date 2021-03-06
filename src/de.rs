use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DeriveInput, Error, Ident,
    Result, Variant,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Enum(enumeration) => derive_enum(&input, enumeration),
        _ => Err(Error::new(Span::call_site(), "only enums are supported")),
    }
}

fn list_field_names(variant: &Variant) -> Vec<Ident> {
    variant
        .fields
        .iter()
        .map(|f| f.ident.clone())
        .enumerate()
        .map(|(i, f)| match f {
            Some(f) => f,
            None => format_ident!("_{}", i),
        })
        .collect::<Vec<_>>()
}

fn impl_deserialize_for_empty_struct(enum_ident: &Ident, name: &Ident) -> TokenStream {
    let dummy = Ident::new(
        &format!("_IMPL_MINIDESERIALIZE_FOR_{}_{}", enum_ident, name),
        Span::call_site(),
    );

    let place_name = format_ident!("__Place_for_{}_{}", enum_ident, name);

    quote! {
        #[allow(non_camel_case_types)]
        struct #name;

        const #dummy: () = {
            #[allow(non_camel_case_types)]
            #[repr(C)]
            struct #place_name {
                out: miniserde::export::Option<#name>,
            }

            impl #place_name {
                fn new(out: &mut miniserde::export::Option<#name>) -> &mut Self {
                    unsafe { &mut *{ out as *mut miniserde::export::Option<#name> as *mut #place_name } }
                }
            }

            impl miniserde::Deserialize for #name {
                fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
                    impl miniserde::de::Visitor for #place_name {
                        fn null(&mut self) -> miniserde::Result<()> {
                            self.out = Some(#name {});
                            Ok(())
                        }
                    }

                    #place_name::new(out)
                }
            }
        };
    }
}

fn variant_builder_impl(enum_ident: &Ident, variant: &Variant) -> TokenStream {
    let data_struct_name = format_ident!("__Data_{}_{}", enum_ident, variant.ident);

    let is_unit = variant.fields.iter().count() == 0;

    let data_struct = if is_unit {
        impl_deserialize_for_empty_struct(enum_ident, &data_struct_name)
    } else {
        let field_names = list_field_names(variant);
        let field_types = variant
            .fields
            .iter()
            .map(|f| f.ty.clone())
            .collect::<Vec<_>>();

        let s = quote! {
            #[allow(non_camel_case_types)]
            #[derive(Deserialize)]
            struct #data_struct_name {
                #(
                    #[allow(non_snake_case)]
                    #field_names: #field_types,
                )*
            }
        };

        s
    };

    quote! {
        #data_struct
    }
}

fn variant_fields_pattern(variant: &Variant) -> TokenStream {
    let variant_name = &variant.ident;
    let is_unit = variant.fields.iter().count() == 0;
    if is_unit {
        return TokenStream::new();
    }

    // Tuple variants doesn't have fields with names
    let is_tuple = variant.fields.iter().filter(|f| f.ident.is_some()).count() == 0;

    let fields = list_field_names(variant);

    if is_tuple {
        quote! {
            (
                #(
                    #variant_name . #fields,
                )*
            )
        }
    } else {
        quote! {
            {
                #(
                    #fields: #variant_name . #fields,
                )*
            }
        }
    }
}

pub fn derive_enum(input: &DeriveInput, enumeration: &DataEnum) -> Result<TokenStream> {
    let ident = &input.ident;
    let dummy = Ident::new(
        &format!("_IMPL_MINIDESERIALIZE_FOR_{}", ident),
        Span::call_site(),
    );

    let names = enumeration
        .variants
        .iter()
        .map(|name| format_ident!("{}", name.ident))
        .collect::<Vec<_>>();

    let variant_builders = enumeration
        .variants
        .iter()
        .map(|variant| variant_builder_impl(ident, variant))
        .collect::<Vec<_>>();

    let var_name = enumeration
        .variants
        .iter()
        .map(|variant| variant.ident.to_string())
        .collect::<Vec<_>>();

    let var_struct_name = enumeration
        .variants
        .iter()
        .map(|variant| format_ident!("__Data_{}_{}", ident, variant.ident))
        .collect::<Vec<_>>();

    let variant_patterns = enumeration
        .variants
        .iter()
        .map(|variant| variant_fields_pattern(variant))
        .collect::<Vec<_>>();

    Ok(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            #(
                #variant_builders
            )*

            #[allow(non_camel_case_types)]
            #[repr(C)]
            struct __TopLevelVisitor {
                __out: miniserde::export::Option<#ident>,
            }

            impl miniserde::Deserialize for #ident {
                fn begin(__out: &mut miniserde::export::Option<Self>) -> &mut dyn miniserde::de::Visitor {
                    unsafe {
                        &mut *{
                            __out
                            as *mut miniserde::export::Option<Self>
                            as *mut __TopLevelVisitor
                        }
                    }
                }
            }

            impl miniserde::de::Visitor for __TopLevelVisitor {
                fn map(&mut self) -> miniserde::Result<miniserde::export::Box<dyn miniserde::de::Map + '_>> {
                    Ok(miniserde::export::Box::new(__TopLevelBuilder {
                        #(
                            #names: miniserde::Deserialize::default(),
                        )*

                        __selected_key: miniserde::Deserialize::default(),
                        __out: &mut self.__out,
                    }))
                }
            }

            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            struct __TopLevelBuilder<'__a> {
                #(
                    #names: miniserde::export::Option<#var_struct_name>,
                )*

                __selected_key: miniserde::export::Option<miniserde::export::String>,
                __out: &'__a mut miniserde::export::Option<#ident>,
            }

            impl<'__a> miniserde::de::Map for __TopLevelBuilder<'__a> {
                fn key(&mut self, __k: &miniserde::export::str) -> miniserde::Result<&mut dyn miniserde::de::Visitor> {
                    match __k {
                        #(
                            #var_name => {
                                self.__selected_key = Some(__k.to_owned());
                                miniserde::export::Ok(miniserde::Deserialize::begin(&mut self.#names))
                            }
                        )*

                        _ => {
                            self.__selected_key = None;
                            miniserde::export::Ok(miniserde::de::Visitor::ignore())
                        }
                    }
                }

                fn finish(&mut self) -> miniserde::Result<()> {
                    match self.__selected_key {
                        #(
                            Some(ref s) if s == #var_name => {
                                #[allow(non_snake_case, unused_variables)]
                                let #names = self.#names.take().ok_or(miniserde::Error)?;

                                *self.__out = miniserde::export::Some(#ident :: #names
                                    #variant_patterns
                                );

                                miniserde::export::Ok(())
                            },
                        )*

                        _ => Err(miniserde::Error),
                    }
                }
            }
        };
    })
}
