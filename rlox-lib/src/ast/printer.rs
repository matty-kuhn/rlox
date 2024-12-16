use super::Visitor;

#[derive(Default)]
struct AstPrinter;

impl Visitor for AstPrinter {
    type Output = String;

    fn visit_binary(&self, expr: &super::Bin) -> Self::Output {
        format!(
            "( {} {} {} )",
            expr.op,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }

    fn visit_unary(&self, expr: &super::Un) -> Self::Output {
        let sign = match expr {
            crate::ast::Un::Minus(_) => "-",
            crate::ast::Un::Bang(_) => "!",
        };
        format!("( {} {} )", sign, expr.inner().accept(self))
    }

    fn visit_literal(&self, expr: &super::Lit) -> Self::Output {
        format!(
            "{}",
            match expr {
                super::Lit::True => format!("true"),
                super::Lit::False => format!("false"),
                super::Lit::Nil => format!("nil"),
                super::Lit::Num(num) => num.to_string(),
                super::Lit::Str(s) => s.to_string(),
            }
        )
    }

    fn visit_grouping(&self, expr: &std::rc::Rc<super::Expr>) -> Self::Output {
        format!("( group {} )", expr.accept(self))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        ast::{Bin, Expr, Lit, Ops, Un},
        tokens::Value,
    };

    #[test]
    fn test_simpl_expr_visit() {
        let expr = Expr::Binary(Bin {
            left: Expr::Unary(Un::Minus(Expr::Literal(Lit::Num(Value::Num(123.0))).into())).into(),
            op: Ops::Star,
            right: Expr::Grouping(Expr::Literal(Lit::Num(Value::Num(45.67))).into()).into(),
        });
        let res = expr.accept(&AstPrinter);
        assert_eq!(res, "( * ( - 123 ) ( group 45.67 ) )")
    }
}
