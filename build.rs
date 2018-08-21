// Bitcoin secp256k1 bindings
// Written in 2015 by
//   Andrew Poelstra
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! # Build script

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![warn(missing_docs)]

extern crate cc;

use std::io::{self, Write};

fn main() {
    // Check whether we can use 64-bit compilation
    #[cfg(target_pointer_width = "64")]
    let use_64bit_compilation = {
        let check = cc::Build::new().file("depend/check_uint128_t.c")
                                    .cargo_metadata(false)
                                    .try_compile("check_uint128_t")
                                    .is_ok();
        if !check {
            writeln!(
                &mut io::stderr(),
                "Warning: Compiling in 32-bit mode on a 64-bit architecture due to lack of uint128_t support."
            ).expect("print to stderr")
        }
        check
    };
    #[cfg(not(target_pointer_width = "64"))]
    let use_64bit_compilation = false;


    // Actual build
    let mut base_config = cc::Build::new();
    base_config.include("depend/secp256k1/")
               .include("depend/secp256k1/include")
               .include("depend/secp256k1/src")
               .flag("-g")
               .flag("-Wno-unused-function") // some ecmult stuff is defined but not used upstream
               .define("SECP256K1_BUILD", Some("1"))
               // TODO these three should be changed to use libgmp, at least until secp PR 290 is merged
               .define("USE_NUM_NONE", Some("1"))
               .define("USE_FIELD_INV_BUILTIN", Some("1"))
               .define("USE_SCALAR_INV_BUILTIN", Some("1"))
               .define("USE_ENDOMORPHISM", Some("1"))
               .define("ENABLE_MODULE_ECDH", Some("1"))
               .define("ENABLE_MODULE_RECOVERY", Some("1"));

    if use_64bit_compilation {
        base_config.define("USE_FIELD_5X52", Some("1"))
                   .define("USE_SCALAR_4X64", Some("1"))
                   .define("HAVE___INT128", Some("1"));
    } else {
        base_config.define("USE_FIELD_10X26", Some("1"))
                   .define("USE_SCALAR_8X32", Some("1"));
    }

    // secp256k1
    base_config.file("depend/secp256k1/contrib/lax_der_parsing.c")
               .file("depend/secp256k1/src/secp256k1.c")
               .compile("libsecp256k1.a");
}

