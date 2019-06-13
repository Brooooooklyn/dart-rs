# dart-rs
Dart rust binding library

## Play
```bash
export BINDGEN_DART_SDK_PATH=[DART_INSTALLED_PATH] // # /usr/local/Cellar/dart/2.3.1/libexec
cargo build
ln -sf target/debug/libdart_example.dylib ./libdart_example.dylib
dart test.dart
```
