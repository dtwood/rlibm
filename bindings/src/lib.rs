extern crate libc;
extern crate libm;

use libc::c_double;

macro_rules! extern_c {
    ($f: ident, $t: ty, $($x: ident),*) => {
        #[no_mangle]
        pub extern "C" fn $f($($x: $t),*) -> $t {
            libm::$f($($x),*)
        }
    };

    ($f: ident, $t: ty, $($x: ident,)*) => { extern_c!($f, $t, $($x),*); }
}

extern_c!(exp, c_double, x,);
extern_c!(sqrt, c_double, x);
