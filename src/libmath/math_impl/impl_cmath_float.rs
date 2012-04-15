import libc::c_int;

#[link_name = "m"]
#[abi = "cdecl"]
native mod lib {

    // Alpabetically sorted by link_name

    #[link_name="acosf"] pure fn acos(n: t) -> t;
    #[link_name="asinf"] pure fn asin(n: t) -> t;
    #[link_name="atanf"] pure fn atan(n: t) -> t;
    #[link_name="atan2f"] pure fn atan2(a: t, b: t) -> t;
    #[link_name="cbrtf"] pure fn cbrt(n: t) -> t;
    #[link_name="ceilf"] pure fn ceil(n: t) -> t;
    #[link_name="copysignf"] pure fn copysign(x: t,
                                              y: t) -> t;
    #[link_name="cosf"] pure fn cos(n: t) -> t;
    #[link_name="coshf"] pure fn cosh(n: t) -> t;
    #[link_name="erff"] pure fn erf(n: t) -> t;
    #[link_name="erfcf"] pure fn erfc(n: t) -> t;
    #[link_name="expf"] pure fn exp(n: t) -> t;
    #[link_name="expm1f"]pure fn expm1(n: t) -> t;
    #[link_name="exp2f"] pure fn exp2(n: t) -> t;
    #[link_name="fabsf"] pure fn abs(n: t) -> t;
    #[link_name="fdimf"] pure fn abs_sub(a: t, b: t) -> t;
    #[link_name="floorf"] pure fn floor(n: t) -> t;
    #[link_name="frexpf"] pure fn frexp(n: t,
                                        &value: c_int) -> t;
    #[link_name="fmaf"] pure fn mul_add(a: t,
                                        b: t, c: t) -> t;
    #[link_name="fmaxf"] pure fn fmax(a: t, b: t) -> t;
    #[link_name="fminf"] pure fn fmin(a: t, b: t) -> t;
    #[link_name="nextafterf"] pure fn nextafter(x: t,
                                                y: t) -> t;
    #[link_name="hypotf"] pure fn hypot(x: t, y: t) -> t;
    #[link_name="ldexpf"] pure fn ldexp(x: t, n: c_int) -> t;

    #[cfg(target_os="linux")]
    #[cfg(target_os="macos")]
    #[cfg(target_os="freebsd")]
    #[link_name="lgammaf_r"] pure fn lgamma(n: t,
                                            &sign: c_int) -> t;

    #[cfg(target_os="win32")]
    #[link_name="__lgammaf_r"] pure fn lgamma(n: t,
                                              &sign: c_int) -> t;

    #[link_name="logf"] pure fn ln(n: t) -> t;
    #[link_name="logbf"] pure fn log_radix(n: t) -> t;
    #[link_name="log1pf"] pure fn ln1p(n: t) -> t;
    #[cfg(target_os="linux")]
    #[cfg(target_os="macos")]
    #[cfg(target_os="win32")]
    #[link_name="log2f"] pure fn log2(n: t) -> t;
    #[link_name="log10f"] pure fn log10(n: t) -> t;
    #[link_name="ilogbf"] pure fn ilog_radix(n: t) -> c_int;
    #[link_name="modff"] pure fn modf(n: t,
                                      &iptr: t) -> t;
    #[link_name="powf"] pure fn pow(n: t, e: t) -> t;
// FIXME enable when rounding modes become available
// (See Issue #1379)
//    #[link_name="rintf"] pure fn rint(n: t) -> t;
    #[link_name="roundf"] pure fn round(n: t) -> t;
    #[link_name="scalbnf"] pure fn ldexp_radix(n: t, i: c_int)
        -> t;
    #[link_name="sinf"] pure fn sin(n: t) -> t;
    #[link_name="sinhf"] pure fn sinh(n: t) -> t;
    #[link_name="sqrtf"] pure fn sqrt(n: t) -> t;
    #[link_name="tanf"] pure fn tan(n: t) -> t;
    #[link_name="tanhf"] pure fn tanh(n: t) -> t;
    #[link_name="tgammaf"] pure fn tgamma(n: t) -> t;
    #[link_name="truncf"] pure fn trunc(n: t) -> t;
}

mod consts {
    // PORT check these by running src/etc/machconsts.c for your architecture
    // FIXME obtain machine float/math constants automatically (Issue #1986)
    const radix: uint = 2u;
    const mantissa_digits: uint = 24u;
    const digits: uint = 6u;
    const min_exp: uint = -125u;
    const max_exp: uint = 128u;
    const min_10_exp: int = -37;
    const max_10_exp: int = 38;
}