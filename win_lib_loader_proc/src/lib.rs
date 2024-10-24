use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn load_from_dll(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_item as syn::Item);
    let mut item = match input {
        syn::Item::Struct(item_struct) => item_struct,
        _ => {
            return quote! {
                compile_error!("Not a struct.");
            }
            .into();
        }
    };

    let struct_visibility = &item.vis;
    let struct_name = &item.ident;

    let fields = match &mut item.fields {
        syn::Fields::Named(fields_named) => fields_named,
        _ => {
            return quote! {
                compile_error!("Only named fields allowed.");
            }
            .into();
        }
    };
    let functions = fields
        .named
        .iter()
        .filter_map(|field| {
            let function_name = field.ident.as_ref().unwrap();
            let function_type = &field.ty;
            let function_type = match function_type {
                syn::Type::BareFn(type_bare_fn) => type_bare_fn,
                _ => {
                    return None;
                }
            };
            let inputs = &function_type.inputs;
            let output = &function_type.output;

            let field_name = format_ident!("__{}", function_name);
            let field = quote! {
                #field_name : #function_type
            };
            let inputs = inputs
                .iter()
                .enumerate()
                .map(|(i, input)| {
                    let name = &input.name;
                    let ty = &input.ty;
                    let name = match name{
                        Some((name,_)) => name.clone(),
                        None => format_ident!("__arg_{}", i),
                    };
                    (name.clone(), quote! {#name : #ty})
                })
                .collect::<Vec<_>>();
            let input_names = inputs.iter().map(|input| &input.0).collect::<Vec<_>>();
            let inputs = inputs.iter().map(|input| &input.1).collect::<Vec<_>>();
            let implementation = quote! {
                #struct_visibility unsafe extern "C" fn #function_name (&self, #(#inputs),* ) #output {
                    (self.#field_name)( #(#input_names),* )
                }
            };

            let function_name_str = format!("{}", function_name);
            let assignment = quote! {
                let #field_name = _lib.get_function::<#function_type>(#function_name_str)?;
            };
            Some((field, implementation, assignment, field_name))
        })
        .collect::<Vec<_>>();

    let field_definitions = functions.iter().map(|x| &x.0).collect::<Vec<_>>();
    let impl_definitions = functions.iter().map(|x| &x.1).collect::<Vec<_>>();
    let assignments = functions.iter().map(|x| &x.2).collect::<Vec<_>>();
    let field_names = functions.iter().map(|x| &x.3).collect::<Vec<_>>();

    let struct_definition = quote! {
        #struct_visibility struct #struct_name{
            win_library: ::win_lib_loader::WinLibrary,
            #(#field_definitions),*
        }
    };

    let dll_path: proc_macro2::TokenStream = _attr.into();

    let implementation = quote! {
        impl #struct_name{
            #struct_visibility fn new() -> Result<Self, ::win_lib_loader::error::LoaderError>{
                let _lib = ::win_lib_loader::WinLibrary::load(#dll_path)?;
                #(#assignments)*
                Ok(#struct_name{
                    win_library: _lib,
                    #(#field_names),*
                })
            }

            #(#impl_definitions)*
        }
    };

    let full = quote! {
        #struct_definition
        #implementation
    };

    full.into()
}
