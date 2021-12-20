pub use crate::internal::{Address, ResourceDecl};

#[macro_export]
macro_rules! declare_resource {
    ( type $x:ident ) => {
        impl ResourceDecl for $x {
            const ADDRESS: Option<Address> = None;
        }
    };
    ( type $x:ident, $e:expr  ) => {
        impl ResourceDecl for $x {
            const ADDRESS: Option<Address> = Some($e);
        }
    };
    ( $x:ident ) => {
        pub enum $x {}
        impl ResourceDecl for $x {
            const ADDRESS: Option<Address> = None;
        }
    };
    ( $x:ident, $e:expr  ) => {
        pub enum $x {}
        impl ResourceDecl for $x {
            const ADDRESS: Option<Address> = Some($e);
        }
    };
}
