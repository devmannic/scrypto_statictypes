pub use crate::internal::{Address, ResourceDecl as StaticResourceDecl, Resource as StaticResource};

#[macro_export]
macro_rules! declare_resource {
    ( type $x:ident ) => {
        impl StaticResource for $x {}
        impl StaticResourceDecl for $x {
            const ADDRESS: Option<Address> = None;
        }
    };
    ( type $x:ident, $e:expr  ) => {
        impl StaticResource for $x {}
        impl StaticResourceDecl for $x {
            const ADDRESS: Option<Address> = Some($e);
        }
    };
    ( $x:ident ) => {
        #[derive(Debug)]
        pub enum $x {}
        impl StaticResource for $x {}
        impl StaticResourceDecl for $x {
            const ADDRESS: Option<Address> = None;
        }
    };
    ( $x:ident, $e:expr  ) => {
        #[derive(Debug)]
        pub enum $x {}
        impl StaticResource for $x {}
        impl StaticResourceDecl for $x {
            const ADDRESS: Option<Address> = Some($e);
        }
    };
}
