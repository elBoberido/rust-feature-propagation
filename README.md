## Test

Run with default features
```
cargo run --package app
```

Run with default features off
```
cargo run --package app --no-default-features
```

Run with default features off and activate some features
```
cargo run --package app --no-default-features --features=std
```

## Note

- the workspace `Cargo.toml` must must add `default-features = false` to the local crate dependencies, else `cargo run --package app` will not run with default features
```
[workspace.dependencies]
sys = { version = "0.8.15", path = "sys", default-features = false }
```

- in every crate, the feature must be propagated to each dependency and the default feature must also contain the local feature
```
[features]
default = [
    "sys/default",
    "base/default",
    "std",
]
std = [
    "sys/std",
    "base/std",
]

[dependencies]
sys = { workspace = true }
base = { workspace = true }
```

## TODO

- [ ] test with crate registry
- [ ] add another default feature to `base` and check if it is used with `cargo run --package app`
    - if it is not used, `default-features = false` needs also to be added to dependencies in each crate
