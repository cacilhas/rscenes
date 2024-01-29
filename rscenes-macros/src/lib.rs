extern crate proc_macro;

use proc_macro::{Span, TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, parse_quote, FnArg, Ident, ItemFn, Pat, Type};

#[proc_macro_attribute]
pub fn draw(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let original = input.block.stmts;
    let mut args: Vec<FnArg> = vec![];
    let mut con_name: Option<String> = None;

    let attr: DrawType = match attr.into_iter().next() {
        Some(TokenTree::Ident(attr)) => attr.to_string().into(),
        Some(attr) => panic!("unexpected attribute {:?}", attr),
        None => panic!("draw macro expects one attribute: 2d or 3d"),
    };

    for arg in &input.sig.inputs {
        args.push(arg.clone());
        if let FnArg::Typed(tpe) = arg {
            if let Type::Path(ty) = &*tpe.ty {
                if let Type::Path(con_tpe) = &attr.get_connection_tpe() {
                    // Naïve type comparison approach
                    if ty.path.segments.last().unwrap().ident.to_string()
                        == con_tpe.path.segments.last().unwrap().ident.to_string()
                    {
                        if let Pat::Ident(ident) = &*tpe.pat {
                            con_name = Some(ident.ident.to_string());
                        }
                    }
                }
            }
        }
    }
    let con_name = Ident::new(
        &con_name.expect(&format!(
            "connection not found in fn {} arguments",
            input.sig.ident
        )),
        Span::call_site().into(),
    );

    let output = match attr {
        DrawType::Draw2D => quote! {

            fn draw_2d(#(#args),*) -> Result<(), String> {
                let camera = self.get_camera_2d();
                #con_name.begin_mode_2d(camera);

                let res = || -> Result<(), String> {
                    #(#original)*
                    Ok(())
                }();

                #con_name.end_mode_2d();
                res
            }
        },
        DrawType::Draw3D => quote! {

            fn draw_3d(#(#args),*) -> Result<(), String> {
                let camera = self.get_camera_3d();
                #con_name.begin_mode_3d(camera);

                let res = || -> Result<(), String> {
                    #(#original)*
                    Ok(())
                }();

                #con_name.end_mode_3d();
                res
            }
        },
    };

    output.into()
}

enum DrawType {
    Draw2D,
    Draw3D,
}

impl DrawType {
    fn get_connection_tpe(&self) -> Type {
        match self {
            DrawType::Draw2D => parse_quote!(rscene::prelude::Connector2D),
            DrawType::Draw3D => parse_quote!(rscene::prelude::Connector3D),
        }
    }
}

impl From<String> for DrawType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "shapes" => DrawType::Draw2D,
            "models" => DrawType::Draw3D,
            value => panic!("unexpected attribute: draw({})", value),
        }
    }
}
