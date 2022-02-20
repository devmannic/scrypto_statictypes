#[cfg(feature = "runtime_typechecks")]
pub(crate) mod runtimechecks {
    use scrypto::prelude::{debug, error};

    use crate::internal::*;

    type AddressKey = std::any::TypeId;

    pub trait Resource: crate::internal::Resource {
        fn index() -> AddressKey;
        fn address() -> Option<Address>;
    }
    impl<T: ResourceDecl + 'static> Resource for T {
        #[inline(always)]
        fn index() -> AddressKey {
            std::any::TypeId::of::<T>()
        }

        #[inline(always)]
        fn address() -> Option<Address> {
            T::ADDRESS
        }
    }

    // Adapted from https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
    // replace this with only_once when it's in rustc standard

    use std::mem::MaybeUninit;
    use std::sync::{Mutex, Once};

    #[derive(Default)]
    struct KnownAddresses {
        addresses: std::collections::HashMap<AddressKey, Address>,
        all_addresses: std::collections::HashSet<Address>,
    }

    struct SingletonReader {
        // Since we will be used in many threads, we need to protect
        // concurrent access
        inner: Mutex<KnownAddresses>,
    }

    fn singleton() -> &'static SingletonReader {
        // Create an uninitialized static
        static mut SINGLETON: MaybeUninit<SingletonReader> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        unsafe {
            ONCE.call_once(|| {
                // Make it
                let singleton = SingletonReader {
                    inner: Mutex::new(KnownAddresses::default()),
                };
                // Store it to the static var, i.e. initialize it
                SINGLETON.write(singleton);
            });

            // Now we give out a shared reference to the data, which is safe to use
            // concurrently.
            SINGLETON.assume_init_ref()
        }
    }
    // end of only_once alternative

    pub fn check_address<RES: Resource>(address: Address) -> bool {
        match RES::address() {
            Some(expected) => {
                let r = expected == address;
                if r {
                    debug!(
                        "check_addr static matched: {}: {}",
                        std::any::type_name::<RES>(),
                        address
                    );
                } else {
                    error!(
                        "check_addr static mismatch {}: {} != {}",
                        std::any::type_name::<RES>(),
                        address,
                        expected
                    );
                }
                r
            }
            None => {
                let mut guard = singleton().inner.lock().unwrap();
                let KnownAddresses {
                    ref mut addresses,
                    ref mut all_addresses,
                } = &mut *guard;
                let i = RES::index();
                match addresses.entry(i) {
                    std::collections::hash_map::Entry::Occupied(o) => {
                        let expected = *o.get();
                        let r = expected == address;
                        if r {
                            debug!(
                                "check_addr dynamic matched: {}: {}",
                                std::any::type_name::<RES>(),
                                address
                            );
                        } else {
                            error!(
                                "check_addr dynamic mismatch {}: {} != {}",
                                std::any::type_name::<RES>(),
                                address,
                                expected
                            );
                        }
                        r
                    }
                    std::collections::hash_map::Entry::Vacant(v) => {
                        // enforce that all Resource declarations are unique, helps detect some errors
                        // ensure that address is not already in the resource map under another name
                        // use the set of values to check
                        if all_addresses.contains(&address) {
                            error!(
                                "check_addr dynamic cannot create address in use: {}: {}",
                                std::any::type_name::<RES>(),
                                address
                            );
                            false
                        } else {
                            debug!(
                                "check_addr dynamic created: {}: {}",
                                std::any::type_name::<RES>(),
                                address
                            );
                            all_addresses.insert(address);
                            v.insert(address);
                            true
                        }
                    }
                }
            }
        }
    }
}
