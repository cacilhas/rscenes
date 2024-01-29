extern crate proc_macro;

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn draw(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let original = input.block.stmts;

    let attr: DrawType = match attr.into_iter().next() {
        Some(TokenTree::Ident(attr)) => attr.to_string().into(),
        Some(attr) => panic!("unexpected attribute {:?}", attr),
        None => panic!("draw macro expects one attribute: 2d or 3d"),
    };

    let output = match attr {
        DrawType::Draw2D => quote! {

            fn draw_2d(&self, connector: rscenes::prelude::Connector2D) -> Result<(), String> {
                let camera = self.get_camera_2d();
                let rcore = connector.rcore;
                rcore.begin_mode_2d(camera);

                let res = || -> Result<(), String> {
                    let rgestures = connector.rgestures;
                    let rshapes = connector.rshapes;
                    let rtextures = connector.rtextures;
                    let rtext = connector.rtext;
                    let raudio = connector.raudio;
                    #(#original)*
                    Ok(())
                }();

                rcore.end_mode_2d();
                res
            }
        },
        DrawType::Draw3D => quote! {

            fn draw_3d(&self, connector: rscenes::prelude::Connector3D) -> Result<(), String> {
                let camera = self.get_camera_3d();
                let rcore = connector.rcore;
                rcore.begin_mode_3d(camera);

                let res = || -> Result<(), String> {
                    let gestures = connector.rgestures;
                    let rcamera = connector.rcamera;
                    let rtextures = connector.rtextures;
                    let rmodels = connector.rmodels;
                    let raudio = connector.raudio;
                    #(#original)*
                    Ok(())
                }();

                rcore.end_mode_3d();
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

impl From<String> for DrawType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "shapes" => DrawType::Draw2D,
            "models" => DrawType::Draw3D,
            value => panic!("unexpected attribute: draw({})", value),
        }
    }
}
