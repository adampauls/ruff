use crate::expression::parentheses::{
    default_expression_needs_parentheses, NeedsParentheses, Parentheses, Parenthesize,
};
use crate::{verbatim_text, FormatNodeRule, PyFormatter};
use ruff_formatter::{write, Buffer, FormatResult};
use rustpython_parser::ast::ExprCompare;

#[derive(Default)]
pub struct FormatExprCompare;

impl FormatNodeRule<ExprCompare> for FormatExprCompare {
    fn fmt_fields(&self, item: &ExprCompare, f: &mut PyFormatter) -> FormatResult<()> {
        write!(f, [verbatim_text(item.range)])
    }
}

impl NeedsParentheses for ExprCompare {
    fn needs_parentheses(&self, parenthesize: Parenthesize, source: &str) -> Parentheses {
        default_expression_needs_parentheses(self.into(), parenthesize, source)
    }
}
