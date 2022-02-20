# Scrypto Static Types

[![Test Status](https://github.com/devmannic/scrypto_statictypes/workflows/Tests/badge.svg?event=push)](https://github.com/devmannic/scrypto_statictypes/actions)
![GitHub release (latest by date)](https://img.shields.io/github/v/release/devmannic/scrypto_statictypes?display_name=tag)
[![Crate](https://img.shields.io/badge/crates.io-on%20hold-orange)](https://crates.io/crates/scrypto_statictypes)
[![API](https://img.shields.io/badge/api-master-green.svg)](https://devmannic.github.io/scrypto_statictypes)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.56+-darkgreen.svg)](https://github.com/devmannic/scrypto_statictypes#rust-version-requirements)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/devmannic/scrypto_statictypes#license)

Use explicit container types with Scrypto!  Leverage the Rust compiler's type checking to increase security and productivity when developing Radix blueprints.

A Scrypto (Rust) library for static types, featuring:

- A simple, usable, safe, and (by default) zero-cost API for compile-time
    static type checking of resources.
- Safe drop in replacements coexist with existing types.  Gradually apply these only where you need them:
  - `Bucket` --> `BucketOf<MYTOKEN>`
  - `Vault` --> `VaultOf<MYTOKEN>`
  - `ResourceDef` --> `ResourceOf<MYTOKEN>`
  - `BucketRef` --> `BucketRefOf<MYTOKEN>`
- Conveniently defined `XRD` Resource to use with `VaultOf<XRD>`, and friends.
- Simple macro to declare new resources: `declare_resource!(MYTOKEN)`
- Optional feature `runtime_typechecks` for safety critical code, or use in
  testing.

It's also worth pointing out what `scrypto_statictypes` *is not*:

- A silver bullet.  This library can ensure that within a single component
  there are no resource mismatches, and it can do this all at compile time.
  But, There is no completely static way to ensure proper usage of resources
  received by a component.  Any (incorrect) ABI may be used when calling a
  function or method on a component.  Transactions are inherently dynamic
  with arguments referencing `ResourceDef`'s which are simply wrappers around
  an `Address`.  Runtime checks are still needed, and luckily are performed
  by the Radix Engine.  However, this library can still help avoid
  implementation errors which would go undetected by the Radix Engine (ie.
  burning the wrong type of resource).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
scrypto_statictypes = { git = "https://github.com/devmannic/scrypto_statictypes" }
```

Optionally, add this to your `Cargo.toml` for extra checks at runtime:

```toml
[features]
default = ["scrypto_statictypes/runtime_typechecks"]
```

Setup static types in your code near the top:

```rust
use scrypto::prelude::*;             // This doesn't replace Scrypto's builtin types.  You will still need them sometimes.
use scrypto_statictypes::prelude::*; // opt-in to use scrypto_statictypes!

// Give a name to any resource you want to statically type check.
declare_resource!(MYTOKEN);             // without a known address
// declare_resource!(XRD, RADIX_TOKEN); // or with a known address (but this line isn't needed, scrypto_statictypes already conveniently includes it)
```

Simple example for a blueprint which creates a token with a declared `MYTOKEN` resource:

```rust
use scrypto::prelude::*;             // This doesn't replace Scrypto's builtin types.  You will still need them sometimes.
use scrypto_statictypes::prelude::*; // opt-in to use scrypto_statictypes

// the new way
declare_resource!(MYTOKEN); // declare a name for a resource that this blueprint creates, or when the `Address` is unknown or not needed.

blueprint! {
    struct MyComponent {
        // my_vault: Vault // the old way
        my_vault: VaultOf<MYTOKEN> // the new way
    }
    pub fn new() -> Component {
        let my_bucket = ResourceBuilder::new_fungible(DIVISIBILITY_MAXIMUM)  // notice we didn't have to explicitly write out the type ie. Bucket<MYTOKEN>
            .metadata("name", "MyToken")
            .metadata("symbol", "MYTOKEN")
            .initial_supply_fungible(1000)
            .into(); // the new way: .into() needed to convert Bucket -> Bucket<MYTOKEN>
        Self {
            // my_vault: Vault::with_bucket(my_bucket) // the old way
            my_vault: VaultOf::with_bucket(my_bucket) // the new way: use VaultOf instead of Vault
        }
    }
    // or even fewer changes when only setting up a vault.
    pub fn new_easier() -> Component {
        let my_bucket = ResourceBuilder::new()  // this is a regular Bucket, but we're only using it to fill the vault
            .metadata("name", "MyToken")
            .metadata("symbol", "MYTOKEN")
            .new_token_fixed(1000);
        Self {
            // my_vault: Vault::with_bucket(my_bucket) // the old way
            my_vault: Vault::with_bucket(my_bucket).into() // the new way: .into() needed to convert Vault -> VaultOf<MYTOKEN>
        }
    }

    // old way (or for when the resource type really is allowed to be anything, just keep using Bucket)
    pub fn receive_any_tokens(&mut self, bucket: Bucket) { /* ... */ }

    // new way - and when optional feature `runtime_typechecks` is enabled, the resource is confirmed to match a "MYTOKEN" before the function body executes.
    pub fn receive_my_tokens(&mut self, bucket: BucketOf<MYTOKEN>) { /* ... */ }
}
```

For any buckets where the asset is of a known type (most of them) replace:

`Bucket` -> `BucketOf<MYTOKEN>`

Similarly with vaults, replace:

`Vault` -> `VaultOf<MYTOKEN>`

This provides a way to explicitly name any resource.  It can be used for anything that can go
in a `Bucket` or `Vault`.  This includes badges and NFTs.

That's just the beginning.... You can also replace:

`ResourceDef` -> `ResourceOf<MYTOKEN>`

`BucketRef` -> `BucketRefOf<MYTOKEN>`

And with runtime checks enabled, you get a more
convenient and safe API for checks that used to be done with `#[auth(some_resource_def)]`.  You can
use one or more `BucketRefOf` arguments to limit access without manual checks against
`some_resource_def`.  Simply change the type of `some_resource_def` to `ResourceOf<SOMETHING>` in the Component struct.  Or if you already have a `VaultOf<SOMETHING>` no new `ResourceOf` is needed.

To let the compiler help as much as possible the `BucketRefOf` type will automatically
drop the reference to its bucket when it goes out of scope.  This works correctly when returning `BucketRefOf`s or calling other functions/methods. Never worry about explicitly calling `bucketref.drop()`.

These new types are usable everywhere: replace function/method argument and return types, local variables, fields in structs (including the main component storage struct).  Everything just works.

For converting existing blueprints just add typing gradually and follow the compiler warnings as a way to audit the code and reach a more secure implementation.  At the "boundaries" where these static types need to be converted, simply use `.into()` for type checked conversions into any of these new types, and `.unwrap()` to convert back to the old, dynamic (standard Scrypto) types.

## Documentation:

More details can be found in the API documentation including a more complex example.

- [API reference (master branch)](https://devmannic.github.io/scrypto_statictypes)


## Crate Features

Optionally, the following features can be enabled:

- `runtime_checks` enables checking for type mismatches at runtime when
  converting between builtin dynamic types (ie. `Bucket` or `Vault`) and
  static types.
  - This may catch errors earlier than the builtin checks that happen on ie. `Bucket::put` and similar, and
    in some cases could detect bugs that would go undetected, such as calling `Bucket::burn` on the wrong bucket.
  - This relies on either an address provided when declaring the resource with `declare_resource!(NAME, ADDRESS)`
    or if no address is provided the implementation remembers the first address seen and checks all future
    accesses match.  This works nicely when a component contains `VaultOf<NAME>` because the Radix Engine will
    instantiate the component struct very early and decode a `Vid` into the `VaultOf<NAME>` which correctly binds
    the address.
  - Errors caught are trapped to the Radix Engine runtime failing the transaction immediately in 
    exactly the same way as when using `Bucket` or `Vault`, even with the exact same error as with a  "bad" `Bucket::put` or `Vault::put`.  Respectively `Err(InvokeError(Trap(Trap { kind: Host(BucketError(MismatchingResourceDef)) })))` and `Err(InvokeError(Trap(Trap { kind: Host(VaultError(AccountingError(MismatchingResourceDef))) })))`


## Examples

See the directories in [/examples](/examples) for complete scrypto packages utilizing this functionality.

* [/examples/mycomponent](/examples/mycomponent) - Same example as in the [API reference (master branch)](https://devmannic.github.io/scrypto_statictypes) so you can easily try and see the compiler errors
* [/examples/badburn1](/examples/badburn1) - Example blueprint which does *NOT* use `scrypto_statictypes` and has a logic error which leads to burning the bucket argument even if it was the wrong asset
* [/examples/fixburn1](/examples/fixburn1) - Direct modification of `BadBurn` to use static types everywhere, and enable runtime type checks.  The test case shows the "bad burn" is caught and the tx fails. -- checkout just the diff of changes in [/misc/bad2fixburn1.diff](/misc/bad2fixburn1.diff)
* [/examples/manyrefs](/examples/manyrefs) - Example using BucketRefOf a whole lot showing it's usefulness for nuanced authentication/verification

## Versions

Scrypto Static Types is suitable for general usage, but not yet at 1.0. We maintain compatibility with Scrypto and pinned versions of the Rust compiler (see below).

Latest Scrypto Static Types versions for Scrypto versions are:

- Version 0.4.1 depending on Scrypto version 0.3.0
- Version 0.3.1 depending on Alexandria Scrypto version 0.2.0
- Version 0.1.1 depending on Pre-Alexandria Scrypto version 0.1.1

The intent is to:

- Keep the `main` branch in sync with the `radixdlt-scrypto` `main` branch
- Keep the `develop` branch in sync with the `radixdlt-scrypto` `develop` branch
    * As a best effort, mostly to support keeping up with `main` as quickly as possible

A detailed [changelog](CHANGELOG.md) is available for releases

Scrypto Static Types has not yet reached 1.0 implying some breaking changes may arrive in the
future ([SemVer](https://semver.org/) allows each 0.x.0 release to include
breaking changes), but breaking changes are minimized and breaking releases are infrequent.

When specifying the dependency requirement using `scrypto_statictypes = { git = "https://github.com/devmannic/scrypto_statictypes" }`
the latest release will always be used, ignoring SemVer.  When Scrypto is
published to crates.io, this library will also be published and it is advised to
update your `Cargo.toml` then to `scrypto_statictypes = "^0"` or later.

### Yanked versions

Some versions of Scrypto Static Types crates may be yanked ("unreleased") from crates.io. Where this occurs,
the crate's CHANGELOG *should* be updated with a rationale, and a search on the
issue tracker with the keyword `yank` *should* uncover the motivation.

### Rust version requirements

Since version 0.1, Scrypto Static Types requires **Rustc version 1.56 (2021 edition) or greater**.

Continuous Integration (CI) will always test the minimum supported Rustc version
(the MSRV). The current policy is that this can be updated in any
Scrypto Static Types release if required, but the change must be noted in the changelog.

## Why and How?

I believe Radix will be a game-changing technology stack for Decentralized
Finance.  Scrypto is already amazing and going to continue to evolve.  I think
at this very early stage it does so many things right, however it's a missed
opportunity to treat all `Bucket`s and `Vault`s as dynamic types that could
hold anything, when in fact they are bound by their `ResourceDef` upon creation.
There is also a lot of duplicate code checking `BucketRef`s as the main form of
authentication when it could be done declaratively in more places than just the existing
auth macros.  These omissions can lead
to entire classes of logic errors which could be avoided.  This means
lost productivity at best, and real vulnerabilities at worst.  I didn't want
development practices to standardize leaving these gaps.

So, I took up the challenge to find a *usable* way to recreate strong static
types with no (or at least minimal) runtime cost.  This meant not introducing a new API for dealing
with Vaults and Buckets.  This is possible with minimal re-implementation which
is inlined and effectively disappears since they are just proxies around the
original types.  The main changes are to enable type propagation in parameters
and return types, and then implementing Rust's `Deref` and `DerefMut` traits
gets us the rest of the way.  It makes the usage seamless.  And since
it's implemented with Rust's generics and `PhantomData` there is no extra
storage of the type information.

Then, going a step further I added runtime tests of the underlying `Address`es.
This is behind a feature flag so is opt-in as it does add some extra
performance overhead.  But, there are cases where it can absolutely detect a
logic error due to misuse of a Bucket or Vault, when the current Radix Engine
constraints would not stop the error.  This is because the Radix Engine just doesn't have
the information about the developer's intent.  By using
specific type annotations this intent can be captured and then appropriately tested
leading to failed transactions instead of vulnerabilities.

All of this is completely optional, and it can be used to *gradually* add types
to programs where it is helpful.

My hope is that others find this valuable and make good use of it.  I would actually
love to see this merged upstream into Scrypto.  But if that never happens
at least we have what is hopefully a high quality library.

## Tips

You can support the original author (`devmannic`) with tips by sending XRD or other
tokens on the Radix protocol mainnet to:

`rdx1qsppkruj82y8zsceswugc8hmm6m6x22vjgwq8tqj8jnt2vcjtmafw8geyjaj9`


# License

Scrypto Static Types is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
