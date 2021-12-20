//! Use explicit container types with Scrypto!  Leverage the Rust compiler's type checking to increase security and productivity when developing Radix blueprints.
//!
//! A Scrypto (Rust) library for static types, featuring:
//!
//! - A simple, usable, safe, and (by default) zero-cost API for compile-time
//!     static type checking of resources.
//! - Safe drop in replacements for `Bucket` (use: `BucketOf<MYTOKEN>`) and `Vault`
//!   (use: `VaultOf<MYTOKEN>`).  Perfectly coexists with existing builtin types
//!   so you can gradually apply these only where you need them.
//! - Conveniently defined `BucketOf<XRD>` and `VaultOf<XRD>`
//! - Simple macro to declare new resources: `declare_resource!(MYTOKEN)`
//! - Optional feature `runtime_typechecks` for safety critical code, or use in
//!   testing
//!
//! # Quick Start
//!
//! ```rust
//! use scrypto::prelude::*;             // This doesn't replace Scrypto or the builtin types.
//! use scrypto_statictypes::prelude::*; // opt-in to use scrypto_statictypes!
//!
//! // Give a name to any resouces you want to statically type check.
//! declare_resource!(MYTOKEN);             // without a known address
//! // declare_resource!(XRD, RADIX_TOKEN); // or with a known address (but scrypto_statictypes already conveniently includes a declaration for XRD)
//! ```
//!
//! Now replace `Vault` with `VaultOf<XRD>` or `VaultOf<MYTOKEN>` and similarly replace `Bucket` with `BucketOf<XRD>` or `BucketOf<MYTOKEN>`.
//!
//! The rest of the API is unchanged, but with
//! these new types propogated to ensure correctness.
//!
//! You can start with just changing the `Vault`'s stored in the `Component` and let the compiler guide you the rest of the way.
//!
//!
//! # A Full Example
//!
//! This is a component that uses both `XRD` and `MYTOKEN` with compile-time static types to find bugs faster.
//!
//! There are commented out lines that will cause compile-time type-missmatch errors because Buckets and Vaults of XRD and MYTOKEN are incorrectly mixed.
//! Try it out!
//!
//! ```
//! # #[macro_use] extern crate scrypto_statictypes;
//! # fn main() {}
//! use scrypto::prelude::*;             // don't forget to use scrypto
//! use scrypto_statictypes::prelude::*; // and now use static types too!
//!
//!
//! // declare_resource!(XRD, RADIX_TOKEN); // not needed, scrypto_statictypes exports this already, but just as an example when the address is known
//! declare_resource!(MYTOKEN); // we can now use BucketOf<MYTOKEN> and VaultOf<MYTOKEN>
//!
//! blueprint! {
//!
//!    // use VaultOf with explicit types instead of Vault
//!    struct MyComponent {
//!         xrd_vault: VaultOf<XRD>,
//!         mytoken_vault: VaultOf<MYTOKEN>,
//!     }
//!
//!     impl MyComponent {
//!
//!         pub fn new() -> Component {
//!
//!             // When changing existing code, it's easier to leave out type annotations and you get the same compile-time checks.  Adding explicit
//!             // static types can make the code more readable, have error messages appear on the lines closest to the problem.
//!             // if runtime checks are used, it can abort the transaction earlier, or in the worst case detect an error the Radix Engine might miss.
//!
//!             // let my_bucket: BucketOf<MYTOKEN> = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM) // try this line instead of the next one to see how the compiler errors differ
//!             let my_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)
//!                 .metadata("name", "MyToken")
//!                 .metadata("symbol", "MYT")
//!                 .initial_supply_fungible(1000)
//!                 .into(); // the .into() changes the inferred type of my_bucket from Bucket to BucketOf.
//!
//!             let xrd_bucket = BucketOf::<XRD>::new(RADIX_TOKEN); // the explicit "turbofish" `::<XRD>` is really only needed when calling ::new().  When creating from other buckets such as `let xrd_bucket = another_bucket.take(1);` no annotations are needed.
//!
//!             // try uncommenting the next line
//!             // xrd_bucket.put(my_bucket); // This creates a relationship between the declared `XRD` resource of xrd_bucket and the type of my_bucket.  This leads to a type mismatch compile error on this line or later depending on how my_bucket was declared.
//!
//!             // If my_bucket is declared as a BucketOf<MYTOKEN> you will see an error like this:
//!             //
//!             //   error[E0308]: mismatched types
//!             //   --> src/lib.rs:85:28
//!             //    |
//!             // 33 |             xrd_bucket.put(my_bucket);
//!             //    |                            ^^^^^^^^^ expected enum `scrypto_statictypes::XRD`, found enum `MYTOKEN`
//!             //    |
//!             //    = note: expected struct `scrypto_statictypes::prelude::BucketOf<scrypto_statictypes::XRD>`
//!             //               found struct `scrypto_statictypes::prelude::BucketOf<MYTOKEN>`
//!
//!
//!             let xrd_vault = VaultOf::<XRD>::new(RADIX_TOKEN);
//!
//!             // OR try uncommenting the next line
//!             // xrd_vault.put(my_bucket); // Similarly, this creates a relationship between the declared `XRD` resource of xrd_vault and the type of my_bucket and also leads to a type mismatch
//!
//!             // Without explicitly declaring the type of my_bucket, the compiler will see these lines and infer the type
//!             // of my_bucket as an BucketOf<XRD>.  Then it will create a compile-time error on the final VaultOf::with_bucket(my_bucket) line
//!             // with an error like:
//!
//!             //   error[E0308]: mismatched types
//!             //     --> src/lib.rs:119:53
//!             //      |
//!             //   71 |                 mytoken_vault: VaultOf::with_bucket(my_bucket),
//!             //      |                                                     ^^^^^^^^^ expected enum `MYTOKEN`, found enum `scrypto_statictypes::XRD`
//!             //      |
//!             //      = note: expected struct `scrypto_statictypes::prelude::BucketOf<MYTOKEN>`
//!             //                 found struct `scrypto_statictypes::prelude::BucketOf<scrypto_statictypes::XRD>`
//!
//!             Self {
//!                 xrd_vault: xrd_vault,
//!                 mytoken_vault: VaultOf::with_bucket(my_bucket), // Use VaultOf instead of Vault, but the same with_bucket(...) API with no need to explicitly write <MYTOKEN>
//!             }
//!             .instantiate()
//!         }
//!
//!         // These special statically typed BucketOf and VaultOf can be used in formal parameters and return types too!
//!
//!         // old way (or for when the resource type really is allowed to be anything, just keep using Bucket)
//!         pub fn receive_any_tokens(&mut self, bucket: Bucket) { /* ... */ }
//!    
//!         // new way - and when optional feature `runtime_typechecks` is enabled, the resource is confirmed to match a "MYTOKEN" before the function body executes.
//!         pub fn receive_my_tokens(&mut self, bucket: BucketOf<MYTOKEN>) { /* ... */ }
//!
//!     }
//! }
//! ```
//!
pub mod prelude;

mod bucketof;
mod internal;
mod runtime;
mod vaultof;
mod resourceof;
mod bucketrefof;

#[macro_use]
mod  macros;

use scrypto::prelude::RADIX_TOKEN;

use crate::macros::*;

declare_resource!(XRD, RADIX_TOKEN);
