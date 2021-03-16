// Take a look at the license at the top of the repository in the LICENSE file.

use proc_macro2::TokenStream;
use proc_macro_error::abort_call_site;
use quote::{quote, format_ident};
use heck::{CamelCase, KebabCase, SnakeCase};

pub const WRONG_PLACE_MSG: &str =
    "This macro should be used on `impl` block for a GObject";

pub fn impl_object_ffi(input: &syn::ItemImpl, args: &syn::AttributeArgs) -> TokenStream {
    let crate_ident = crate::utils::crate_ident_new();
    let mut methods = TokenStream::new();

    let mut prefix = Some("fixme_".to_string()); // could derive the prefix from crate name or Cargo option?
    for arg in args {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                if nv.path.is_ident("prefix") {
                    if let syn::Lit::Str(lit) = &nv.lit {
                        prefix = Some(lit.value());
                    }
                }
            }
            _ => {},
        }
    }

    let syn::ItemImpl {
        self_ty,
        items,
        ..
    } = &input;

    let ty = match &**self_ty {
        syn::Type::Path(syn::TypePath { path, .. }) => {
            path.get_ident().unwrap().to_string()
        }
        _ => panic!(),
    };

    let ty_lower = ty.to_snake_case();

    for i in items.iter() {
        if let syn::ImplItem::Method(m) = i {
            let method_name = m.sig.ident.to_string().to_snake_case();
            //let attrs = parse_item_attributes(&m.attrs, "object_impl").unwrap();
            let doc = get_doc_attrs(&m.attrs);
            for i in &m.sig.inputs {
                //dbg!(i);
            }
            let out = return_to_c(&m.sig.output);
            let fn_name = format_ident!("{}{}_{}", prefix.as_ref().unwrap(), ty_lower, method_name);
            let m = quote! {
                #(#doc)*
                #[no_mangle]
                pub unsafe extern "C" fn #fn_name(this: *mut <#self_ty as #crate_ident::subclass::types::ObjectSubclass>::Instance) #out {
                    let imp = (*this).get_impl();
                    todo!()
                }
            };
            methods.extend(m);
        }
    }

    let get_type = format_ident!("{}{}_get_type", prefix.unwrap(), ty_lower);

    quote! {
        #input

        #methods

        #[no_mangle]
        pub extern "C" fn #get_type() -> #crate_ident::ffi::GType {
            #self_ty::get_type().to_glib()
        }
    }
}

fn get_doc_attrs(attrs: &[syn::Attribute]) -> Vec<&syn::Attribute> {
    attrs.iter().filter(|x| x.path.is_ident("doc")).collect()
}

fn return_to_c(t: &syn::ReturnType) -> TokenStream {
    match t {
        syn::ReturnType::Default => quote!{},
        syn::ReturnType::Type(_, t) => {
            let ret = match &**t {
                syn::Type::Array(_) => {todo!()}
                syn::Type::BareFn(_) => {todo!()}
                syn::Type::Group(_) => {todo!()}
                syn::Type::ImplTrait(_) => {todo!()}
                syn::Type::Infer(_) => {todo!()}
                syn::Type::Macro(_) => {todo!()}
                syn::Type::Never(_) => {todo!()}
                syn::Type::Paren(_) => {todo!()}
                syn::Type::Path(_) => {
                    quote!{ #t } //T as GlibPtrDefault>::GlibType;
                }
                syn::Type::Ptr(_) => {todo!()}
                syn::Type::Reference(_) => {todo!()}
                syn::Type::Slice(_) => {todo!()}
                syn::Type::TraitObject(_) => {todo!()}
                syn::Type::Tuple(_) => {todo!()}
                syn::Type::Verbatim(_) => {todo!()}
                syn::Type::__TestExhaustive(_) => {todo!()}
            };
            quote!{ -> #ret }
        }
    }
}
