# Tiny Rust Projects

<img src="doc-assets/ferris.png" alt="ferris" width="200"/>

A collection of tiny Rust applications and concepts implemented to showcase my familiarity with the language.

---

## List of applications

### CLI applications

* [echo-rs](/echo-rs/)
* Summary: Simple UNIX echo command clone implemented in Rust.
* Tools/Crates used:
  * [clap](https://crates.io/crates/clap)

* [Minigrep](/minigrep/)
  * Summary: Simple UNIX grep command clone implemented in Rust based on "The book" implementation.
  * Tools/Crates used:
    * None, just pure Rust!

* [JSON to CSV](/json_to_csv/)
  * Summary: Simple app that consults an API and stores the results in a CSV file.
  * Tools/Crates used:
    * [rust-csv](https://docs.rs/csv/latest/csv/)
    * [serde](https://serde.rs/)
    * [reqwest](https://docs.rs/reqwest/0.11.11/reqwest/)

* [coin-rs](/coin-rs/) (TODO)
  * Summary: A simple application to keep track of currency conversion rates to Brazilian Real.

### Web applications

Not yet!

### Concepts/PoCs

* [Rabbit-rs](/rabbit-rs/)
  * Summary: Simple RabbitMQ message consumer.
  * Tools/Crates used:
    * [amiquip](https://crates.io/crates/amiquip)
    * [serde](https://serde.rs/)
    * [reqwest](https://docs.rs/reqwest/0.11.11/reqwest/)
