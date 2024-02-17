# ej (edge)

edge lite package

## Dev Workflow

```bash
cargo bump patch --git-tag
git push --follow-tags
```

```bash
# cargo release
cargo publish

# build a release file
cargo build --release
```

## Todo

- mqtt service
- bluetooth (ble) service
- db persistence?