## WebP bindings for Rust

- `webp`: Rust-idiomatic wrappers for `libwebp` (not yet implemented)
- `libwebp`: safe wrappers for `libwebp-sys`
- `libwebp-sys`: hand-written raw bindings of the C libwebp library

### `webp`

The `webp` crate is not yet implemented.

### `libwebp`

The `libwebp` crate has three features:

- `0.5`: use APIs >= libwebp-0.5.0. Especially, it uses the `WebPFree` instead
  of `free` if the feature is specified.
- `0.6`: use APIs >= libwebp-0.6.0. Assumes `0.5`.
- `bundled`: compiles the bundled libwebp C library. Requires the `cc` crate. Automatically assumes `0.6`.

The interface is similar to the raw interface, but with these modifications:

- Multiple values are returned as tuples, not through return-pointers. Return-pointer parameters are eliminated.
- Pairs of pointers and lengths are combined into slice parameters.
- Cleanup functions like `WebPFree` and `WebPIDelete` aren't wrapped. They're called in `Drop` implementation instead.
- Some functions return `Result` to avoid invalid values in case of failure.
- C-specific types like `c_int` are avoided.

Example:

```rust
#[macro_use]
extern crate approx;
extern crate libwebp;
use libwebp::*;

fn main() {
    let data = b"\
        RIFFV\x00\x00\x00WEBPVP8\x20\
        J\x00\x00\x00\xD0\x01\x00\x9D\x01*\x03\x00\x02\x00\x02\x00\
        4%\xA8\x02t\x01\x0E\xFE\x03\x8E\x00\x00\xFE\xAD\xFF\xF1\
        \x5C\xB4\xF8\xED\xFF\xF0\xC0\xBA\xBF\x93\x05\xEA\x0C\x9F\x93?\
        \xE8\xC0\xBF?\xFF\xA9\xBF\xFF${\xCB\xFFF\x05\xF9\xFF
        \xFDM\xFE0\xE5\x86\xAA\x071#o\x00\x00\x00";
    let (width, height, data) = WebPDecodeRGBA(data).unwrap();
    assert_eq!((width, height), (3, 2));
    assert_abs_diff_eq!(data[0], 67, epsilon = 1);
# }
```

### `libwebp-sys`

The `libwebp-sys` crate has three features:

- `0.5`: use APIs >= libwebp-0.5.0. Especially, it uses the `WebPFree` instead
  of `free` if the feature is specified.
- `0.6`: use APIs >= libwebp-0.6.0. Assumes `0.5`.
- `bundled`: compiles the bundled libwebp C library. Requires the `cc` crate. Automatically assumes `0.6`.

It's a direct translation of the following libwebp public interfaces:

- `webp/decode.h`
- `webp/encode.h`
- `webp/types.h`
- `webp/demux.h`
- `webp/mux.h`
- `webp/mux_types.h`

All definitions are placed directly under the crate.
