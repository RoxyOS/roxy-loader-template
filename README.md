# roxy-loader-template

Minimal kernel template using `roxy-loader` as the bootloader.

When the kernel boots successfully, it draws a white line on the screen.

## Requirements

Required tools:

- Rust nightly
- `qemu-system-x86_64`

If you use Nix, enter the dev shell with:

```bash
nix develop
```

## Usage

Run the kernel in QEMU:

```bash
cargo xrun
```
