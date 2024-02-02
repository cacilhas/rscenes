/// Make easier to register setup functions
#[macro_export]
macro_rules! setup {
    (|$con:ident| { $($stmt:expr)* }) => {
        |$con: $crate::prelude::PlainConnector| -> Result<(), String> {
            $($stmt)*
            Ok(())
        }
    };

    (|$con:ident| $stmt:expr) => {
        |$con: $crate::prelude::PlainConnector| -> Result<(), String> {
            $stmt;
            Ok(())
        }
    };

    (move |$con:ident| { $($stmt:expr)* }) => {
        move |$con: $crate::prelude::PlainConnector| -> Result<(), String> {
            $($stmt)*
            Ok(())
        }
    };

    (move |$con:ident| $stmt:expr) => {
        move |$con: $crate::prelude::PlainConnector| -> Result<(), String> {
            $stmt;
            Ok(())
        }
    };
}
