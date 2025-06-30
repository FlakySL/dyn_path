# Testing dyn_path

Testing dyn_path is a simple task, since the project is composed
by a single lib file that exports two macro rules, there is only
one test file in the `test` module at the program [source folder].

In here we only rely on the `cargo test` testing utility, since
we don't need anything more fancy.

You can simply add test cases to the `test` module with all the
use cases for your implementation and that should be OK.

[source folder]: ./src/
