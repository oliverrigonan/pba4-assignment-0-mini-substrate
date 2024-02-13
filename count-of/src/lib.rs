//! A procedural macro to generate count_of methods on collections of
//! arbitrary enums.

// You may uncomment and use the inflector crate.
use inflector::Inflector;
use quote::{quote, format_ident};

#[proc_macro_derive(CountOf)]
pub fn count_of(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse_macro_input!(input as syn::ItemEnum);
    let name = input.ident;

    let trait_name = format_ident!("{}VecExt", name);

    let variants = input.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let variant_count = variant_name.to_string().to_snake_case() + "_count";
        let variant_count_ident = proc_macro2::Ident::new(&variant_count, variant_name.span());

		quote! {
            fn #variant_count_ident(&self) -> usize {
                self.as_ref().iter().filter(|&x| x == &#name::#variant_name).count()
            }
        }
    });

	let output = quote! {
        pub trait #trait_name: AsRef<[#name]> {
            #(#variants)*
        }
        impl<T> #trait_name for T where T: AsRef<[#name]> {}
    };

	proc_macro::TokenStream::from(output)
}
