import libc::c_int;

#[link_name = "m"]
#[abi = "cdecl"]
native mod lib {

    // Alpabetically sorted by link_name

    pure fn acos(n: t) -> t;
    pure fn asin(n: t) -> t;
    pure fn atan(n: t) -> t;
    pure fn atan2(a: t, b: t) -> t;
    pure fn cbrt(n: t) -> t;
    pure fn ceil(n: t) -> t;
    pure fn copysign(x: t, y: t) -> t;
    pure fn cos(n: t) -> t;
    pure fn cosh(n: t) -> t;
    pure fn erf(n: t) -> t;
    pure fn erfc(n: t) -> t;
    pure fn exp(n: t) -> t;
    pure fn expm1(n: t) -> t;
    pure fn exp2(n: t) -> t;
    #[link_name="fabs"] pure fn abs(n: t) -> t;
    // rename: for clarity and consistency with add/sub/mul/div
    #[link_name="fdim"] pure fn abs_sub(a: t, b: t) -> t;
    pure fn floor(n: t) -> t;
    // rename: for clarity and consistency with add/sub/mul/div
    #[link_name="fma"] pure fn mul_add(a: t, b: t,
                                       c: t) -> t;
    #[link_name="fmax"] pure fn fmax(a: t, b: t) -> t;
    #[link_name="fmin"] pure fn fmin(a: t, b: t) -> t;
    pure fn nextafter(x: t, y: t) -> t;
    pure fn hypot(x: t, y: t) -> t;
    // renamed: log is a reserved keyword; ln seems more natural, too
    #[link_name="log"] pure fn ln(n: t) -> t;
    // renamed: "logb" /often/ is confused for log2 by beginners
    #[link_name="logb"] pure fn log_radix(n: t) -> t;
    // renamed: to be consitent with log as ln
    #[link_name="log1p"] pure fn ln1p(n: t) -> t;
    pure fn log10(n: t) -> t;
    #[cfg(target_os="linux")]
    #[cfg(target_os="macos")]
    #[cfg(target_os="win32")]
    pure fn log2(n: t) -> t;
    #[link_name="ilogb"] pure fn ilog_radix(n: t) -> c_int;
    fn modf(n: t, iptr: &mut t) -> t;
    pure fn pow(n: t, e: t) -> t;
// FIXME enable when rounding modes become available
// (See Issue #1379)
//    pure fn rint(n: t) -> t;
    pure fn round(n: t) -> t;
    pure fn sin(n: t) -> t;
    pure fn sinh(n: t) -> t;
    pure fn sqrt(n: t) -> t;
    pure fn tan(n: t) -> t;
    pure fn tanh(n: t) -> t;
    pure fn tgamma(n: t) -> t;
    pure fn trunc(n: t) -> t;
}

#[link_name = "m"]
#[abi = "cdecl"]
native mod lib_wrapped {
    fn frexp(n: t, value: *mut c_int) -> t;
    pure fn ldexp(x: t, n: c_int) -> t;
    #[cfg(target_os = "linux")]
    #[cfg(target_os = "macos")]
    #[cfg(target_os = "freebsd")]
    #[link_name="lgamma_r"] fn lgamma(n: t, sign: *mut c_int) -> t;
    #[cfg(target_os = "win32")]
    #[link_name="__lgamma_r"] fn lgamma(n: t, sign: *mut c_int) -> t;
    // rename: for consistency with logradix
    #[link_name="scalbn"] pure fn ldexp_radix(n: t, i: c_int) -> t;
}

mod consts {
    // PORT check these by running src/etc/machconsts.c for your architecture
    // FIXME obtain machine float/math constants automatically (Issue #1986)
    const radix: uint = 2u;
    const mantissa_digits: uint = 53u;
    const digits: uint = 15u;
    const min_exp: uint = -1021u;
    const max_exp: uint = 1024u;
    const min_10_exp: int = -307;
    const max_10_exp: int = 308;
}