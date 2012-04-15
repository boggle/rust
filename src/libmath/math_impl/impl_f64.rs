type t = f64;

const NaN: t = 0.0_f64/0.0_f64;
const infinity: t = 1.0_f64/0.0_f64;
const neg_infinity: t = -1.0_f64/0.0_f64;

mod consts {
    const zero: t = 0.0f64;
    const one: t = 1.0f64;
    const two: t = 2.0f64;
    const three: t = 3.0f64;
    const four: t = 4.0f64;
    const ten: t = 10.0f64;

    // FIXME this is wrong! replace with hexadecimal (%a) constants below
    // (see Issue #1433)
    const min_value: t = 2.225074e-308_f64;
    const max_value: t = 1.797693e+308_f64;
    const epsilon: t = 2.220446e-16_f64;

    #[doc = "Archimedes' constant"]
    const pi: t = 3.14159265358979323846264338327950288_f64;

    #[doc = "pi/2.0"]
    const frac_pi_2: t = 1.57079632679489661923132169163975144_f64;

    #[doc = "pi/4.0"]
    const frac_pi_4: t = 0.785398163397448309615660845819875721_f64;

    #[doc = "1.0/pi"]
    const frac_1_pi: t = 0.318309886183790671537767526745028724_f64;

    #[doc = "2.0/pi"]
    const frac_2_pi: t = 0.636619772367581343075535053490057448_f64;

    #[doc = "2.0/sqrt(pi)"]
    const frac_2_sqrtpi: t = 1.12837916709551257389615890312154517_f64;

    #[doc = "sqrt(2.0)"]
    const sqrt2: t = 1.41421356237309504880168872420969808_f64;

    #[doc = "1.0/sqrt(2.0)"]
    const frac_1_sqrt2: t = 0.707106781186547524400844362104849039_f64;

    #[doc = "Euler's number"]
    const e: t = 2.71828182845904523536028747135266250_f64;

    #[doc = "log2(e)"]
    const log2_e: t = 1.44269504088896340735992468100189214_f64;

    #[doc = "log10(e)"]
    const log10_e: t = 0.434294481903251827651128918916605082_f64;

    #[doc = "ln(2.0)"]
    const ln_2: t = 0.693147180559945309417232121458176568_f64;

    #[doc = "ln(10.0)"]
    const ln_10: t = 2.30258509299404568401799145468436421_f64;
/*
    const min_value: c_double = 0x1p-1022_f64;
    const max_value: c_double = 0x1.fffffffffffffp+1023_f64;
    const epsilon: c_double = 0x1p-52_f64;

    const pi: c_double = 0x1.921fb54442d18p+1_f64;
    const div_1_pi: c_double = 0x1.45f306dc9c883p-2_f64;
    const div_2_pi: c_double = 0x1.45f306dc9c883p-1_f64;
    const div_pi_2: c_double = 0x1.921fb54442d18p+0_f64;
    const div_pi_4: c_double = 0x1.921fb54442d18p-1_f64;
    const div_2_sqrtpi: c_double = 0x1.20dd750429b6dp+0_f64;
    const e: c_double = 0x1.5bf0a8b145769p+1_f64;
    const log2_e: c_double = 0x1.71547652b82fep+0_f64;
    const log10_e: c_double = 0x1.bcb7b1526e50ep-2_f64;
    const ln_2: c_double = 0x1.62e42fefa39efp-1_f64;
    const ln_10: c_double = 0x1.26bb1bbb55516p+1_f64;
    const sqrt2: c_double = 0x1.6a09e667f3bcdp+0_f64;
    const div_1_sqrt2: c_double = 0x1.6a09e667f3bcdp-1_f64;
*/	
}