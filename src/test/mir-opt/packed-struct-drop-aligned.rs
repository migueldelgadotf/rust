// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-wasm32-bare compiled with panic=abort by default

fn main() {
    let mut x = Packed(Aligned(Droppy(0)));
    x.0 = Aligned(Droppy(0));
}

struct Aligned(Droppy);
#[repr(packed)]
struct Packed(Aligned);

struct Droppy(usize);
impl Drop for Droppy {
    fn drop(&mut self) {}
}

// END RUST SOURCE
// START rustc.main.EraseRegions.before.mir
// fn main() -> () {
//     let mut _0: ();
//     scope 1 {
//     }
//     scope 2 {
//         let mut _1: Packed;
//     }
//     let mut _2: Aligned;
//     let mut _3: Droppy;
//     let mut _4: Aligned;
//     let mut _5: Droppy;
//     let mut _6: Aligned;
//
//     bb0: {
//         StorageLive(_1);
//         ...
//         _1 = Packed(move _2,);
//         ...
//         StorageLive(_6);
//         _6 = move (_1.0: Aligned);
//         drop(_6) -> [return: bb4, unwind: bb3];
//     }
//     bb1: {
//         resume;
//     }
//     bb2: {
//         StorageDead(_1);
//         return;
//     }
//     bb3: {
//         (_1.0: Aligned) = move _4;
//         drop(_1) -> bb1;
//     }
//     bb4: {
//         StorageDead(_6);
//         (_1.0: Aligned) = move _4;
//         StorageDead(_4);
//         _0 = ();
//         drop(_1) -> [return: bb2, unwind: bb1];
//     }
// }
// END rustc.main.EraseRegions.before.mir
