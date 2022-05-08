# Testing in Rust

## Running Tests

`cargo test` creates a test binary and then run the tests using that binary.

You can pass flags to `cargo test` itself, or to the resulting test binary. The flags can be separate with `--`
i.e flags before `--` are passed directly to `cargo test` while after are passed to the binary.
- To see flags you can pass to `cargo test` run `cargo test --help`.
- To see flags you can pass to the binary run `cargo test -- --help`

By default, Rust tests are run in parallel. This can be a problem if tests share state. To disable this,
use `cargo test -- --test-threads=1`. We are passing the `test-thread` flag to the binary here to let
it know it should only use 1 thread to run tests.

By default, Rust tests capture all outputs i.e `println!` statements will not be printed if the test passes,
only if the test fails. To override this, run tests with `cargo test -- --show-output`.

To run only on test use `cargo test test_name` i.e `cargo test it_works`
- Actually, `test_name` could be a substring of the test name. Rust will run all tests that match this substring.

If there's a time-consuming test you don't want to always run with `cargo test`, you can mark them as `#[ignore]`.
- If you want to run just the ignored tests, do `cargo test -- --ignored`.
- If you want all tests together including ignored ones, run `cargo test -- --include-ignored`.

## Test Organisation

### Unit tests
- By convention, unit tests are put in `src` directory in each file together with the code they are testing. A module named `tests` should be created in each file to contain the test functions and to annotate the module with `#[cfg(test)]`
- The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`
- Testing private functions are often contentious. For example in Java you are required to create a different class for unit testing, so it cannot access private methods in the class-under-test. You have to make that method package-private to be test-able. However due to the nature of Rust where the tests module is put in the same file and we can use `super` to bring even the non-public functions into scope, Rust allows you to test private functions!

### Integration tests