# Generative Art

## Working

Simply run using command:

```bash
cargo run -r
```

You can also build and then run to save some time from building on the first run

```bash
cargo build -r
./target/release/generation

```

**Note**: If you are running on Wayland you would face some errors which can be resolved by doing:
```
export WAYLAND_DISPLAY=wayland-1
```

## Variations

Each commit is a different example all together. Shift through commits using

```bash
git checkout HEAD~2/3/...
```

Return back to `main` using


```bash
git checkout main
```
