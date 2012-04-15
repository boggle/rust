type t = float;

// Keep this in sync with impl_f32 and impl_f64

const NaN: t = fXY::NaN as t;
const infinity: t = fXY::infinity as t;
const neg_infinity: t = fXY::infinity as t;

mod consts {
   const min_value: t = fXY::consts::min_value as t;
   const max_value: t = fXY::consts::max_value as t;
   const epsilon: t = fXY::consts::epsilon as t;

	const pi: t = fXY::consts::pi as t;
	const frac_pi_2: t = fXY::consts::frac_pi_2 as t;
	const frac_pi_4: t = fXY::consts::frac_pi_4 as t;
	const frac_1_pi: t = fXY::consts::frac_1_pi as t;
	const frac_2_pi: t = fXY::consts::frac_2_pi as t;
	const frac_2_sqrtpi: t = fXY::consts::frac_2_sqrtpi as t;
	const sqrt2: t = fXY::consts::sqrt2 as t;
	const frac_1_sqrt2: t = fXY::consts::frac_1_sqrt2 as t;
	const e: t = fXY::consts::e as t;
	const log2_e: t = fXY::consts::log2_e as t;
	const log10_e: t = fXY::consts::log10_e as t;
	const ln_2: t = fXY::consts::ln_2 as t;
	const ln_10: t = fXY::consts::ln_10 as t;
}

