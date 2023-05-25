# C bindings and Swift wrapper

The C bindings are exported as a module in the form of a `XCFramework` which is visible to Swift.

To cross-compile for iOS, simulator and macOS platforms (ignoring Catalyst for now):

```
# iOS
rustup target add aarch64-apple-ios

# Simulator
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios

# macOS
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
```

[More info TBD]

