# Rust Bootcamp — Account Trait Example

A small Rust exercise demonstrating traits, structs, mutable methods, and branching with an in-memory bank-account example.

## What it does

`src/main.rs` defines an `Account` trait and implements it for `BankAccount`. The example creates two accounts, deposits 150 into an initial balance of 500, withdraws 100 from an initial balance of 300, and prints final balances of 650 and 200.

## Run

Install Rust with `rustc` available on your path. This repository does not contain `Cargo.toml`, so compile the single file directly:

```sh
git clone https://github.com/Murat-Selim/Rust_Bootcamp.git
cd Rust_Bootcamp
rustc src/main.rs -o account-demo
```

Run `./account-demo` on macOS/Linux or `.\account-demo.exe` on Windows.

## Scope and validation

This is an educational example: balances use floating-point numbers, data is not persisted, and it is not a financial application. There is no dedicated automated test suite. Run the executable and check the printed final balances.

## License

[MIT](LICENSE).
