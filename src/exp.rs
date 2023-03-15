mod try_vec;
use try_vec::try_vec;
mod try_async;
use try_async::try_async;

use tungstenite_rs::{make_answer, show_streams, AnswerFn, HelperAttr};

make_answer!();

// Example: Basic function
#[show_streams]
fn invoke1() {}
// out: attr: ""
// out: item: "fn invoke1() { }"

// Example: Attribute with input
#[show_streams(bar)]
fn invoke2() {}
// out: attr: "bar"
// out: item: "fn invoke2() {}"

// Example: Multiple tokens in the input
#[show_streams(multiple => tokens)]
fn invoke3() {}
// out: attr: "multiple => tokens"
// out: item: "fn invoke3() {}"

// Example:
#[show_streams { delimiters }]
fn invoke4() {}
// out: attr: "delimiters"
// out: item: "fn invoke4() {}"

#[derive(AnswerFn)]
struct Struct;

#[derive(HelperAttr, Debug)]
struct Struct1 {
    #[helper]
    field: (),
}
