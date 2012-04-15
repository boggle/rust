import libc::c_int;
import libc::c_double;

// Bessel functions are only guaranteed to exist for c_double,
// i.e. rust fp code needs to cast (with potential loss of precision)
type t = c_double;

#[link_name = "m"]
#[abi = "cdecl"]
native mod lib {
    pure fn j0(n: t) -> t;
    pure fn j1(n: t) -> t;
    pure fn jn(i: c_int, n: t) -> t;

    pure fn y0(n: t) -> t;
    pure fn y1(n: t) -> t;
    pure fn yn(i: c_int, n: t) -> t;
}