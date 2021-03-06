

/*
 * The compiler code necessary to support the #fmt extension. Eventually this
 * should all get sucked into either the standard library extfmt module or the
 * compiler syntax extension plugin interface.
 */
import extfmt::ct::*;
import base::*;
import codemap::span;
import ext::build::*;
export expand_syntax_ext;

fn expand_syntax_ext(cx: ext_ctxt, sp: span, arg: ast::mac_arg,
                     _body: ast::mac_body) -> @ast::expr {
    let arg = get_mac_arg(cx,sp,arg);
    let args: [@ast::expr] =
        alt arg.node {
          ast::expr_vec(elts, _) { elts }
          _ {
            cx.span_fatal(sp, "#fmt requires arguments of the form `[...]`.")
          }
        };
    if vec::len::<@ast::expr>(args) == 0u {
        cx.span_fatal(sp, "#fmt requires a format string");
    }
    let fmt =
        expr_to_str(cx, args[0],
                    "first argument to #fmt must be a string literal.");
    let fmtspan = args[0].span;
    #debug("Format string:");
    log(debug, fmt);
    fn parse_fmt_err_(cx: ext_ctxt, sp: span, msg: str) -> ! {
        cx.span_fatal(sp, msg);
    }
    let parse_fmt_err = bind parse_fmt_err_(cx, fmtspan, _);
    let pieces = parse_fmt_string(fmt, parse_fmt_err);
    ret pieces_to_expr(cx, sp, pieces, args);
}

// FIXME: A lot of these functions for producing expressions can probably
// be factored out in common with other code that builds expressions.
// FIXME: Cleanup the naming of these functions
// NOTE: Moved many of the common ones to build.rs --kevina
fn pieces_to_expr(cx: ext_ctxt, sp: span, pieces: [piece], args: [@ast::expr])
   -> @ast::expr {
    fn make_path_vec(_cx: ext_ctxt, ident: ast::ident) -> [ast::ident] {
        ret ["extfmt", "rt", ident];
    }
    fn make_rt_path_expr(cx: ext_ctxt, sp: span, ident: str) -> @ast::expr {
        let path = make_path_vec(cx, ident);
        ret mk_path(cx, sp, path);
    }
    // Produces an AST expression that represents a RT::conv record,
    // which tells the RT::conv* functions how to perform the conversion

    fn make_rt_conv_expr(cx: ext_ctxt, sp: span, cnv: conv) -> @ast::expr {
        fn make_flags(cx: ext_ctxt, sp: span, flags: [flag]) -> @ast::expr {
            let mut flagexprs: [@ast::expr] = [];
            for flags.each {|f|
                let mut fstr;
                alt f {
                  flag_left_justify { fstr = "flag_left_justify"; }
                  flag_left_zero_pad { fstr = "flag_left_zero_pad"; }
                  flag_space_for_sign { fstr = "flag_space_for_sign"; }
                  flag_sign_always { fstr = "flag_sign_always"; }
                  flag_alternate { fstr = "flag_alternate"; }
                }
                flagexprs += [make_rt_path_expr(cx, sp, fstr)];
            }
            ret mk_vec_e(cx, sp, flagexprs);
        }
        fn make_count(cx: ext_ctxt, sp: span, cnt: count) -> @ast::expr {
            alt cnt {
              count_implied {
                ret make_rt_path_expr(cx, sp, "count_implied");
              }
              count_is(c) {
                let count_lit = mk_int(cx, sp, c);
                let count_is_path = make_path_vec(cx, "count_is");
                let count_is_args = [count_lit];
                ret mk_call(cx, sp, count_is_path, count_is_args);
              }
              _ { cx.span_unimpl(sp, "unimplemented #fmt conversion"); }
            }
        }
        fn make_ty(cx: ext_ctxt, sp: span, t: ty) -> @ast::expr {
            let mut rt_type;
            alt t {
              ty_hex(c) {
                alt c {
                  case_upper { rt_type = "ty_hex_upper"; }
                  case_lower { rt_type = "ty_hex_lower"; }
                }
              }
              ty_bits { rt_type = "ty_bits"; }
              ty_octal { rt_type = "ty_octal"; }
              _ { rt_type = "ty_default"; }
            }
            ret make_rt_path_expr(cx, sp, rt_type);
        }
        fn make_conv_rec(cx: ext_ctxt, sp: span, flags_expr: @ast::expr,
                         width_expr: @ast::expr, precision_expr: @ast::expr,
                         ty_expr: @ast::expr) -> @ast::expr {
            ret mk_rec_e(cx, sp,
                         [{ident: "flags", ex: flags_expr},
                          {ident: "width", ex: width_expr},
                          {ident: "precision", ex: precision_expr},
                          {ident: "ty", ex: ty_expr}]);
        }
        let rt_conv_flags = make_flags(cx, sp, cnv.flags);
        let rt_conv_width = make_count(cx, sp, cnv.width);
        let rt_conv_precision = make_count(cx, sp, cnv.precision);
        let rt_conv_ty = make_ty(cx, sp, cnv.ty);
        ret make_conv_rec(cx, sp, rt_conv_flags, rt_conv_width,
                          rt_conv_precision, rt_conv_ty);
    }
    fn make_conv_call(cx: ext_ctxt, sp: span, conv_type: str, cnv: conv,
                      arg: @ast::expr) -> @ast::expr {
        let fname = "conv_" + conv_type;
        let path = make_path_vec(cx, fname);
        let cnv_expr = make_rt_conv_expr(cx, sp, cnv);
        let args = [cnv_expr, arg];
        ret mk_call(cx, arg.span, path, args);
    }
    fn make_new_conv(cx: ext_ctxt, sp: span, cnv: conv, arg: @ast::expr) ->
       @ast::expr {
        // FIXME: Extract all this validation into extfmt::ct

        fn is_signed_type(cnv: conv) -> bool {
            alt cnv.ty {
              ty_int(s) {
                alt s { signed { ret true; } unsigned { ret false; } }
              }
              ty_float { ret true; }
              _ { ret false; }
            }
        }
        let unsupported = "conversion not supported in #fmt string";
        alt cnv.param {
          option::none { }
          _ { cx.span_unimpl(sp, unsupported); }
        }
        for cnv.flags.each {|f|
            alt f {
              flag_left_justify { }
              flag_sign_always {
                if !is_signed_type(cnv) {
                    cx.span_fatal(sp,
                                  "+ flag only valid in " +
                                      "signed #fmt conversion");
                }
              }
              flag_space_for_sign {
                if !is_signed_type(cnv) {
                    cx.span_fatal(sp,
                                  "space flag only valid in " +
                                      "signed #fmt conversions");
                }
              }
              flag_left_zero_pad { }
              _ { cx.span_unimpl(sp, unsupported); }
            }
        }
        alt cnv.width {
          count_implied { }
          count_is(_) { }
          _ { cx.span_unimpl(sp, unsupported); }
        }
        alt cnv.precision {
          count_implied { }
          count_is(_) { }
          _ { cx.span_unimpl(sp, unsupported); }
        }
        alt cnv.ty {
          ty_str { ret make_conv_call(cx, arg.span, "str", cnv, arg); }
          ty_int(sign) {
            alt sign {
              signed { ret make_conv_call(cx, arg.span, "int", cnv, arg); }
              unsigned {
                ret make_conv_call(cx, arg.span, "uint", cnv, arg);
              }
            }
          }
          ty_bool { ret make_conv_call(cx, arg.span, "bool", cnv, arg); }
          ty_char { ret make_conv_call(cx, arg.span, "char", cnv, arg); }
          ty_hex(_) { ret make_conv_call(cx, arg.span, "uint", cnv, arg); }
          ty_bits { ret make_conv_call(cx, arg.span, "uint", cnv, arg); }
          ty_octal { ret make_conv_call(cx, arg.span, "uint", cnv, arg); }
          ty_float { ret make_conv_call(cx, arg.span, "float", cnv, arg); }
          ty_poly { ret make_conv_call(cx, arg.span, "poly", cnv, arg); }
          _ { cx.span_unimpl(sp, unsupported); }
        }
    }
    fn log_conv(c: conv) {
        alt c.param {
          some(p) { log(debug, "param: " + int::to_str(p, 10u)); }
          _ { #debug("param: none"); }
        }
        for c.flags.each {|f|
            alt f {
              flag_left_justify { #debug("flag: left justify"); }
              flag_left_zero_pad { #debug("flag: left zero pad"); }
              flag_space_for_sign { #debug("flag: left space pad"); }
              flag_sign_always { #debug("flag: sign always"); }
              flag_alternate { #debug("flag: alternate"); }
            }
        }
        alt c.width {
          count_is(i) { log(debug,
                                 "width: count is " + int::to_str(i, 10u)); }
          count_is_param(i) {
            log(debug,
                     "width: count is param " + int::to_str(i, 10u));
          }
          count_is_next_param { #debug("width: count is next param"); }
          count_implied { #debug("width: count is implied"); }
        }
        alt c.precision {
          count_is(i) { log(debug,
                                 "prec: count is " + int::to_str(i, 10u)); }
          count_is_param(i) {
            log(debug,
                     "prec: count is param " + int::to_str(i, 10u));
          }
          count_is_next_param { #debug("prec: count is next param"); }
          count_implied { #debug("prec: count is implied"); }
        }
        alt c.ty {
          ty_bool { #debug("type: bool"); }
          ty_str { #debug("type: str"); }
          ty_char { #debug("type: char"); }
          ty_int(s) {
            alt s {
              signed { #debug("type: signed"); }
              unsigned { #debug("type: unsigned"); }
            }
          }
          ty_bits { #debug("type: bits"); }
          ty_hex(cs) {
            alt cs {
              case_upper { #debug("type: uhex"); }
              case_lower { #debug("type: lhex"); }
            }
          }
          ty_octal { #debug("type: octal"); }
          ty_float { #debug("type: float"); }
          ty_poly { #debug("type: poly"); }
        }
    }
    let fmt_sp = args[0].span;
    let mut n = 0u;
    let mut tmp_expr = mk_str(cx, sp, "");
    let nargs = vec::len::<@ast::expr>(args);
    for pieces.each {|pc|
        alt pc {
          piece_string(s) {
            let s_expr = mk_str(cx, fmt_sp, s);
            tmp_expr = mk_binary(cx, fmt_sp, ast::add, tmp_expr, s_expr);
          }
          piece_conv(conv) {
            n += 1u;
            if n >= nargs {
                cx.span_fatal(sp,
                              "not enough arguments to #fmt " +
                                  "for the given format string");
            }
            #debug("Building conversion:");
            log_conv(conv);
            let arg_expr = args[n];
            let c_expr = make_new_conv(cx, fmt_sp, conv, arg_expr);
            tmp_expr = mk_binary(cx, fmt_sp, ast::add, tmp_expr, c_expr);
          }
        }
    }
    let expected_nargs = n + 1u; // n conversions + the fmt string

    if expected_nargs < nargs {
        cx.span_fatal
            (sp, #fmt["too many arguments to #fmt. found %u, expected %u",
                           nargs, expected_nargs]);
    }
    ret tmp_expr;
}
//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
