#[inline(always)]
pure fn is_NaN(f: t) -> bool { f != f }

#[inline(always)]
pure fn negated(f: t) -> t { ret - f; }

#[inline(always)]
pure fn inverse(f: t) -> t { ret  consts::one / f; }

#[inline(always)]
pure fn add(x: t, y: t) -> t { ret x + y; }

#[inline(always)]
pure fn sub(x: t, y: t) -> t { ret x - y; }

#[inline(always)]
pure fn mul(x: t, y: t) -> t { ret x * y; }

#[inline(always)]
pure fn div(x: t, y: t) -> t { ret x / y; }

#[inline(always)]
pure fn rem(x: t, y: t) -> t { ret x % y; }

#[inline(always)]
pure fn lt(x: t, y: t) -> bool { ret x < y; }

#[inline(always)]
pure fn le(x: t, y: t) -> bool { ret x <= y; }

#[inline(always)]
pure fn eq(x: t, y: t) -> bool { ret x == y; }

#[inline(always)]
pure fn ne(x: t, y: t) -> bool { ret x != y; }

#[inline(always)]
pure fn ge(x: t, y: t) -> bool { ret x >= y; }

#[inline(always)]
pure fn gt(x: t, y: t) -> bool { ret x > y; }

#[doc = "
Returns true if `x` is a positive number, including +0.0 and +Infinity
"]
#[inline(always)]
pure fn is_positive(x: t) -> bool
    { ret x > consts::zero || (consts::one/x) == infinity; }

#[doc = "
Returns true if `x` is a negative number, including -0.0 and -Infinity
"]
#[inline(always)]
pure fn is_negative(x: t) -> bool
    { ret x < consts::zero || (consts::one/x) == neg_infinity; }

#[doc = "
Returns true if `x` is a negative number, including -0.0 and -Infinity

This is the same as `is_negative`.
"]
#[inline(always)]
pure fn is_nonpositive(x: t) -> bool {
  ret x < consts::zero || (consts::one/x) == neg_infinity;
}

#[doc = "
Returns true if `x` is a positive number, including +0.0 and +Infinity

This is the same as `is_positive`.)
"]
#[inline(always)]
pure fn is_nonnegative(x: t) -> bool {
  ret x > consts::zero || (consts::one/x) == infinity;
}

#[doc = "
Returns true if `x` is a consts::zero number (positive or negative 0.0)
"]
#[inline(always)]
pure fn is_zero(x: t) -> bool {
    ret x == consts::zero || x == -consts::zero;
}

#[doc = "Returns true if `x`is an infinite number"]
#[inline(always)]
pure fn is_infinite(x: t) -> bool {
    ret x == infinity || x == neg_infinity;
}

#[doc = "Returns true if `x`is a finite number"]
#[inline(always)]
pure fn is_finite(x: t) -> bool {
    ret !(is_NaN(x) || is_infinite(x));
}

#[inline(always)]
pure fn signbit(x: t) -> int {
    if is_negative(x) { ret 1; } else { ret 0; }
}

#[cfg(target_os="linux")]
#[cfg(target_os="macos")]
#[cfg(target_os="win32")]
#[inline(always)]
pure fn logarithm(n: t, b: t) -> t {
    ret log2(n) / log2(b);
}

#[cfg(target_os="freebsd")]
#[inline(always)]
pure fn logarithm(n: t, b: t) -> t {
    // FIXME check if it is good to use log2 instead of ln here;
    // in theory should be faster since the radix is 2
    // See Issue #2000
    ret ln(n) / ln(b);
}

#[cfg(target_os="freebsd")]
#[inline(always)]
pure fn log2(n: t) -> t {
    ret ln(n) / consts::ln_2;
}


/**
 * Section: Arithmetics
 */

#[doc = "
Compute the exponentiation of an integer by another integer as a floating point value

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

