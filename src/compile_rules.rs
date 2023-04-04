#[cfg(not(any(feature = "native-tls", feature = "rustls")))]
compile_error!("one of the features ['native-tls', 'rustls'] must be enabled");

#[cfg(all(feature = "native-tls", feature = "rustls"))]
compile_error!("'native-tls' and 'rustls' cannot be enabled at the same time");
