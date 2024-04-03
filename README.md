# A home for all Spiel Rust bindings

## Speech Provider Rust Bindings

These bindings are auto-generated with the following commands:

```sh
# In libspeechprovider
meson setup build -Drust-bindings=true
meson compile -C build
cp -a build/speechprovider-rs ../rust-bindings
```

We should have some CI job do that in the future..