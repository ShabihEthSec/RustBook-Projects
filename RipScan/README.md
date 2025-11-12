
## About RipScan

**As a** developer learning Rust,
**I want** a tiny CLI that searches for a query in a file (with an optional case-insensitive mode),
**so that** I can test CLI behavior, unit tests, and environment-driven flags while practicing Rust I/O and lifetimes.

**Acceptance criteria**

* Running the binary with `cargo run -- <query> <file>` prints every matching line (case-sensitive by default).
* Setting the environment variable `IGNORE_CASE` (e.g. `IGNORE_CASE=1`) makes the search case-insensitive.
* Unit tests in `src/lib.rs` (`test_search` and `test_search_case_insensitive`) pass with `cargo test`.

---

## How to clone the project

Replace `<repo-url>` with your repository URL (HTTPS or SSH):

```bash
# HTTPS
git clone https://github.com/your-username/rip_scan.git

# or SSH
git clone git@github.com:your-username/rip_scan.git

cd rip_scan
```

> If your local folder name is different, `cd` into the project root where `Cargo.toml` lives.

---

## Build & run (basic)

```bash
# build
cargo build

# run (case-sensitive)
cargo run -- Rust poem.txt
```

If your program compiles and the file exists, it will print matching lines from `poem.txt`.

---

## Example poem test file

Create a `poem.txt` in the project root (or point to any text file):

```
Rust is fast and fearless.
rust builds the future.
Code in silence, think in loops.
The night is quiet, the bugs sleep.
Trust the process, trust the code.
Learn Rust, earn trust.
Dream in logic, wake in light.
Fearless minds build Rust again.
```

### Example runs

Case-sensitive (only lines with `Rust` exactly):

```bash
cargo run -- Rust poem.txt
# Expected output:
# Rust is fast and fearless.
# Fearless minds build Rust again.
```

Case-insensitive (set env var before running):

```bash
# Unix / macOS / WSL
IGNORE_CASE=1 cargo run -- rust poem.txt

# PowerShell
$env:IGNORE_CASE = "1"; cargo run -- rust poem.txt

# cmd.exe
set IGNORE_CASE=1
cargo run -- rust poem.txt
```

This should print lines containing `rust` in any case (both `Rust` and `rust`).

---

## Run unit tests

From the project root:

```bash
cargo test
```

Expected output for the two tests in `lib.rs` (approx):

```
running 2 tests
test tests::test_search ... ok
test tests::test_search_case_insensitive ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

If a test fails, `cargo test` will show the failing test and a diff between expected and actual values thanks to `assert_eq!`.

---

## Troubleshooting tips

* **Crate name warning**: If you see a warning like `crate 'RipScan' should have a snake case name`, change `name = "RipScan"` to `name = "rip_scan"` in `Cargo.toml`.
* **If `IGNORE_CASE=0` still triggers insensitive search**: your code currently treats *any* existence of `IGNORE_CASE` as `true`. To treat `"0"` or `"false"` as false, change the check to parse the value (e.g. `env::var("IGNORE_CASE").map(|v| v != "0" && v.to_lowercase() != "false").unwrap_or(false)`).
* **If `cargo run` prints nothing**: ensure the file path is correct and `Config::build` parsed arguments correctly.
* **If tests call themselves**: ensure test function names don’t collide with library function names (e.g., name test functions `test_search_case_insensitive` rather than `search_case_insensitive`).
* **To see stdout from tests** use:

  ```bash
  cargo test -- --nocapture
  ```

