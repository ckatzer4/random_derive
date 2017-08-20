extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;

#[proc_macro_derive(RandTrait)]
pub fn derive_rand(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse the string representation into a syntax tree
    let ast = syn::parse_derive_input(&source).unwrap();

    // Build the output
    let expanded = expand_rand(&ast);

    // Return the generated impl as a token stream
    expanded.parse().unwrap()
}

fn expand_rand(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let rand_body = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref body)) => {
            let fields = body.iter()
                .filter_map(|field| field.ident.as_ref())
                .map(|ident| quote! { #ident: rand::random() })
                .collect::<Vec<_>>();
            quote! { #name { #(#fields),* } }
        },
        syn::Body::Struct(syn::VariantData::Tuple(ref body)) => {
            let fields = (0..body.len())
                .map(syn::Ident::from)
                .map(|_| quote! { rand::random() })
                .collect::<Vec<_>>();
            quote! { #name ( #(#fields),* ) }
        },
        syn::Body::Struct(syn::VariantData::Unit) => {
            quote! { #name }
        },
        syn::Body::Enum(ref body) => {
            let mut variants = body.iter().enumerate()
                .map(|(i, variant)| parse_enum_variant(&variant, &name, Some(i)))
                .collect::<Vec<_>>();

            let num = variants.len();

            // We also need to include the default case, which is the first variant
            let default_case = &body[0].clone();
            let default_tokens = parse_enum_variant(default_case, &name, None);
            variants.push(default_tokens);
                    
            quote! { 
                let r: usize = rand::random::<usize>() % #num;
                match r { 
                    #(#variants),* 
                } 
            }
        },
    };

    quote! {
        impl Rand for #name {
            fn rand<R: rand::Rng>(rng: &mut R) -> #name { #rand_body }
        }
    }

}

fn parse_enum_variant(
            variant: &syn::Variant, 
            name: &syn::Ident, 
            index: Option<usize>,
        ) -> quote::Tokens {
    // Expands a single variant of an enum into tokens
    let unqualified_ident = &variant.ident;
    let ident = quote! { #name::#unqualified_ident };
    match variant.data {
        syn::VariantData::Struct(ref body) => {
            let idents = body.iter()
                .filter_map(|field| field.ident.as_ref())
                .collect::<Vec<_>>();;
            let random_idents = idents.iter()
                .map(|ident| quote! { #ident: rand::random() })
                .collect::<Vec<_>>();
            match index {
                Some(i) => quote! { #i => #ident { #(#random_idents),* } },
                None => quote! { _ => #ident { #(#random_idents),* } },
            }
        },
        syn::VariantData::Tuple(ref body) => {
            let idents = (0..body.len())
                .map(|index| syn::Ident::from(format!("x{}", index)))
                .collect::<Vec<_>>();
            let random_idents = idents.iter()
                .map(|_| quote! { rand::random() })
                .collect::<Vec<_>>();
            match index {
                Some(i) => quote! { #i => #ident ( #(#random_idents),* ) },
                None => quote! { _ => #ident ( #(#random_idents),* ) },
            }
        },
        syn::VariantData::Unit => {
            match index {
                Some(i) => quote! { #i => #ident },
                None => quote! { _ => #ident },
            }
        },
    }
}
