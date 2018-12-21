use crate::eval::Object;
use crate::compiler::ByteCode;
use crate::code::convert_two_u8s_be_to_usize;

const STACK_SIZE : usize = 2048;

struct VM {
    instructions: Vec<u8>,
    constants: Vec<Object>,
    stack: [Object; STACK_SIZE],
    sp: usize, // stores the next FREE space on the stack
}

impl VM {
    fn new(byte_code: ByteCode) -> Self {
        VM {
            instructions: byte_code.instructions,
            constants: byte_code.constants,
            // we rely on the stack pointer to ensure we don't read uninitialized memory
            // this should have the same result as [Object::Null, STACK_SIZE] which is not allow because Object is not copy
            stack: unsafe { std::mem::zeroed() },
            sp: 0
        }
    }

    fn run(&mut self) {
        let mut ip = 0; // instruction pointer

        while ip < self.instructions.len() {
            let instruction_address = ip;
            ip += 1;

            match self.instructions[instruction_address] {
                0x01 => {
                    // OpConstant
                    let const_index = convert_two_u8s_be_to_usize(self.instructions[ip], self.instructions[ip + 1]);
                    ip += 2;
                    self.push(self.constants[const_index].clone());
                },
                0x02 => {
                    // OpPop
                    self.pop();
                },
                0x03 => {
                    // OpAdd
                    match (self.pop(), self.pop()) {
                        (Object::Integer(right), Object::Integer(left)) => self.push(Object::Integer(left + right)),
                        _ => panic!("unhandled argument types to OpAdd"),
                    }
                },
                0x04 => {
                    // OpSub
                    match (self.pop(), self.pop()) {
                        (Object::Integer(right), Object::Integer(left)) => self.push(Object::Integer(left - right)),
                        _ => panic!("unhandled argument types to OpSub"),
                    }
                },
                0x05 => {
                    // OpMul
                    match (self.pop(), self.pop()) {
                        (Object::Integer(right), Object::Integer(left)) => self.push(Object::Integer(left * right)),
                        _ => panic!("unhandled argument types to OpMul"),
                    }
                },
                0x06 => {
                    // OpDiv
                    match (self.pop(), self.pop()) {
                        (Object::Integer(right), Object::Integer(left)) => self.push(Object::Integer(left / right)),
                        _ => panic!("unhandled argument types to OpDiv"),
                    }
                },
                0x07 => {
                    // OpTrue
                    self.push(Object::Boolean(true));
                },
                0x08 => {
                    // OpFalse
                    self.push(Object::Boolean(false));
                },
                _ => panic!("unhandled instruction"),
            }
        }
    }

    fn push(&mut self, obj: Object) {
        self.stack[self.sp] = obj;
        self.sp += 1; // ignoring the potential stack overflow
    }

    fn pop(&mut self) -> Object {
        // ignoring the potential of stack underflow
        // cloning rather than mem::replace to support the last_popped method for testing
        let obj = self.stack[self.sp - 1].clone();
        self.sp -= 1;

        obj
    }

    fn last_popped(&self) -> &Object {
        // the stack pointer points to the next "free" space, which also holds the most recently popped element
        &self.stack[self.sp]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::compile_from_source;

    #[test]
    fn run_infix() {
        assert_last_popped("1 + 2;", Object::Integer(3));
        assert_last_popped("1 - 2;", Object::Integer(-1));
        assert_last_popped("3 * 2;", Object::Integer(6));
        assert_last_popped("6 / 2;", Object::Integer(3));
    }

    #[test]
    fn run_bool() {
        assert_last_popped("true;", Object::Boolean(true));
        assert_last_popped("false;", Object::Boolean(false));
    }

    fn assert_last_popped(input: &str, obj: Object) {
        let byte_code = compile_from_source(input);

        let mut vm = VM::new(byte_code);
        vm.run();

        assert_eq!(&obj, vm.last_popped());
    }
}
