/// Make easier to register setup functions
#[macro_export]
macro_rules! setup {
    (|$con:ident| { $($stmt:expr)* }) => {
        |$con| -> Result<(), String> {
            $($stmt)*
            Ok(())
        }
    };

    (|$con:ident| $stmt:expr) => {
        |$con| -> Result<(), String> {
            $stmt;
            Ok(())
        }
    };

    (move |$con:ident| { $($stmt:expr)* }) => {
        move |$con| -> Result<(), String> {
            $($stmt)*
            Ok(())
        }
    };

    (move |$con:ident| $stmt:expr) => {
        move |$con| -> Result<(), String> {
            $stmt;
            Ok(())
        }
    };
}
