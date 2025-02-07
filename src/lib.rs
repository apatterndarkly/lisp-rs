mod env;
mod eval;
mod lexer;
mod object;
mod parser;

use object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub fn lisp_rs_eval(input: &str) -> String {
    let mut env = Rc::new(RefCell::new(env::Env::new()));
    let val = eval::eval(input, &mut env);
    match val {
        Ok(Object::Void) => "".to_string(),
        Ok(Object::Integer(n)) => n.to_string(),
        Ok(Object::Bool(b)) => b.to_string(),
        Ok(Object::Symbol(s)) => s.to_string(),
        Ok(Object::Lambda(params, body, _)) => {
            let mut res = "Lambda(".to_string();
            for param in params {
                res.push_str(&format!("{} ", param));
            }
            res.push(')');
            for expr in (*body).iter() {
                res.push_str(&format!(" {}", expr));
            }
            res
        }
        Ok(Object::List(list)) => {
            let mut res = "(".to_string();
            for (i, obj) in (*list).iter().enumerate() {
                if i > 0 {
                    res.push(' ');
                }
                res.push_str(&format!("{}", obj));
            }
            res.push(')');
            res
        }
        Ok(Object::ListData(list)) => {
            let mut res = "(".to_string();
            for (i, obj) in list.iter().enumerate() {
                if i > 0 {
                    res.push(' ');
                }
                res.push_str(&format!("{}", obj));
            }
            res.push(')');
            res.to_string()
        }
        Ok(Object::String(s)) => s.to_string(),
        Ok(Object::Keyword(s)) => s.to_string(),
        Ok(Object::BinaryOp(s)) => s.to_string(),
        Ok(Object::Float(n)) => n.to_string(),
        Err(e) => e.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let program = "
            (map (lambda (x) (* x x)) (list 1 2 3 4 5))
        ";
        let result = lisp_rs_eval(program);
        assert_eq!(result, "(1 4 9 16 25)");
    }

    #[test]
    fn test_filter() {
        let program = "
            (filter (lambda (x) (= 0 (% x 2))) (list 1 2 3 4 5))
        ";
        let result = lisp_rs_eval(program);
        assert_eq!(result, "(2 4)");
    }

    #[test]
    fn test_reduce() {
        let program = "
            (reduce (lambda (x y) (+ x y)) (list 1 2 3 4 5))
        ";
        let result = lisp_rs_eval(program);
        assert_eq!(result, "15");
    }
}
