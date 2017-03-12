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
    let shrink_body = shrink(&ast);

    let name = &ast.ident;
    let output = quote! {
        impl #impl_generics Arbitrary for #name #ty_generics #where_clause {
            #[allow(unused_variables)]
            fn arbitrary<G: Gen>(#gen: &mut G) -> Self {
                #arbitrary_body
            }

            fn shrink(&self) -> Box<Iterator<Item=Self>> {
                #shrink_body
            }
        }
    };

    output.parse().unwrap()
}

fn arbitrary(ast: &syn::DeriveInput, gen: &syn::Ident) -> quote::Tokens {
    let ty = &ast.ident;

    match ast.body {
        syn::Body::Enum(ref variants) => {
            let len = variants.len();
            let mut cases = variants.iter().enumerate()
                .map(|(i, variant)| {
                    let ident = syn::Ident::new(ty.to_string() +
                                                "::" +
                                                &variant.ident.to_string());
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
        syn::Body::Struct(ref data) => arbitrary_variant(&ty, &gen, data),
    }
}

fn arbitrary_variant(ident: &syn::Ident, gen: &syn::Ident,
                     data: &syn::VariantData) -> quote::Tokens {
    match *data {
        syn::VariantData::Struct(ref fields) => {
            let f = fields.iter()
                .filter_map(|field| {
                    field.ident.as_ref().map(|ident| {
                        let ty = type_associated_func(&field.ty);
                        quote! { #ident: #ty::arbitrary(#gen) }
                    })
                }).collect::<Vec<_>>();
            quote! { #ident { #(#f),* } }
        },
        syn::VariantData::Tuple(ref fields) => {
            let f = fields.iter()
                .map(|field| {
                    let ty = type_associated_func(&field.ty);
                    quote! { #ty::arbitrary(#gen) }
                }).collect::<Vec<_>>();
            quote! { #ident ( #(#f),* ) }
        },
        syn::VariantData::Unit => quote! { #ident },
    }
}

/// Transform types with a generic paramter `A<B>` into the token sequence
/// `A::<B>` in order to call associated functions.
fn type_associated_func(ty: &syn::Ty) -> quote::Tokens {
    let q = quote!(#ty).to_string();
    let mut output = quote::Tokens::new();

    match q.find('<') {
        Some(pos) => {
            let (a, b) = q.split_at(pos);
            output.append(a);
            output.append("::");
            output.append(b);
        },
        None => output.append(&q)
    }

    output
}


fn shrink(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    match ast.body {
        syn::Body::Struct(ref data) => shrink_struct_variant(name, data),
        syn::Body::Enum(ref variants) => {
            let cases = variants.iter()
                .map(|variant| shrink_enum_variant(name, variant))
                .collect::<Vec<_>>();
            quote! { match *self { #(#cases),* } }
        }
    }
}

fn shrink_enum_variant(ty: &syn::Ident, variant: &syn::Variant)
                       -> quote::Tokens {
    let unqualified_ident = &variant.ident;
    let ident = quote! { #ty::#unqualified_ident };

    match variant.data {
        syn::VariantData::Unit => quote! {
            #ident => quickcheck::empty_shrinker()
        },
        syn::VariantData::Tuple(ref fields) => {
            if fields.len() == 1 {
                return quote!{
                    #ident(ref x) => Box::new(x.shrink().map(#ident))
                };
            }

            let names = (0..fields.len())
                .map(|i| syn::Ident::new(format!("x{}", i)))
                .collect::<Vec<_>>();
            let tuple = quote!{ #(#names),* };

            quote!{
                #ident(#tuple) => {
                    Box::new((#tuple).shrink().map(|(#tuple)| #ident(#tuple)))
                }
            }
        }
        syn::VariantData::Struct(ref fields) => {
            if fields.len() == 1 {
                let field = &fields[0].ident;
                return quote!{
                    #ident{ref #field} => Box::new(
                        #field.shrink().map(|#field| #ident{#field: #field})
                    )
                };
            }

            let mut names = Vec::with_capacity(fields.len());
            let mut parts = Vec::with_capacity(fields.len());

            for field in fields {
                if let Some(ident) = field.ident.as_ref() {
                    names.push(ident);
                    parts.push(quote! { #ident: #ident });
                }
            }
            let tuple = quote!{ #(#names),* };
            let destructure = quote! { #ident { #(#parts),* } };

            quote! {
                #ident{ #tuple } => {
                    Box::new((#tuple).shrink().map(|(#tuple)| #destructure))
                }
            }
        }
    }
}

fn shrink_struct_variant(ty: &syn::Ident, data: &syn::VariantData)
                         -> quote::Tokens {
    match *data {
        syn::VariantData::Struct(ref fields) => {
            if fields.len() == 1 {
                let ident = &fields[0].ident;

                return quote! {Box::new(
                    self.#ident.shrink().map(|#ident| #ty { #ident: #ident })
                )};
            }

            let mut names = Vec::with_capacity(fields.len());
            let mut parts = Vec::with_capacity(fields.len());
            let mut tuple = Vec::with_capacity(fields.len());

            for field in fields {
                let ident = &field.ident;
                names.push(quote! { #ident });
                parts.push(quote! { #ident: #ident });
                tuple.push(quote! { self.#ident });
            }

            quote! {Box::new(
                ( #(#tuple),* ).shrink()
                    .map(|( #(#names),* )| #ty { #(#parts),* })
            )}
        },
        syn::VariantData::Tuple(ref fields) => {
            if fields.len() == 1 {
                return quote! { Box::new(self.0.shrink().map(#ty)) };
            }

            let mut tuple = Vec::with_capacity(fields.len());
            let mut names = Vec::with_capacity(fields.len());

            for i in 0..fields.len() {
                let index = syn::Ident::new(i);

                tuple.push(quote!{ self.#index });
                names.push(syn::Ident::new(format!("x{}", i)));
            }

            let destructure = quote!{ #(#names),* };
            quote! {Box::new(
                ( #(#tuple),* ).shrink().map(|(#destructure)| #ty (#destructure))
            )}
        },
        syn::VariantData::Unit => quote! { quickcheck::empty_shrinker() },
    }
}
