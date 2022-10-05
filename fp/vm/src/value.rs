use std::ops;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Nil,
    Int(i64),
    Float(f64),
    Str(String),
}

type OpResult = Result<Value, ()>;

impl ops::Add for Value {
    type Output = OpResult;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Nil => ops_utils::op_with_nil(),
            Self::Int(v) => ops_utils::add_int(v, rhs),
            Self::Float(v) => ops_utils::add_float(v, rhs),
            Self::Str(s) => ops_utils::add_str(s, rhs),
        }
    }
}

impl ops::Sub for Value {
    type Output = OpResult;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Nil => ops_utils::op_with_nil(),
            Self::Int(v) => ops_utils::sub_int(v, rhs),
            Self::Float(v) => ops_utils::sub_float(v, rhs),
            Self::Str(s) => ops_utils::sub_str(s, rhs),
        }
    }
}

impl ops::Mul for Value {
    type Output = OpResult;
    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Self::Nil => ops_utils::op_with_nil(),
            Self::Int(v) => ops_utils::mul_int(v, rhs),
            Self::Float(v) => ops_utils::mul_float(v, rhs),
            Self::Str(s) => ops_utils::mul_str(s, rhs),
        }
    }
}

impl ops::Div for Value {
    type Output = OpResult;
    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Self::Nil => ops_utils::op_with_nil(),
            Self::Int(v) => ops_utils::div_int(v, rhs),
            Self::Float(v) => ops_utils::div_float(v, rhs),
            Self::Str(s) => ops_utils::div_str(s, rhs),
        }
    }
}

mod ops_utils {
    use super::{OpResult, Value};

    // TODO: use custom #[derive] macros to impl add, sub...

    pub(super) fn op_with_nil() -> OpResult {
        Err(())
    }

    // add
    pub(super) fn add_int(lhs: i64, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => Ok(Value::Int(lhs + v)),
            Value::Float(v) => Ok(Value::Float(lhs as f64 + v)),
            Value::Str(s) => {
                let lhs = lhs.to_string();
                Ok(Value::Str(lhs + &s))
            }
        }
    }

    pub(super) fn add_float(lhs: f64, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => Ok(Value::Float(lhs + v as f64)),
            Value::Float(v) => Ok(Value::Float(lhs as f64 + v)),
            Value::Str(s) => {
                let lhs = lhs.to_string();
                Ok(Value::Str(lhs + &s))
            }
        }
    }

    pub(super) fn add_str(lhs: String, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => {
                let v = v.to_string();
                Ok(Value::Str(lhs + &v))
            }
            Value::Float(v) => {
                let v = v.to_string();
                Ok(Value::Str(lhs + &v))
            }
            Value::Str(s) => Ok(Value::Str(lhs + &s)),
        }
    }

    // sub
    pub(super) fn sub_int(lhs: i64, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => Ok(Value::Int(lhs - v)),
            Value::Float(v) => Ok(Value::Float(lhs as f64 - v)),
            Value::Str(_) => Err(()),
        }
    }

    pub(super) fn sub_float(lhs: f64, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => Ok(Value::Float(lhs - v as f64)),
            Value::Float(v) => Ok(Value::Float(lhs as f64 - v)),
            Value::Str(_) => Err(()),
        }
    }

    pub(super) fn sub_str(_lhs: String, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(_) => Err(()),
            Value::Float(_) => Err(()),
            Value::Str(_) => Err(()),
        }
    }

    // mul
    pub(super) fn mul_int(lhs: i64, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => Ok(Value::Int(lhs * v)),
            Value::Float(v) => Ok(Value::Float(lhs as f64 * v)),
            Value::Str(_) => Err(()),
        }
    }

    pub(super) fn mul_float(lhs: f64, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => Ok(Value::Float(lhs * v as f64)),
            Value::Float(v) => Ok(Value::Float(lhs as f64 * v)),
            Value::Str(_) => Err(()),
        }
    }

    pub(super) fn mul_str(lhs: String, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => {
                if v < 0 {
                    return Err(());
                }
                let result = lhs.repeat(v as usize);
                Ok(Value::Str(result))
            }
            Value::Float(_) => Err(()),
            Value::Str(_) => Err(()),
        }
    }

    // div
    pub(super) fn div_int(lhs: i64, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => Ok(Value::Float(lhs as f64 / v as f64)),
            Value::Float(v) => Ok(Value::Float(lhs as f64 / v)),
            Value::Str(_) => Err(()),
        }
    }

    pub(super) fn div_float(lhs: f64, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(v) => Ok(Value::Float(lhs / v as f64)),
            Value::Float(v) => Ok(Value::Float(lhs as f64 / v)),
            Value::Str(_) => Err(()),
        }
    }

    pub(super) fn div_str(_lhs: String, rhs: Value) -> OpResult {
        match rhs {
            Value::Nil => Err(()),
            Value::Int(_) => Err(()),
            Value::Float(_) => Err(()),
            Value::Str(_) => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{OpResult, Value};

    // this function and macro is just ensure result is right
    // dont use these things for test
    fn value_op_any(
        a: Value,
        b_and_result: Vec<(Value, OpResult)>,
        op: Box<dyn Fn(Value, Value) -> OpResult>,
    ) {
        for (b, r) in b_and_result {
            let a = a.clone();
            let result = op(a, b);
            assert_eq!(result, r)
        }
    }

    macro_rules! make_a_and_result {
        ($r_nil: expr, $r_int: expr, $r_float: expr, $r_str: expr) => {
            vec![
                (Value::Nil, $r_nil),
                (Value::Int(42), $r_int),
                (Value::Float(42.1), $r_float),
                (Value::Str("test".to_owned()), $r_str),
            ]
        };
    }

    #[test]
    fn any_add_any() {
        let a = Value::Nil;
        let b_and_result = make_a_and_result!(Err(()), Err(()), Err(()), Err(()));
        value_op_any(a, b_and_result, Box::new(|a, b| a + b));
    }

    #[test]
    fn nil_sub_any() {
        let a = Value::Nil;
        let b_and_result = make_a_and_result!(Err(()), Err(()), Err(()), Err(()));
        value_op_any(a, b_and_result, Box::new(|a, b| a - b));
    }

    #[test]
    fn nil_mul_any() {
        let a = Value::Nil;
        let b_and_result = make_a_and_result!(Err(()), Err(()), Err(()), Err(()));
        value_op_any(a, b_and_result, Box::new(|a, b| a * b));
    }

    #[test]
    fn nil_div_any() {
        let a = Value::Nil;
        let b_and_result = make_a_and_result!(Err(()), Err(()), Err(()), Err(()));
        value_op_any(a, b_and_result, Box::new(|a, b| a / b));
    }

    #[test]
    fn int_add_any() {
        let a = Value::Int(12);
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Int(54)),
            Ok(Value::Float(54.1)),
            Ok(Value::Str("12test".to_owned()))
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a + b));
    }

    #[test]
    fn int_sub_any() {
        let a = Value::Int(12);
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Int(-30)),
            Ok(Value::Float(-30.1)),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a - b));
    }

    #[test]
    fn int_mul_any() {
        let a = Value::Int(12);
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Int(504)),
            Ok(Value::Float(12 as f64 * 42.1)),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a * b));
    }

    #[test]
    fn int_div_any() {
        let a = Value::Int(12);
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Float(12 as f64 / 42 as f64)),
            Ok(Value::Float(12 as f64 / 42.1)),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a / b));
    }

    #[test]
    fn float_add_any() {
        let a = Value::Float(12.1);
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Float(54.1)),
            Ok(Value::Float(54.2)),
            Ok(Value::Str("12.1test".to_owned()))
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a + b));
    }

    #[test]
    fn float_sub_any() {
        let a = Value::Float(12.1);
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Float(-29.9)),
            Ok(Value::Float(-30.0)),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a - b));
    }

    #[test]
    fn float_mul_any() {
        let a = Value::Float(12.1);
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Float(12.1 * 42 as f64)),
            Ok(Value::Float(12.1 * 42.1)),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a * b));
    }
    #[test]
    fn float_div_any() {
        let a = Value::Float(12.1);
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Float(12.1 / 42 as f64)),
            Ok(Value::Float(12.1 / 42.1)),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a / b));
    }

    #[test]
    fn str_add_any() {
        let a = Value::Str("a test str".to_owned());
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Str("a test str42".to_owned())),
            Ok(Value::Str("a test str42.1".to_owned())),
            Ok(Value::Str("a test strtest".to_owned()))
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a + b));
    }

    #[test]
    fn str_sub_any() {
        let a = Value::Str("a test str".to_owned());
        let b_and_result = make_a_and_result!(
            Err(()),
            Err(()),
            Err(()),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a - b));
    }
    
    #[test]
    fn str_mul_any() {
        let a = Value::Str("a test str".to_owned());
        let b_and_result = make_a_and_result!(
            Err(()),
            Ok(Value::Str("a test str".to_owned().repeat(42))),
            Err(()),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a * b));
    }

    #[test]
    fn str_div_any() {
        let a = Value::Str("a test str".to_owned());
        let b_and_result = make_a_and_result!(
            Err(()),
            Err(()),
            Err(()),
            Err(())
        );
        value_op_any(a, b_and_result, Box::new(|a, b| a / b));
    }
}
