use crate::eval::Object;
use crate::parser::{Statement, Expr, parse};
use crate::code::{make_op, OpCode};
use crate::lexer::lexer;
use crate::parser::Operator;

#[derive(Debug, PartialEq)]
pub struct ByteCode {
    pub instructions: Vec<u8>,
    pub constants: Vec<Object>
}

impl ByteCode {
    fn new() -> Self {
        ByteCode {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
}

fn add_constant(obj: Object, byte_code: &mut ByteCode) -> u16 {
    byte_code.constants.push(obj);
    (byte_code.constants.len() - 1) as u16 // cast to u16 because that is the size of our constant pool index
}

fn add_instruction(op_code: OpCode, byte_code: &mut ByteCode) -> u16 {
    let position_of_new_instruction = byte_code.instructions.len() as u16;
    byte_code.instructions.extend(make_op(op_code));

    position_of_new_instruction
}

fn compile_expression(expr: Expr, byte_code: &mut ByteCode) {
    match expr {
        Expr::Const(num) => {
            let const_index = add_constant(Object::Integer(num), byte_code);
            add_instruction(OpCode::OpConstant(const_index), byte_code);
        },
        Expr::Infix { left, operator, right } => {
            compile_expression(*left, byte_code);
            compile_expression(*right, byte_code);
            match operator {
                Operator::Plus => add_instruction(OpCode::OpAdd, byte_code),
                Operator::Minus => add_instruction(OpCode::OpSub, byte_code),
                Operator::Multiply => add_instruction(OpCode::OpMul, byte_code),
                Operator::Divide => add_instruction(OpCode::OpDiv, byte_code),
                _ => panic!("unsupported infix operator"),
            };
        },
        _ => panic!("unsupported expression"),
    }
}

fn compile(ast: Vec<Statement>) -> ByteCode {
    let mut byte_code = ByteCode::new();

    for statement in ast {
        match statement {
            Statement::Let { .. } => {},
            Statement::Return { .. } => {},
            Statement::Expression(expr) => {
                compile_expression(expr, &mut byte_code);

                // pop one element from the stack after each expression statement to clean up
                add_instruction(OpCode::OpPop, &mut byte_code);
            },
        }
    }

    byte_code
}

pub fn compile_from_source(input: &str) -> ByteCode {
    let mut tokens = lexer().parse(input.as_bytes()).unwrap();
    let ast = parse(&mut tokens);
    compile(ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_infix() {
        compile_infix_template("+", OpCode::OpAdd);
        compile_infix_template("-", OpCode::OpSub);
        compile_infix_template("*", OpCode::OpMul);
        compile_infix_template("/", OpCode::OpDiv);
    }

    fn compile_infix_template(infix_str: &str, op_code: OpCode) {
        let input = format!("1 {} 2;", infix_str);
        let byte_code = compile_from_source(&input);

        let expected_instructions = vec![
            OpCode::OpConstant(0),
            OpCode::OpConstant(1),
            op_code,
            OpCode::OpPop
        ]
            .into_iter()
            .flat_map(make_op)
            .collect();

        assert_eq!(
            ByteCode {
                instructions: expected_instructions,
                constants: vec![Object::Integer(1), Object::Integer(2)]
            },
            byte_code
        );
    }
}
