use crate::expression::Expr;
pub fn eval<'a>(
    expr: &'a Expr,
    vars: &mut Vec<(String, f64)>,
    funcs: &mut Vec<(String, Vec<String>, Expr)>,) 
    -> Result<f64, String> 
{
    match expr {
        Expr::Float(x) => Ok(*x),
        Expr::Neg(a) => Ok(-eval(a, vars, funcs)?),
        Expr::Add(a, b) => Ok(eval(a, vars, funcs)? + eval(b, vars, funcs)?),
        Expr::Sub(a, b) => Ok(eval(a, vars, funcs)? - eval(b, vars, funcs)?),
        Expr::Mul(a, b) => Ok(eval(a, vars, funcs)? * eval(b, vars, funcs)?),
        Expr::Div(a, b) => Ok(eval(a, vars, funcs)? / eval(b, vars, funcs)?),
        Expr::Var(name) => if let Some((_, val)) = vars.iter().rev().find(|(var, _)| var == name) {
            Ok(*val)
        } else {
            Err(format!("Cannot find variable `{}` in scope", name))
        },
        Expr::Let { name, rhs, then } => {
            let rhs = eval(rhs, vars, funcs)?;
            vars.push((name.clone(), rhs));
            let output = eval(then, vars, funcs);
            //vars.pop();
            output
        },
        Expr::Call(name, args) => if let Some((_, arg_names, body)) = funcs
            .iter()
            .rev()
            .find(|(var, _, _)| var == name)
            .cloned()
        {
            if arg_names.len() == args.len() {
                let mut args = args
                    .iter()
                    .map(|arg| eval(arg, vars, funcs))
                    .zip(arg_names.iter().cloned())
                    .map(|(val, name)| Ok((name, val?)))
                    .collect::<Result<_, String>>()?;
                vars.append(&mut args);
                let output = eval(&body, vars, funcs);
                vars.truncate(vars.len() - args.len());
                output
            } else {
                Err(format!(
                    "Wrong number of arguments for function `{}`: expected {}, found {}",
                    name,
                    arg_names.len(),
                    args.len(),
                ))
            }
        } else {
            Err(format!("Cannot find function `{}` in scope", name))
        },
        Expr::Fn { name, args, body, then } => {
            funcs.push((name.clone(), args.clone(), *body.clone()));
            let output = eval(then, vars, funcs);
            //funcs.pop();
            output
        },
        _ => { todo!("Not Implemented Yet!") }
    }
}