# Deno Runtime Debugging

### Environment

```
$ uname -a
Linux c66f2e42d6d7 6.1.51-0-virt #1-Alpine SMP Mon, 04 Sep 2023 08:04:05 +0000 aarch64 GNU/Linux
$ cargo version
cargo 1.72.1 (103a7ff2e 2023-08-15)
# valgrind --version
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

```
$  valgrind --tool=dhat --dhat-out-file=dhat-master.json ./target/debug/rusty-v8-memtest
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