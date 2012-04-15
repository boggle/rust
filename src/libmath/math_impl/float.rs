type t = float;
type m = fXY::t;

const NaN: float = 0.0/0.0;
const infinity: float = 1.0/0.0;
const neg_infinity: float = -1.0/0.0;

mod consts {
    const zero: float = 0f;
    const one: float = 1f;
    const two: float = 2f;
    const three: float = 3f;
    const four: float = 4f;
    const ten: float = 10f;

    const min_value: float = fXY::consts::min_value as float;
    const max_value: float = fXY::consts::max_value as float;
    const epsilon: float = fXY::consts::epsilon as float;

    const pi: float = fXY::consts::pi as float;
    const frac_pi_2: float = fXY::consts::frac_pi_2 as float;
    const frac_pi_4: float = fXY::consts::frac_pi_4 as float;
    const frac_1_pi: float = fXY::consts::frac_1_pi as float;
    const frac_2_pi: float = fXY::consts::frac_2_pi as float;
    const frac_2_sqrtpi: float = fXY::consts::frac_2_sqrtpi as float;
    const sqrt2: float = fXY::consts::sqrt2 as float;
    const frac_1_sqrt2: float = fXY::consts::frac_1_sqrt2 as float;
    const e: float = fXY::consts::e as float;
    const log2_e: float = fXY::consts::log2_e as float;
    const log10_e: float = fXY::consts::log10_e as float;
    const ln_2: float = fXY::consts::ln_2 as float;
    const ln_10: float = fXY::consts::ln_10 as float;
}

