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
}
