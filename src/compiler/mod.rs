use crate::eval::Object;
use crate::parser::{Statement, Expr, parse};
use crate::code::{make_op, OpCode};
use crate::lexer::lexer;
use crate::parser::Operator;
use crate::parser::Prefix;
use crate::compiler::symbol_table::SymbolTable;

mod symbol_table;

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

struct Compiler {
    byte_code: ByteCode,
    symbol_table: SymbolTable,
}

impl Compiler {
    fn compile_from_source(input: &str) -> ByteCode {
        let mut compiler = Compiler {
            byte_code: ByteCode::new(),
            symbol_table: SymbolTable::new(),
        };

        let mut tokens = lexer().parse(input.as_bytes()).unwrap();
        let ast = parse(&mut tokens);
        compiler.compile_statements(ast);

        compiler.byte_code
    }

    fn add_constant(&mut self, obj: Object) -> u16 {
        self.byte_code.constants.push(obj);
        (self.byte_code.constants.len() - 1) as u16 // cast to u16 because that is the size of our constant pool index
    }

    fn add_instruction(&mut self, op_code: OpCode) -> u16 {
        let position_of_new_instruction = self.byte_code.instructions.len() as u16;
        self.byte_code.instructions.extend(make_op(op_code));

        position_of_new_instruction
    }

    fn change_op(&mut self, position: usize, op_code: OpCode) {
        let op_bytes = make_op(op_code);

        self.byte_code.instructions.splice(position..position+op_bytes.len(), op_bytes);
    }

    fn compile_expression(&mut self, expr: Expr) {
        match expr {
            Expr::Const(num) => {
                let const_index = self.add_constant(Object::Integer(num));
                self.add_instruction(OpCode::OpConstant(const_index));
            },
            Expr::Infix { left, operator, right } => {
                match &operator {
                    Operator::LessThan => {
                        // flip left/right order so that less than statements can be re-written as greater than statements
                        // this allows the vm to only support a greater than instruction
                        self.compile_expression(*right);
                        self.compile_expression(*left);
                    },
                    _ => {
                        self.compile_expression(*left);
                        self.compile_expression(*right);
                    }
                }
                match operator {
                    Operator::Plus => self.add_instruction(OpCode::OpAdd),
                    Operator::Minus => self.add_instruction(OpCode::OpSub),
                    Operator::Multiply => self.add_instruction(OpCode::OpMul),
                    Operator::Divide => self.add_instruction(OpCode::OpDiv),
                    Operator::Equals => self.add_instruction(OpCode::OpEquals),
                    Operator::NotEquals => self.add_instruction(OpCode::OpNotEquals),
                    Operator::GreaterThan | Operator::LessThan => {
                        // greater than and less than can share one op-code because the
                        //    order of the operands are flipped when they are pushed on to the stack
                        self.add_instruction(OpCode::OpGreaterThan)
                    },
                };
            },
            Expr::Prefix {prefix: Prefix::Minus, value} => {
                self.compile_expression(*value);
                self.add_instruction(OpCode::OpMinus);
            },
            Expr::Prefix {prefix: Prefix::Bang, value} => {
                self.compile_expression(*value);
                self.add_instruction(OpCode::OpBang);
            },
            Expr::Boolean(true) => { self.add_instruction(OpCode::OpTrue); },
            Expr::Boolean(false) => { self.add_instruction(OpCode::OpFalse); },
            Expr::If {condition, consequence, alternative} => {
                self.compile_expression(*condition);
                let op_jump_position = self.byte_code.instructions.len();
                self.add_instruction(OpCode::OpJumpNotTrue(9999));
                self.compile_statements(consequence);
                if self.last_instruction_is_pop() {
                    self.remove_last_pop();
                }
                if alternative.is_empty() {
                    self.change_op(
                        op_jump_position,
                        OpCode::OpJumpNotTrue(self.byte_code.instructions.len() as u16)
                    );
                } else {
                    self.change_op(
                        op_jump_position,
                        OpCode::OpJumpNotTrue(self.byte_code.instructions.len() as u16 + 3) // plus three to account for extra jump at end of if block
                    );

                    let op_jump_position = self.byte_code.instructions.len();
                    self.add_instruction(OpCode::OpJump(9999));
                    self.compile_statements(alternative);
                    if self.last_instruction_is_pop() {
                        self.remove_last_pop();
                    }
                    self.change_op(
                        op_jump_position,
                        OpCode::OpJump(self.byte_code.instructions.len() as u16)
                    );
                }
            },
            Expr::Ident(name) => {
                match self.symbol_table.resolve(&name) {
                    None => panic!("attempted to use undefined variable"),
                    Some(index) => {
                        self.add_instruction(OpCode::OpGetGlobal(index));
                    },
                }
            },
            _ => panic!("unsupported expression"),
        };
    }

    fn last_instruction_is_pop(&self) -> bool {
        self.byte_code.instructions.last() == Some(&make_op(OpCode::OpPop)[0])
    }

    fn remove_last_pop(&mut self) {
        self.byte_code.instructions.pop();
    }

    fn compile_statements(&mut self, ast: Vec<Statement>) {
        for statement in ast {
            match statement {
                Statement::Let { name, value } => {
                    self.compile_expression(value);
                    let symbol_index = self.symbol_table.define(name);
                    self.add_instruction(OpCode::OpSetGlobal(symbol_index));
                },
                Statement::Return { .. } => unimplemented!(),
                Statement::Expression(expr) => {
                    self.compile_expression(expr);

                    // pop one element from the stack after each expression statement to clean up
                    self.add_instruction(OpCode::OpPop);
                },
            }
        }
    }

}

pub fn compile_from_source(input: &str) -> ByteCode {
    // wrap compiler method to hide compiler struct from outside this module
    Compiler::compile_from_source(input)
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
        let input = "if (true) { 10; } else { 20; };";
        let byte_code = compile_from_source(input);

        let expected_instructions = vec![
            OpCode::OpTrue, // 0000
            OpCode::OpJumpNotTrue(10), // 0001
            OpCode::OpConstant(0), // 0004
            OpCode::OpJump(13), // 0007
            OpCode::OpConstant(1), // 0010
            OpCode::OpPop, // 0013
        ]
            .into_iter()
            .flat_map(make_op)
            .collect();

        assert_eq!(
            ByteCode {
                instructions: expected_instructions,
                constants: vec![Object::Integer(10), Object::Integer(20)]
            },
            byte_code
        );
    }

    #[test]
    fn compile_if_else_extra_statement() {
        let input = "if (true) { 10; } else { 20; }; 3333;";
        let byte_code = compile_from_source(input);

        let expected_instructions = vec![
            OpCode::OpTrue, // 0000
            OpCode::OpJumpNotTrue(10), // 0001
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

    #[test]
    fn compile_let_single_var() {
        let input = "let one = 1;";
        let byte_code = compile_from_source(input);

        let expected_instructions = vec![
            OpCode::OpConstant(0),
            OpCode::OpSetGlobal(0),
        ]
            .into_iter()
            .flat_map(make_op)
            .collect();

        assert_eq!(
            ByteCode {
                instructions: expected_instructions,
                constants: vec![Object::Integer(1),]
            },
            byte_code
        );
    }

    #[test]
    fn compile_let_multiple_var() {
        let input = "let one = 1; let two = 2;";
        let byte_code = compile_from_source(input);

        let expected_instructions = vec![
            OpCode::OpConstant(0),
            OpCode::OpSetGlobal(0),
            OpCode::OpConstant(1),
            OpCode::OpSetGlobal(1),
        ]
            .into_iter()
            .flat_map(make_op)
            .collect();

        assert_eq!(
            ByteCode {
                instructions: expected_instructions,
                constants: vec![Object::Integer(1), Object::Integer(2),]
            },
            byte_code
        );
    }

    #[test]
    fn compile_let_get() {
        let input = "let one = 1; one;";
        let byte_code = compile_from_source(input);

        let expected_instructions = vec![
            OpCode::OpConstant(0),
            OpCode::OpSetGlobal(0),
            OpCode::OpGetGlobal(0),
            OpCode::OpPop,
        ]
            .into_iter()
            .flat_map(make_op)
            .collect();

        assert_eq!(
            ByteCode {
                instructions: expected_instructions,
                constants: vec![Object::Integer(1),]
            },
            byte_code
        );
    }
}
