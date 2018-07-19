# libecic
libecic (eci-conformance) is a library written in Rust for checking (and in the future, amending) the various properties of WebAssembly bytecode.
Initially designed with the properties of the ewasm ECI (Ethereum Contract Interface) in mind, libecic can be easily expanded to support checking other properties of ewasm bytecode.

libecic is also intended to be able to compile to WebAssembly and be called from a browser environment. This feature is however not ready yet, and is still being worked on.
