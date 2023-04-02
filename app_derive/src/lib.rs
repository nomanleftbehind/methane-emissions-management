// // mod attributes;

// use proc_macro2::TokenStream;
// use quote::{quote, quote_spanned};
// use syn::spanned::Spanned;
// use syn::{
//     parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
// };

// #[proc_macro_derive(FigureOut, attributes(loader))]
// pub fn derive_heap_size(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     // Parse the input tokens into a syntax tree.
//     let input = parse_macro_input!(input as DeriveInput);

//     // Used in the quasi-quotation below as `#name`.
//     let name = input.ident;

//     let attributes = input.attrs;

//     // Add a bound `T: HeapSize` to every type parameter T.
//     let generics = add_trait_bounds(input.generics);
//     let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

//     // Generate an expression to sum up the heap size of each field.
//     // let sum = heap_size_sum(&input.data);

//     let expanded = quote! {
//         // The generated impl.
//         impl #impl_generics FigureOut for #name #ty_generics #where_clause {
//             fn printff(&self) {
//                 println!("self: {:#?}, name: {:#?}", self, stringify!(#name));
//             }
//         }
//     };

//     // Hand the output tokens back to the compiler.
//     proc_macro::TokenStream::from(expanded)
// }

// // Add a bound `T: HeapSize` to every type parameter T.
// fn add_trait_bounds(mut generics: Generics) -> Generics {
//     for param in &mut generics.params {
//         if let GenericParam::Type(ref mut type_param) = *param {
//             type_param.bounds.push(parse_quote!(heapsize::HeapSize));
//         }
//     }
//     generics
// }

// fn build_query(input: &syn::DeriveInput) -> Result<String, syn::Error> {
//     let query = attributes::extract_attr(input, "query")?;

//     Ok(query)
// }
