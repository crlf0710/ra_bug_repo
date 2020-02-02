extern crate proc_macro;
use proc_macro::TokenStream;

pub fn foo(_attrs: TokenStream, input: TokenStream) -> TokenStream {
    let data: syn::Data = syn::parse_macro_input!(input as syn::DeriveInput).data;
    let _struct_data = data.into_data_struct(); // <- ra cannot infer type here
    unimplemented!()
}

use easy_ext::ext;

#[ext(SynDataExt)]
impl syn::Data {
    pub(crate) fn into_data_struct(self) -> Option<syn::DataStruct> {
        unimplemented!()
    }
}
