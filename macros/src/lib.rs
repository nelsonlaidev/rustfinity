#[macro_export]
macro_rules! math_operations {
    ($n1:expr, $op:expr, $n2:expr) => {{
        let n1 = $n1;
        let n2 = $n2;
        let op = $op;

        let result = match op {
            "+" => n1 + n2,
            "-" => n1 - n2,
            "*" => n1 * n2,
            "/" => {
                if n2 == 0 {
                    panic!("Division by zero");
                }
                n1 / n2
            }
            _ => {
                panic!("Unsupported operator: {}", op);
            }
        };

        format!("{} {} {} = {}", n1, op, n2, result)
    }};
}

pub fn main() {
    assert_eq!(math_operations!(4, "+", 2), "4 + 2 = 6");
    assert_eq!(math_operations!(10, "-", 3), "10 - 3 = 7");
    assert_eq!(math_operations!(6, "*", 4), "6 * 4 = 24");
    assert_eq!(math_operations!(15, "/", 3), "15 / 3 = 5");
}
