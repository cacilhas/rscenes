extern crate proc_macro;

use core::panic;

use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse2, parse_macro_input, ItemFn, Stmt};

#[proc_macro_attribute]
pub fn draw(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let original = input.block.stmts;

    let attr = match attr.into_iter().next() {
        Some(TokenTree::Literal(data)) => data.to_string(),
        _ => panic!("unexpected syntax"),
    };

    let body = vec![
        Stmt::Expr(
            match attr.as_str() {
                "2d" => parse2(quote! {
                    let rcore = connector.recore;
                    let rgestures = connector.rgestures;
                    let rshapes = connector.rshapes;
                    let rtextures = connector.rtextures;
                    let rtext = connector.rtext;
                    let raudio = connector.raudio;
                    let camera = self.get_camera_2d();
                    rcore.begin_mode_2D(camera);
                })
                .unwrap(),
                "3d" => parse2(quote! {
                    let rcore = connector.recore;
                    let rgestures = connector.rgestures;
                    let rcamera = connector.rcamera;
                    let rtextures = connector.rtextures;
                    let rmodels = connector.rmodels;
                    let raudio = connector.raudio;
                    let camera = self.get_camera_3d();
                    rcore.begin_mode_3D(camera);
                })
                .unwrap(),
                unknown => panic!("unpexpected attribute {}", unknown),
            },
            None,
        ),
        Stmt::Expr(
            parse2(quote! {
            #(#original)*
            })
            .unwrap(),
            None,
        ),
        Stmt::Expr(
            match attr.as_str() {
                "2d" => parse2(quote! {
                    rcore.end_mode_2D();
                })
                .unwrap(),
                "3d" => parse2(quote! {
                    rcore.end_mode_3D();
                })
                .unwrap(),
                unknown => panic!("unpexpected attribute {}", unknown),
            },
            None,
        ),
    ];

    let output = match attr.as_str() {
        "2d" => quote! {
            fn draw_2d(&self, connector: &rscenes::prelude::Connector2D) -> Result<(), String> {
                #(#body)*
                Ok(())
            }
        },
        "3d" => quote! {
            fn draw_3d(&self, connector: &rscenes::prelude::Connector3D) -> Result<(), String> {
                #(#body)*
                Ok(())
            }
        },
        unknown => panic!("unpexpected attribute {}", unknown),
    };

    output.into()
}
