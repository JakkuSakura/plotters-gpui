# plotters-gpui

[GPUI](https://gpui.rs) backend for [plotters](https://github.com/plotters-rs/plotters).

Please goto [https://github.com/JakkuSakura/plotters-gpui](https://github.com/JakkuSakura/plotters-gpui)
for more information.

Due to gpui not being published on crates.io, you need to add the following to your `Cargo.toml`:

```toml
[dependencies]
plotters-gpui = { git = "https://github.com/JakkuSakura/plotters-gpui" }
```

If you failed to build on linux due to font-kit, you might need to add the following to your `Cargo.toml`:

```toml
[dependencies]
font-kit = { git = "https://github.com/JakkuSakura/font-kit-patched", features = ["source-fontconfig-dlopen"] }

# this is to sync versions of font-kit
[patch."https://github.com/zed-instustries/font-kit"]
font-kit = { git = "https://github.com/JakkuSakura/font-kit-patched" }

# because plotters' font-kit might fail
[patch.crates-io]
font-kit = { git = "https://github.com/JakkuSakura/font-kit-patched" }

```

You might be interested in [https://github.com/JakkuSakura/gpui-plot](https://github.com/JakkuSakura/gpui-plot), as it
provides interactivity and more stuff on top of plotters-gpui

## Show cases

<img width="300" src="https://github.com/user-attachments/assets/58104fbd-35e7-40a1-be8d-ad18945acacb" />
<img width="300" src="https://github.com/user-attachments/assets/86c95b28-74db-44d3-8599-910d24adee55" />
<img width="300" src="https://github.com/user-attachments/assets/f599b6a8-946d-492a-a423-c2805fb22c4c" />
<img width="300" src="https://github.com/user-attachments/assets/066f3f92-9671-48cf-8383-9a55d1bf0ef7" />
<img width="300" src="https://github.com/user-attachments/assets/e837b24e-50c6-4ddb-ad06-d5084920b424" />
<img width="300" src="https://github.com/user-attachments/assets/fd61a1cb-ef51-4654-9a50-4154694de57e" />
<img width="300" src="https://github.com/user-attachments/assets/4cd02da9-bf8f-4e5e-9b4b-2423787ded0d" />

## Performance

plotters-gpui try to maintain very good performance compared to that in javascript/python, even better than CPU based
plotting libraries

### Metal Performance HUD

Use the following environment variable to enable Metal Performance HUD:

```text
MTL_HUD_ENABLED=1
Enables the Metal Performance HUD.
```

```shell
MTL_HUD_ENABLED=1 cargo run ...
```
