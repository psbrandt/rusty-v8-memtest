# Deno Runtime Debugging

## Environment

```
$ uname -a
Linux c66f2e42d6d7 6.1.51-0-virt #1-Alpine SMP Mon, 04 Sep 2023 08:04:05 +0000 aarch64 GNU/Linux
$ cargo version
cargo 1.72.1 (103a7ff2e 2023-08-15)
$ valgrind --version
valgrind-3.19.0
```

## Results

### `master`

This is using the code from the `master` branch, which contains only the
following:

```rust
fn main() {
    deno_core::JsRuntime::new(deno_core::RuntimeOptions::default());
}
```

I ran DHAT as follows:

```
$ valgrind --tool=dhat --dhat-out-file=dhat-master.json ./target/debug/rusty-v8-memtest
==20430== DHAT, a dynamic heap analysis tool
==20430== Copyright (C) 2010-2018, and GNU GPL'd, by Mozilla Foundation
==20430== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==20430== Command: ./target/debug/rusty-v8-memtest
==20430==
==20430==
==20430== Total:     2,766,579 bytes in 3,128 blocks
==20430== At t-gmax: 664,075 bytes in 1,134 blocks
==20430== At t-end:  36,701 bytes in 659 blocks
==20430== Reads:     11,243,734 bytes
==20430== Writes:    3,642,207 bytes
==20430==
==20430== To view the resulting profile, open
==20430==   file:///usr/libexec/valgrind/dh_view.html
==20430== in a web browser, click on "Load...", and then select the file
==20430==   /opt/rusty-v8-memtest/dhat-master.json
==20430== The text at the bottom explains the abbreviations used in the output.
```

See the DHAT viewer results as PDF [here](./dhat-output/master/dhat-master.pdf).

### `jemalloc`

The code in the `jemalloc` branch is as follows:

```rust
#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    deno_core::JsRuntime::new(deno_core::RuntimeOptions::default());
}
```

The DHAT output is:

```
$ valgrind --tool=dhat --dhat-out-file=dhat-jemalloc.json ./target/debug/rusty-v8-memtest
==24968== DHAT, a dynamic heap analysis tool
==24968== Copyright (C) 2010-2018, and GNU GPL'd, by Mozilla Foundation
==24968== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==24968== Command: ./target/debug/rusty-v8-memtest
==24968==
==24968==
==24968== Total:     2,708,886 bytes in 2,890 blocks
==24968== At t-gmax: 639,278 bytes in 1,034 blocks
==24968== At t-end:  33,909 bytes in 657 blocks
==24968== Reads:     11,190,119 bytes
==24968== Writes:    3,589,676 bytes
==24968==
==24968== To view the resulting profile, open
==24968==   file:///usr/libexec/valgrind/dh_view.html
==24968== in a web browser, click on "Load...", and then select the file
==24968==   /opt/rusty-v8-memtest/dhat-jemalloc.json
==24968== The text at the bottom explains the abbreviations used in the output.
```

See the DHAT viewer results [here](./dhat-output/master/dhat-master.pdf).

### `master` without WASM

Since it looked like some memory was being leaked by the WASM engine, I tried
with a custom build of V8 without WASM:

```sh
$ GN_ARGS="v8_enable_webassembly=false" V8_FROM_SOURCE=1 cargo build -vv
```

This gives:

```
$ valgrind --tool=dhat --dhat-out-file=dhat-no-wasm.json ./target/debug/rusty-v8-memtest
==29611== DHAT, a dynamic heap analysis tool
==29611== Copyright (C) 2010-2018, and GNU GPL'd, by Mozilla Foundation
==29611== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==29611== Command: ./target/debug/rusty-v8-memtest
==29611==
Error: unrecognized flag --wasm-test-streaming
The remaining arguments were ignored: --harmony-import-assertions --harmony-import-attributes --no-validate-asm --turbo_fast_api_calls --harmony-change-array-by-copy
Try --help for options
==29611==
==29611== Total:     3,628,306 bytes in 9,378 blocks
==29611== At t-gmax: 853,363 bytes in 1,294 blocks
==29611== At t-end:  18,789 bytes in 553 blocks
==29611== Reads:     34,087,294 bytes
==29611== Writes:    24,618,078 bytes
==29611==
==29611== To view the resulting profile, open
==29611==   file:///usr/libexec/valgrind/dh_view.html
==29611== in a web browser, click on "Load...", and then select the file
==29611==   /opt/rusty-v8-memtest/dhat-no-wasm.json
==29611== The text at the bottom explains the abbreviations used in the output.
```

DHAT viewer result [here](./dhat-output/no-wasm/dhat-no-wasm.pdf).

### `jemalloc` without WASM

Using the custom V8 build above, I tried the same thing with the `jemalloc`
allocator:

```
$ valgrind --tool=dhat --dhat-out-file=dhat-jemalloc-no-wasm.json ./target/debug/rusty-v8-memtest
==29649== DHAT, a dynamic heap analysis tool
==29649== Copyright (C) 2010-2018, and GNU GPL'd, by Mozilla Foundation
==29649== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==29649== Command: ./target/debug/rusty-v8-memtest
==29649==
Error: unrecognized flag --wasm-test-streaming
The remaining arguments were ignored: --harmony-import-assertions --harmony-import-attributes --no-validate-asm --turbo_fast_api_calls --harmony-change-array-by-copy
Try --help for options
==29649==
==29649== Total:     3,570,441 bytes in 9,129 blocks
==29649== At t-gmax: 825,414 bytes in 1,118 blocks
==29649== At t-end:  15,725 bytes in 549 blocks
==29649== Reads:     33,955,905 bytes
==29649== Writes:    24,576,913 bytes
==29649==
==29649== To view the resulting profile, open
==29649==   file:///usr/libexec/valgrind/dh_view.html
==29649== in a web browser, click on "Load...", and then select the file
==29649==   /opt/rusty-v8-memtest/dhat-jemalloc-no-wasm.json
==29649== The text at the bottom explains the abbreviations used in the output.
```

[DHAT viewer output](./dhat-output/jemalloc-no-wasm/dhat-jemalloc-no-wasm.pdf)


### With Dispose

Running `dispose` and `dispose_platform` manually seems to help. The following code:


```rust
fn main() {

    deno_core::JsRuntime::new(deno_core::RuntimeOptions::default());

    unsafe {
        v8::V8::dispose(); 
    }

    v8::V8::dispose_platform();
}
```

Gives:

```
$ valgrind --tool=dhat --dhat-out-file=dhat-with-dispose.json ./target/debug/rusty-v8-memtest
==3196== DHAT, a dynamic heap analysis tool
==3196== Copyright (C) 2010-2018, and GNU GPL'd, by Mozilla Foundation
==3196== Using Valgrind-3.19.0 and LibVEX; rerun with -h for copyright info
==3196== Command: ./target/debug/rusty-v8-memtest
==3196==
==3196==
==3196== Total:     2,766,643 bytes in 3,129 blocks
==3196== At t-gmax: 664,107 bytes in 1,134 blocks
==3196== At t-end:  6,970 bytes in 47 blocks
==3196== Reads:     11,242,267 bytes
==3196== Writes:    3,645,139 bytes
==3196==
==3196== To view the resulting profile, open
==3196==   file:///usr/libexec/valgrind/dh_view.html
==3196== in a web browser, click on "Load...", and then select the file
==3196==   /opt/rusty-v8-memtest/dhat-with-dispose.json
==3196== The text at the bottom explains the abbreviations used in the output.
```

[DHAT viewer](./dhat-output/with-dispose/dhat-with-dispose.pdf)



