// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::gc::{Gc, GC};

// error-pattern:fail

fn fold_local() -> Gc<Vec<int>> {
    box(GC) vec!(0,0,0,0,0,0)
}

fn fold_remote() -> Gc<Vec<int>> {
    fail!();
}

fn main() {
    let _lss = (fold_local(), fold_remote());
}
