/// Make easier to register setup functions
#[macro_export]
macro_rules! setup {
    ($manager:ident, |$con:ident| $stmt:expr) => {
        $manager.add_setup(|$con| -> Result<(), String> {
            $stmt;
            Ok(())
        })
    };

    ($manager:ident, |$con:ident| { $($stmt:expr)* }) => {
        $manager.add_setup(|$con| -> Result<(), String> {
            $($stmt)*
            Ok(())
        })
    };

    ($manager:ident, $callback:tt) => {
        $manager.add_setup($callback)
    }
}
