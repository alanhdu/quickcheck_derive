extern crate proc_macro;
extern crate quickcheck;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Arbitrary)]
pub fn impl_arbitrary(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = syn::Ident::new("g");
    let arbitrary_body = arbitrary(&ast, &gen);

    let name = &ast.ident;
    let output = quote! {
        impl #impl_generics Arbitrary for #name #ty_generics #where_clause {
            #[allow(unused_variables)]
            fn arbitrary<G: Gen>(#gen: &mut G) -> Self {
                #arbitrary_body
            }
        }
    };

    output.parse().unwrap()
}

fn arbitrary(ast: &syn::DeriveInput, gen: &syn::Ident) -> quote::Tokens
{
    let n = &ast.ident;
    let name = quote! { #n };

    match ast.body {
        syn::Body::Enum(ref variants) => {
            let len = variants.len();
            let mut cases = variants.iter()
                .enumerate()
                .map(|(i, variant)| {
                    let unqualified_ident = &variant.ident;
                    let ident = quote! { #name::#unqualified_ident };
                    let body = arbitrary_variant(&ident, &gen, &variant.data);
                    quote! { #i => #body }
                }).collect::<Vec<_>>();
            cases.push(quote! { _ => unreachable!() });

            quote! {
                assert!(#gen.size() >= #len);
                match #gen.gen_range::<usize>(0, #len) {
                    #(#cases),*
                }
            }
        },
        syn::Body::Struct(ref data) => arbitrary_variant(&name, &gen, data),
    }
}

fn arbitrary_variant(ident: &quote::Tokens, gen: &syn::Ident,
                     data: &syn::VariantData) -> quote::Tokens {
    match *data {
        syn::VariantData::Struct(ref fields) => {
            let f = fields.iter()
                .filter_map(|field| {
                    field.ident.as_ref().map(|ident| {
                        let ty = &field.ty;
                        quote! { #ident: #ty::arbitrary(#gen) }
                    })
                })
            .collect::<Vec<_>>();
            quote! { #ident { #(#f),* } }
        },
        syn::VariantData::Tuple(ref fields) => {
            let f = fields.iter()
                .map(|field| {
                    let ty = &field.ty;
                    quote! { #ty::arbitrary(#gen) }
                })
            .collect::<Vec<_>>();
            quote! { #ident ( #(#f),* ) }
        },
        syn::VariantData::Unit => quote! {
            #ident
        },
    }
}
