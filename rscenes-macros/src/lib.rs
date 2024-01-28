extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn draw(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = parse_macro_input!(item as ItemFn);
    let original = input.block.stmts;

    let attr: DrawType = match attr.into_iter().next() {
        Some(TokenTree::Literal(attr)) => attr.into(),
        Some(attr) => panic!("unexpected attribute {}", attr),
        None => panic!("draw macro expects one attribute: 2d or 3d"),
    };

    let output = match attr {
        DrawType::Draw2D => quote! {
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
                camera: rscenes::prelude::Camera2D,
            ) -> Result<(), String {
                #(#original)*
                Ok(())
            }
        },
        DrawType::Draw3D => quote! {
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
                camera: rscenes::prelude::Camera2D,
            ) -> Result<(), String {
                #(#original)*
                Ok(())
            }
        },
    };

    output.into()
}

enum DrawType {
    Draw2D,
    Draw3D,
}

impl From<Literal> for DrawType {
    fn from(value: Literal) -> Self {
        match value.to_string().as_str() {
            "2d" | "2D" => DrawType::Draw2D,
            "3d" | "3D" => DrawType::Draw3D,
            value => panic!("unexpected attribute: draw({})", value),
        }
    }
}
