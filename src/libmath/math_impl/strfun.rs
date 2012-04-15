
/**
 * Section: String Conversions
 */

#[doc = "
Converts a floating point value of type t to a string

# Arguments

* num - The floating point value
* digits - The number of significant digits
* exact - Whether to enforce the exact number of significant digits
"]
fn to_str_common(num: t, digits: uint, exact: bool) -> str {
    if is_NaN(num) { ret "NaN"; }
    if num == infinity { ret "inf"; }
    if num == neg_infinity { ret "-inf"; }
    let mut (num, accum) = if num < consts::zero {
      (-num, "-")
    } else {
      (num, "")
    };
    let trunc = num as uint;
    let mut frac = num - (trunc as t);
    accum += uint::str(trunc);
    if (frac < consts::epsilon && !exact) || digits == 0u { ret accum; }
    accum += ".";
    let mut i = digits;
    let mut epsilon = consts::one / pow_with_uint(10u, i);
    while i > 0u && (frac >= consts::epsilon || exact) {
        frac *= consts::ten;
        epsilon *= consts::ten;
        let digit = frac as uint;
        accum += uint::str(digit);
        frac -= digit as t;
        i -= 1u;
    }
    ret accum;

}

#[doc = "
Converts a floating point to a string with exactly the number of
provided significant digits

# Arguments

* num - The floating point value
* digits - The number of significant digits
"]
fn to_str_exact(num: t, digits: uint) -> str {
    to_str_common(num, digits, true)
}

#[doc = "
Converts a floating point to a string with a maximum number of
significant digits

# Arguments

* num - The floating point value
* digits - The number of significant digits
"]
fn to_str(num: t, digits: uint) -> str {
    to_str_common(num, digits, false)
}

#[doc = "
Convert a string to a floating point value

This function accepts strings such as

* '3.14'
* '+3.14', equivalent to '3.14'
* '-3.14'
* '2.5E10', or equivalently, '2.5e10'
* '2.5E-10'
* '', or, equivalently, '.' (understood as 0)
* '5.'
* '.5', or, equivalently,  '0.5'
* 'inf', '-inf', 'NaN'

Leading and trailing whitespace are ignored.

# Arguments

* num - A string

# Return value

`none` if the string did not represent a valid number.  Otherwise, `some(n)`
where `n` is the ting-point number represented by `[num]`.
"]
fn from_str(num: str) -> option<t> {
   if num == "inf" {
       ret some(infinity);
   } else if num == "-inf" {
       ret some(neg_infinity);
   } else if num == "NaN" {
       ret some(NaN);
   }

   let mut pos = 0u;               //Current byte position in the string.
                                   //Used to walk the string in O(n).
   let len = str::len(num);        //Length of the string, in bytes.

   if len == 0u { ret none; }
   let mut total = consts::zero;    //Accumulated result
   let mut c     = 'z';                   //Latest char.

   //The string must start with one of the following characters.
   alt str::char_at(num, 0u) {
      '-' | '+' | '0' to '9' | '.' {}
      _ { ret none; }
   }

   //Determine if first char is '-'/'+'. Set [pos] and [neg] accordingly.
   let mut neg = false;               //Sign of the result
   alt str::char_at(num, 0u) {
      '-' {
          neg = true;
          pos = 1u;
      }
      '+' {
          pos = 1u;
      }
      _ {}
   }

   //Examine the following chars until '.', 'e', 'E'
   while(pos < len) {
       let char_range = str::char_range_at(num, pos);
       c   = char_range.ch;
       pos = char_range.next;
       alt c {
         '0' to '9' {
           total = total * consts::ten;
           total += ((c as int) - ('0' as int)) as t;
         }
         '.' | 'e' | 'E' {
           break;
         }
         _ {
           ret none;
         }
       }
   }

   if c == '.' {//Examine decimal part
      let mut decimal = consts::one;
      while(pos < len) {
         let char_range = str::char_range_at(num, pos);
         c = char_range.ch;
         pos = char_range.next;
         alt c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6'| '7' | '8' | '9'  {
                 decimal /= consts::ten;
                 total += (((c as int) - ('0' as int)) as t)*decimal;
             }
             'e' | 'E' {
                 break;
             }
             _ {
                 ret none;
             }
         }
      }
   }

   if (c == 'e') | (c == 'E') {//Examine exponent
      let mut exponent = 0u;
      let mut neg_exponent = false;
      if(pos < len) {
          let char_range = str::char_range_at(num, pos);
          c   = char_range.ch;
          alt c  {
             '+' {
                pos = char_range.next;
             }
             '-' {
                pos = char_range.next;
                neg_exponent = true;
             }
             _ {}
          }
          while(pos < len) {
             let char_range = str::char_range_at(num, pos);
             c = char_range.ch;
             alt c {
                 '0' | '1' | '2' | '3' | '4' | '5' | '6'| '7' | '8' | '9' {
                     exponent *= 10u;
                     exponent += ((c as uint) - ('0' as uint));
                 }
                 _ {
                     break;
                 }
             }
             pos = char_range.next;
          }
          let multiplier = pow_with_uint(10u, exponent);
              //Note: not [int::pow], otherwise, we'll quickly
              //end up with a nice overflow
          if neg_exponent {
             total = total / multiplier;
          } else {
             total = total * multiplier;
          }
      } else {
         ret none;
      }
   }

   if(pos < len) {
     ret none;
   } else {
     if(neg) {
        total *= negated(consts::one);
     }
     ret some(total);
   }
}

