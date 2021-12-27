//! Convenience re-export of common members
//!
//! Like the standard library's prelude, this module simplifies importing of
//! common items. Unlike the standard prelude, the contents of this module must
//! be imported manually:
//!
//! ```
//! use scrypto_statictypes::prelude::*;
//! ```
pub use crate::XRD;
pub use crate::bucketof::BucketOf;
pub use crate::vaultof::VaultOf;
pub use crate::resourceof::ResourceOf;
pub use crate::bucketrefof::BucketRefOf;
pub use crate::macros::*; // this is for things a macro might generate
pub use crate::{declare_resource}; // this is for the macros themselves, and must be explicitly named (at top level due to #[macro_export])
pub use crate::internal::{Unwrap, WithInner, UncheckedInto}; // to access trait methods with_inner(...) and unchecked_into()
pub use crate::exts::{Deposit, DepositOf, DepositOfExplicit, Withdraw, WithdrawOf, WithdrawOfExplicit};

