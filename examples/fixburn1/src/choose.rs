#[cfg(feature = "runtime_typechecks")]
mod lib;

#[cfg(not(feature = "runtime_typechecks"))]
mod staticlib;