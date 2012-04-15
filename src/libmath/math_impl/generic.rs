import libc::c_int;

#[inline] pure fn is_NaN(f: t) -> bool { f != f }
#[inline] pure fn negated(f: t) -> t { ret - f; }
#[inline] pure fn inverse(f: t) -> t { ret consts::one / f; }

#[inline] pure fn add(x: t, y: t) -> t { ret x + y; }
#[inline] pure fn sub(x: t, y: t) -> t { ret x - y; }
#[inline] pure fn mul(x: t, y: t) -> t { ret x * y; }
#[inline] pure fn div(x: t, y: t) -> t { ret x / y; }
#[inline] pure fn rem(x: t, y: t) -> t { ret x % y; }
#[inline] pure fn lt(x: t, y: t) -> bool { ret x < y; }
#[inline] pure fn le(x: t, y: t) -> bool { ret x <= y; }
#[inline] pure fn eq(x: t, y: t) -> bool { ret x == y; }
#[inline] pure fn ne(x: t, y: t) -> bool { ret x != y; }
#[inline] pure fn ge(x: t, y: t) -> bool { ret x >= y; }
#[inline] pure fn gt(x: t, y: t) -> bool { ret x > y; }

#[doc = "
Returns true if `x` is a positive number, including +0.0 and +Infinity
"]
#[inline]
pure fn is_positive(x: t) -> bool
    { ret x > consts::zero || (consts::one/x) == infinity; }

#[doc = "
Returns true if `x` is a negative number, including -0.0 and -Infinity
"]
#[inline]
pure fn is_negative(x: t) -> bool
    { ret x < consts::zero || (consts::one/x) == neg_infinity; }

#[doc = "
Returns true if `x` is a negative number, including -0.0 and -Infinity

This is the same as `is_negative`.
"]
#[inline]
pure fn is_nonpositive(x: t) -> bool {
  ret x < consts::zero || (consts::one/x) == neg_infinity;
}

#[doc = "
Returns true if `x` is a positive number, including +0.0 and +Infinity

This is the same as `is_positive`.)
"]
#[inline]
pure fn is_nonnegative(x: t) -> bool {
  ret x > consts::zero || (consts::one/x) == infinity;
}

#[doc = "
Returns true if `x` is a consts::zero number (positive or negative 0.0)
"]
#[inline]
pure fn is_zero(x: t) -> bool {
    ret x == consts::zero || x == -consts::zero;
}

#[doc = "Returns true if `x`is an infinite number"]
#[inline]
pure fn is_infinite(x: t) -> bool {
    ret x == infinity || x == neg_infinity;
}

#[doc = "Returns true if `x`is a finite number"]
#[inline]
pure fn is_finite(x: t) -> bool {
    ret !(is_NaN(x) || is_infinite(x));
}

#[inline]
pure fn signbit(x: t) -> int {
    if is_negative(x) { ret 1; } else { ret 0; }
}

impl extensions for t {
  #[inline] pure fn is_infinite() -> bool { ret is_infinite(self); }
  #[inline] pure fn is_finite() -> bool { ret is_finite(self); }
  #[inline] pure fn is_NaN() -> bool { ret is_NaN(self); }
  #[inline] pure fn is_zero() -> bool { ret is_zero(self); }
  #[inline] pure fn is_positive() -> bool { ret is_positive(self); }
  #[inline] pure fn is_nonpositive() -> bool { ret is_nonpositive(self); }
  #[inline] pure fn is_negative() -> bool { ret is_negative(self); }
  #[inline] pure fn is_nonnegative() -> bool { ret is_nonnegative(self); }
  #[inline] pure fn negated() -> t { ret negated(self); }
  #[inline] pure fn inverse() -> t { ret inverse(self); }
  #[inline] pure fn signbit() -> int { ret signbit(self); }
}

#[cfg(target_os="linux")]
#[cfg(target_os="macos")]
#[cfg(target_os="win32")]
#[inline]
pure fn logarithm(n: t, b: t) -> t {
    ret log2(n) / log2(b);
}

#[cfg(target_os="freebsd")]
#[inline]
pure fn logarithm(n: t, b: t) -> t {
    // FIXME check if it is good to use log2 instead of ln here;
    // in theory should be faster since the radix is 2
    // See Issue #2000
    ret ln(n) / ln(b);
}

#[cfg(target_os="freebsd")]
#[inline]
pure fn log2(n: t) -> t {
    ret ln(n) / consts::ln_2;
}

/**
 * Section: Arithmetics
 */

#[doc = "
Compute the exponentiation of an integer by another integer as a floating
point value

# Arguments

* x - The base
* pow - The exponent

# Return value

`NaN` if both `x` and `pow` are `0u`, otherwise `x^pow`
"]
pure fn pow_with_uint(base: uint, pow: uint) -> t {
   if base == 0u {
      if pow == 0u {
        ret NaN;
      }
       ret consts::zero;
   }
   let mut my_pow     = pow;
   let mut total      = consts::one;
   let mut multiplier = base as t;
   unsafe {
       while (my_pow > 0u) {
         if my_pow % 2u == 1u {
           total = total * multiplier;
         }
         my_pow     /= 2u;
         multiplier *= multiplier;
       }
   }
   ret total;
}

// TODO make this work with regions, cf c_(double,float)
#[inline] fn frexp(n: t, value: *mut int) -> t {
  ret cmath::lib_wrapped::frexp(n, value as *mut c_int);
}

#[inline] pure fn ldexp(x: t, n: int) -> t {
  ret cmath::lib_wrapped::ldexp(x, n as c_int);
}

// TODO make this work with regions, cf c_(double,float)
#[inline] fn lgamma(n: t, value: *mut int) -> t {
  ret cmath::lib_wrapped::lgamma(n, value as *mut c_int);
}

#[inline] pure fn ldexp_radix(x: t, i: int) -> t {
  ret cmath::lib_wrapped::ldexp_radix(x, i as c_int);
}

#[inline] pure fn j0(n: t) -> t
  { ret c_bessel::lib::j0(n as c_bessel::t) as t; }

#[inline] pure fn j1(n: t) -> t
  { ret c_bessel::lib::j1(n as c_bessel::t) as t; }

#[inline] pure fn jn(i: int, n: t) -> t
  { ret c_bessel::lib::jn(i as c_int, n as c_bessel::t) as t; }

#[inline] pure fn y0(n: t) -> t
  { ret c_bessel::lib::y0(n as c_bessel::t) as t; }

#[inline] pure fn y1(n: t) -> t
  { ret c_bessel::lib::y1(n as c_bessel::t) as t; }

#[inline] pure fn yn(i: int, n: t) -> t
  { ret c_bessel::lib::yn(i as c_int, n as c_bessel::t) as t; }
