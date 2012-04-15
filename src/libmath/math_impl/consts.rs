export radix, mantissa_digits, digits, min_exp, max_exp, min_10_exp, max_10_exp;
export min_value, max_value, epsilon;
export pi, frac_pi_2, frac_pi_4, frac_1_pi, frac_2_pi, frac_2_sqrtpi, sqrt2, frac_1_sqrt2, e, log2_e, log10_e, ln_2, ln_10;
export zero, one, two, three, four, ten;

import cmath::consts::radix;
import cmath::consts::mantissa_digits;
import cmath::consts::digits;
import cmath::consts::min_exp;
import cmath::consts::max_exp;
import cmath::consts::min_10_exp;
import cmath::consts::max_10_exp;

import impl::consts::min_value;
import impl::consts::max_value;
import impl::consts::epsilon;

import impl::consts::pi;
import impl::consts::frac_pi_2;
import impl::consts::frac_pi_4;
import impl::consts::frac_1_pi;
import impl::consts::frac_2_pi;
import impl::consts::frac_2_sqrtpi;
import impl::consts::sqrt2;
import impl::consts::frac_1_sqrt2;
import impl::consts::e;
import impl::consts::log2_e;
import impl::consts::log10_e;
import impl::consts::ln_2;
import impl::consts::ln_10;

// Type abstract constants for writing generic floating point code

const zero: t = 0 as t;
const one: t = 1 as t;
const two: t = 2 as t;
const three: t = 3 as t;
const four: t = 4 as t;
const ten: t = 10 as t;