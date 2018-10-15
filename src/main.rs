extern crate failure;

mod custom_fail_type;
mod error_kind_pair;

fn main() {
    // custom_fail_type::example();
    error_kind_pair::example();
}
