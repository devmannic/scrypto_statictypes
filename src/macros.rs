pub use crate::internal::{
    Address, Resource as StaticResource, ResourceDecl as StaticResourceDecl,
};

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
        #[allow(non_camel_case_types)]
        pub enum $x {}
        impl StaticResource for $x {}
        impl StaticResourceDecl for $x {
            const ADDRESS: Option<Address> = None;
        }
    };
    ( $x:ident, $e:expr  ) => {
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub enum $x {}
        impl StaticResource for $x {}
        impl StaticResourceDecl for $x {
            const ADDRESS: Option<Address> = Some($e);
        }
    };
}
