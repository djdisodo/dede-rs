//! # dede
//! DErive DEref
//! 
//! there were already some macros for deriving `Deref`
//! but wasn't flexible enough
//! 
//! this macro supports structs with generic types and tuple structs
//! 
//! ```rust
//! use dede::*;
//! 
//! #[derive(Deref, DerefMut)]
//! pub struct Foo {
//! 	#[deref]
//! 	bar: usize
//! }
//! ```



use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Fields, ImplItem, Meta, Token, punctuated::Punctuated, Type};
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Deref, attributes(deref))]
pub fn derive_deref(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse::<DeriveInput>(item).expect("`Deref` macro can only be used for struct");
	if let Data::Struct(data_struct) = input.data {
		let mut field_option: Option<(TokenStream2, &Type)> = None;
		match &data_struct.fields {
			Fields::Named(fields_named) => {
				'a: for field in fields_named.named.iter() {
					for attr in field.attrs.iter() {
						if let Meta::Path(path) =  attr.parse_meta().unwrap() {
							let last = path.segments.last().unwrap();
							let ident = &last.ident;
							if ident.to_string() == "deref" {
								field_option = Some((field.ident.as_ref().unwrap().to_token_stream(), &field.ty));
								break 'a;
							}
						}
					}
				}
			},
			Fields::Unnamed(fields_unnamed) => {
				let mut i = 0;
				'b: for field in fields_unnamed.unnamed.iter() {
					for attr in field.attrs.iter() {
						if let Meta::Path(path) =  attr.parse_meta().unwrap() {
							let last = path.segments.last().unwrap();
							let ident = &last.ident;
							if ident.to_string() == "deref" {
								field_option = Some((syn::Index::from(i).to_token_stream(), &field.ty));
								break 'b;
							}
						}
					}
					i += 1;
				}
			},
			Fields::Unit => {}
		};
		if let Some((field_token, field_type)) = field_option {
			let mut type_args: Punctuated<_, syn::token::Comma> = Punctuated::new();
			for param in input.generics.type_params() {
				type_args.push(param.ident.clone());
			}

			let items = quote! {
				type Target = #field_type;

				fn deref(&self) -> &Self::Target {
					&self.#field_token
				}
			};

			let span = proc_macro2::Span::call_site();
			let ident = &input.ident;
			syn::ItemImpl {
				attrs: vec![],
				defaultness: None,
				unsafety: None,
				impl_token: Token![impl](span),
				generics: input.generics.clone(),
				trait_: Some((None, syn::parse2(quote! { std::ops::Deref }).unwrap(), Token![for](span))),
				self_ty: Box::new(syn::parse2(quote! { #ident<#type_args> }).unwrap()),
				brace_token: syn::token::Brace { span },
				items: vec![ImplItem::Verbatim(items)] 
			}.to_token_stream().into()
		} else {
			proc_macro::TokenStream::new()
		}
	} else {
		proc_macro::TokenStream::new()
	}
}

#[proc_macro_derive(DerefMut, attributes(deref))]
pub fn derive_deref_mut(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = syn::parse::<DeriveInput>(item).expect("`DerefMut` macro can only be used for struct");
	if let Data::Struct(data_struct) = input.data {
		let mut field_option: Option<(TokenStream2, &Type)> = None;
		match &data_struct.fields {
			Fields::Named(fields_named) => {
				'a: for field in fields_named.named.iter() {
					for attr in field.attrs.iter() {
						if let Meta::Path(path) =  attr.parse_meta().unwrap() {
							let last = path.segments.last().unwrap();
							let ident = &last.ident;
							if ident.to_string() == "deref" {
								field_option = Some((field.ident.as_ref().unwrap().to_token_stream(), &field.ty));
								break 'a;
							}
						}
					}
				}
			},
			Fields::Unnamed(fields_unnamed) => {
				let mut i = 0;
				'b: for field in fields_unnamed.unnamed.iter() {
					for attr in field.attrs.iter() {
						if let Meta::Path(path) =  attr.parse_meta().unwrap() {
							let last = path.segments.last().unwrap();
							let ident = &last.ident;
							if ident.to_string() == "deref" {
								field_option = Some((syn::Index::from(i).to_token_stream(), &field.ty));
								break 'b;
							}
						}
					}
					i += 1;
				}
			},
			Fields::Unit => {}
		};
		if let Some((field_token, _field_type)) = field_option {
			let mut type_args: Punctuated<_, syn::token::Comma> = Punctuated::new();
			for param in input.generics.type_params() {
				type_args.push(param.ident.clone());
			}

			let items = quote! {
				fn deref_mut(&mut self) -> &mut Self::Target {
					&mut self.#field_token
				}
			};

			let span = proc_macro2::Span::call_site();
			let ident = &input.ident;
			syn::ItemImpl {
				attrs: vec![],
				defaultness: None,
				unsafety: None,
				impl_token: Token![impl](span),
				generics: input.generics.clone(),
				trait_: Some((None, syn::parse2(quote! { std::ops::DerefMut }).unwrap(), Token![for](span))),
				self_ty: Box::new(syn::parse2(quote! { #ident<#type_args> }).unwrap()),
				brace_token: syn::token::Brace { span },
				items: vec![ImplItem::Verbatim(items)] 
			}.to_token_stream().into()
		} else {
			proc_macro::TokenStream::new()
		}
	} else {
		proc_macro::TokenStream::new()
	}
}
