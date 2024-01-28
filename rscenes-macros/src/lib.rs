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

    let body = Stmt::Expr(
        parse2(quote! {
            #(#original)*
        })
        .unwrap(),
        None,
    );

    let output = match attr.as_str() {
        "2d" => quote! {
            fn draw_2d(&self, connector: &rscenes::prelude::Connector2D) -> Result<(), String> {
                let camera = self.get_camera_2d(),
                rcore.begin_mode_2D(camera);
                let res = self.__draw_2d(
                    connector.rcore,
                    connector.rgestures,
                    connector.rshapes,
                    connector.rtextures,
                    connector.rtext,
                    connector.raudio,
                    camera,
                );
                rcore.end_mode_2D();
                res
            }

            fn __draw_2d(
                &self,
                rcore: rscenes::prelude::Rcore,
                rgestures: rscenes::prelude::Rgestures,
                rshapes: rscenes::prelude::Rshapes,
                textures: rscenes::prelude::Rtextures,
                rtext: rscenes::prelude::Rtext,
                raudio: rscenes::prelude::Raudio,
                camera: Box<rscenes::prelude::Camera2D>,
            ) -> Result<(), String {
                #body
                Ok(())
            }
        },
        "3d" => quote! {
            fn draw_3d(&self, connector: &rscenes::prelude::Connector3D) -> Result<(), String> {
                let rcore = connector.recore;
                let rgestures = connector.rgestures;
                let rcamera = connector.rcamera;
                let rtextures = connector.rtextures;
                let rmodels = connector.rmodels;
                let raudio = connector.raudio;
                let camera = self.get_camera_3d();

                rcore.begin_mode_3D(camera);
                #body
                rcore.end_mode_3D();
                Ok(())
            }

            fn draw_3d(&self, connector: &rscenes::prelude::Connector3D) -> Result<(), String> {
                let camera = self.get_camera_3d(),
                rcore.begin_mode_3D(camera);
                let res = self.__draw_3d(
                    connector.rcore,
                    connector.rgestures,
                    connector.rcamera,
                    connector.rtextures,
                    connector.rmodels,
                    connector.raudio,
                    camera,
                );
                rcore.end_mode_3D();
                res
            }

            fn __draw_3d(
                &self,
                rcore: rscenes::prelude::Rcore,
                rgestures: rscenes::prelude::Rgestures,
                rcamera: rscenes::prelude::Rcamera,
                textures: rscenes::prelude::Rtextures,
                rmodels: rscenes::prelude::Rmodels,
                raudio: rscenes::prelude::Raudio,
                camera: Box<rscenes::prelude::Camera2D>,
            ) -> Result<(), String {
                #body
                Ok(())
            }
        },
        unknown => panic!("unpexpected attribute {}", unknown),
    };

    output.into()
}
