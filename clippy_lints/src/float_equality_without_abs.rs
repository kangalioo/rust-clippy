use rustc_lint::{LateLintPass, LateContext};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_hir::*;
use rustc_errors::Applicability;

use if_chain::if_chain;
use crate::utils::{span_lint_and_sugg, match_qpath, snippet};

declare_clippy_lint! {
    /// **What it does:** Checks for statements of the form `(a - b) < f32::EPSILON` or
    /// `(a - b) < f64::EPSILON`. Note the missing `.abs()`.
    ///
    /// **Why is this bad?** The code without `.abs()` likely has a bug.
    ///
    /// **Known problems:** If the user can ensure that b is larger than a, the `.abs()` is
    /// technically unneccessary. However, it will make the code more robust and doesn't have any
    /// large performance implications. If the abs call was deliberately left out for performance
    /// reasons, it is probably better to state this explicitly in the code, which then can be done
    /// with an allow.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// pub fn is_roughly_equal(a: f32, b: f32) -> bool {
    ///     (a - b) < f32::EPSILON
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// pub fn is_roughly_equal(a: f32, b: f32) -> bool {
    ///     (a - b).abs() < f32::EPSILON
    /// }
    /// ```
    pub FLOAT_EQUALITY_WITHOUT_ABS,
    correctness,
    "float equality check without `.abs()`"
}

declare_lint_pass!(FloatEqualityWithoutAbs => [FLOAT_EQUALITY_WITHOUT_ABS]);

impl<'tcx> LateLintPass<'tcx> for FloatEqualityWithoutAbs {
    fn check_expr(&mut self, ctx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        // The two sides of the `(a - b) < EPSILON` comparison
        let a_minus_b;
        let epsilon;
        
        if let ExprKind::Binary(ref op, ref left, ref right) = expr.kind {
            if op.node == BinOpKind::Lt {
                a_minus_b = left;
                epsilon = right;
            } else if op.node == BinOpKind::Gt {
                epsilon = left;
                a_minus_b = right;
            } else {
                return;
            }
        } else {
            return;
        }

        if_chain! {
            // check if the side is actually of the form `(a - b)`
            if let ExprKind::Binary(ref op, ref _a, ref _b) = a_minus_b.kind;
            if BinOpKind::Sub == op.node;

            // check if `larger` is `f32::EPSILON` or `f64::EPSILON`
            if let ExprKind::Path(ref epsilon_path) = epsilon.kind;
            if match_qpath(epsilon_path, &["f32", "EPSILON"]) || match_qpath(epsilon_path, &["f64", "EPSILON"]);
            
            then {
                let a_minus_b_string = snippet(
                    ctx,
                    a_minus_b.span,
                    "(...)",
                );
                let suggestion = match a_minus_b_string.starts_with('(') {
                    true => format!("{}.abs()", a_minus_b_string),
                    false => format!("({}).abs()", a_minus_b_string),
                };

                span_lint_and_sugg(
                    ctx,
                    FLOAT_EQUALITY_WITHOUT_ABS,
                    expr.span,
                    "float equality check without `.abs()`",
                    "add `.abs()`",
                    suggestion,
                    Applicability::MaybeIncorrect,
                );
            }
        }
    }
}
