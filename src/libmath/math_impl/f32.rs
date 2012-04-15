type t = f32;

const NaN: t = 0.0_f32/0.0_f32;
const infinity: t = 1.0_f32/0.0_f32;
const neg_infinity: t = -1.0_f32/0.0_f32;

mod consts {

    // FIXME this is wrong! replace with hexadecimal (%a) constants below
    // (see Issue #1433)
    const min_value: t = 1.175494e-38_f32;
    const max_value: t = 3.402823e+38_f32;
    const epsilon: t = 0.000000_f32;

    #[doc = "Archimedes' constant"]
    const pi: t = 3.14159265358979323846264338327950288_f32;

    #[doc = "pi/2.0"]
    const frac_pi_2: t = 1.57079632679489661923132169163975144_f32;

    #[doc = "pi/4.0"]
    const frac_pi_4: t = 0.785398163397448309615660845819875721_f32;

    #[doc = "1.0/pi"]
    const frac_1_pi: t = 0.318309886183790671537767526745028724_f32;

    #[doc = "2.0/pi"]
    const frac_2_pi: t = 0.636619772367581343075535053490057448_f32;

    #[doc = "2.0/sqrt(pi)"]
    const frac_2_sqrtpi: t = 1.12837916709551257389615890312154517_f32;

    #[doc = "sqrt(2.0)"]
    const sqrt2: t = 1.41421356237309504880168872420969808_f32;

    #[doc = "1.0/sqrt(2.0)"]
    const frac_1_sqrt2: t = 0.707106781186547524400844362104849039_f32;

    #[doc = "Euler's number"]
    const e: t = 2.71828182845904523536028747135266250_f32;

    #[doc = "log2(e)"]
    const log2_e: t = 1.44269504088896340735992468100189214_f32;

    #[doc = "log10(e)"]
    const log10_e: t = 0.434294481903251827651128918916605082_f32;

    #[doc = "ln(2.0)"]
    const ln_2: t = 0.693147180559945309417232121458176568_f32;

    #[doc = "ln(10.0)"]
    const ln_10: t = 2.30258509299404568401799145468436421_f32;

/*
    const min_value: c_float = 0x1p-126_f32;
    const max_value: c_float = 0x1.fffffep+127_f32;
    const epsilon: c_float = 0x1p-23_f32;

    const pi: c_float = 0x1.921fb6p+1_f32;
    const div_1_pi: c_float = 0x1.45f306p-2_f32;
    const div_2_pi: c_float = 0x1.45f306p-1_f32;
    const div_pi_2: c_float = 0x1.921fb6p+0_f32;
    const div_pi_4: c_float = 0x1.921fb6p-1_f32;
    const div_2_sqrtpi: c_float = 0x1.20dd76p+0_f32;
    const e: c_float = 0x1.5bf0a8p+1_f32;
    const log2_e: c_float = 0x1.715476p+0_f32;
    const log10_e: c_float = 0x1.bcb7b2p-2_f32;
    const ln_2: c_float = 0x1.62e43p-1_f32;
    const ln_10: c_float = 0x1.26bb1cp+1_f32;
    const sqrt2: c_float = 0x1.6a09e6p+0_f32;
    const div_1_sqrt2: c_float = 0x1.6a09e6p-1_f32;
}
*/
}



