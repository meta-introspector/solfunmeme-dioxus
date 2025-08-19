
// Helper macro for easy component definition
#[macro_export]
macro_rules! define_mcp_component {
    (
        $(#[$attr:meta])*
        $vis:vis async fn $name:ident() -> Result<$ret:ty, $err:ty> $body:block
        $(, $config:tt)*
    ) => {
        #[mcp_component($($config)*)]
        $(#[$attr])*
        $vis async fn $name() -> Result<$ret, $err> $body
    };
}
