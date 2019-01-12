use crate::eval::Object;
use crate::parser::{Statement, Expr, parse};
use crate::code::{make_op, OpCode};
use crate::lexer::lexer;
use crate::parser::Operator;
use crate::parser::Prefix;

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

fn change_op(position: usize, op_code: OpCode, byte_code: &mut ByteCode) {
    let op_bytes = make_op(op_code);

    byte_code.instructions.splice(position..position+op_bytes.len(), op_bytes);
}

fn compile_expression(expr: Expr, byte_code: &mut ByteCode) {
    match expr {
        Expr::Const(num) => {
            let const_index = add_constant(Object::Integer(num), byte_code);
            add_instruction(OpCode::OpConstant(const_index), byte_code);
        },
        Expr::Infix { left, operator, right } => {
            match &operator {
                Operator::LessThan => {
                    // flip left/right order so that less than statements can be re-written as greater than statements
                    // this allows the vm to only support a greater than instruction
                    compile_expression(*right, byte_code);
                    compile_expression(*left, byte_code);
                },
                _ => {
                    compile_expression(*left, byte_code);
                    compile_expression(*right, byte_code);
                }
            }
            match operator {
                Operator::Plus => add_instruction(OpCode::OpAdd, byte_code),
                Operator::Minus => add_instruction(OpCode::OpSub, byte_code),
                Operator::Multiply => add_instruction(OpCode::OpMul, byte_code),
                Operator::Divide => add_instruction(OpCode::OpDiv, byte_code),
                Operator::Equals => add_instruction(OpCode::OpEquals, byte_code),
                Operator::NotEquals => add_instruction(OpCode::OpNotEquals, byte_code),
                Operator::GreaterThan | Operator::LessThan => {
                    // greater than and less than can share one op-code because the
                    //    order of the operands are flipped when they are pushed on to the stack
                    add_instruction(OpCode::OpGreaterThan, byte_code)
                },
            };
        },
        Expr::Prefix {prefix: Prefix::Minus, value} => {
            compile_expression(*value, byte_code);
            add_instruction(OpCode::OpMinus, byte_code);
        },
        Expr::Prefix {prefix: Prefix::Bang, value} => {
            compile_expression(*value, byte_code);
            add_instruction(OpCode::OpBang, byte_code);
        },
        Expr::Boolean(true) => { add_instruction(OpCode::OpTrue, byte_code); },
        Expr::Boolean(false) => { add_instruction(OpCode::OpFalse, byte_code); },
        Expr::If {condition, consequence, alternative} => {
            compile_expression(*condition, byte_code);
            let op_jump_position = byte_code.instructions.len();
            add_instruction(OpCode::OpJumpNotTrue(9999), byte_code);
            compile(consequence, byte_code);
            if last_instruction_is_pop(byte_code) {
                remove_last_pop(byte_code);
            }
            change_op(
                op_jump_position,
                OpCode::OpJumpNotTrue(byte_code.instructions.len() as u16),
                byte_code
            );
            if !alternative.is_empty() {
                let op_jump_position = byte_code.instructions.len();
                add_instruction(OpCode::OpJump(9999), byte_code);
                compile(alternative, byte_code);
                if last_instruction_is_pop(byte_code) {
                    remove_last_pop(byte_code);
                }
                change_op(
                    op_jump_position,
                    OpCode::OpJump(byte_code.instructions.len() as u16),
                    byte_code
                );
            }
        },
        _ => panic!("unsupported expression"),
    };
}

fn last_instruction_is_pop(byte_code: &ByteCode) -> bool {
    byte_code.instructions.last() == Some(&make_op(OpCode::OpPop)[0])
}

fn remove_last_pop(byte_code: &mut ByteCode) {
    byte_code.instructions.pop();
}

fn compile(ast: Vec<Statement>, byte_code: &mut ByteCode) {
    for statement in ast {
        match statement {
            Statement::Let { .. } => {},
            Statement::Return { .. } => {},
            Statement::Expression(expr) => {
                compile_expression(expr, byte_code);

                // pop one element from the stack after each expression statement to clean up
                add_instruction(OpCode::OpPop, byte_code);
            },
        }
    }
}

pub fn compile_from_source(input: &str) -> ByteCode {
    let mut tokens = lexer().parse(input.as_bytes()).unwrap();
    let ast = parse(&mut tokens);
    let mut byte_code = ByteCode::new();
    compile(ast, &mut byte_code);

    byte_code
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

    #[test]
    fn compile_if() {
        let input = "if (true) { 10; }; 3333;";
        let byte_code = compile_from_source(input);

        let expected_instructions = vec![
            OpCode::OpTrue, // 0000
            OpCode::OpJumpNotTrue(7), // 0001
            OpCode::OpConstant(0), // 0004
            OpCode::OpPop, // 0007
            OpCode::OpConstant(1), // 0008
            OpCode::OpPop, // 0011
        ]
            .into_iter()
            .flat_map(make_op)
            .collect();

        assert_eq!(
            ByteCode {
                instructions: expected_instructions,
                constants: vec![Object::Integer(10), Object::Integer(3333)]
            },
            byte_code
        );
    }

    #[test]
    fn compile_if_else() {
        let input = "if (true) { 10; } else { 20; }; 3333;";
        let byte_code = compile_from_source(input);

        let expected_instructions = vec![
            OpCode::OpTrue, // 0000
            OpCode::OpJumpNotTrue(7), // 0001
            OpCode::OpConstant(0), // 0004
            OpCode::OpJump(13), // 0007
            OpCode::OpConstant(1), // 0010
            OpCode::OpPop, // 0013
            OpCode::OpConstant(2), // 0014
            OpCode::OpPop, // 0017
        ]
            .into_iter()
            .flat_map(make_op)
            .collect();

        assert_eq!(
            ByteCode {
                instructions: expected_instructions,
                constants: vec![Object::Integer(10), Object::Integer(20), Object::Integer(3333)]
            },
            byte_code
        );
    }
}
