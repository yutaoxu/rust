// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-flags: -Z parse-only

fn main() {
    let a = 0;
    let _b = 0;
    let _ = 0;
    let mut b = 0;
    let mut _b = 0;
    let mut _ = 0; //~ ERROR expected identifier, found `_`
    //~^ NOTE `_` is a wildcard pattern, not an identifier
    //~| NOTE expected identifier
}
