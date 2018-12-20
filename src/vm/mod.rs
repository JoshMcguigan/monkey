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

    fn stack_peek(&self) -> &Object {
        &self.stack[self.sp - 1]
    }

    fn run(&mut self) {
        let mut ip = 0; // instruction pointer

        while ip < self.instructions.len() {
            let instruction_address = ip;
            ip += 1;

            match self.instructions[instruction_address] {
                0 => {
                    // OpConstant
                    let const_index = convert_two_u8s_be_to_usize(self.instructions[ip], self.instructions[ip + 1]);
                    ip += 2;
                    self.push(self.constants[const_index].clone());
                },
                _ => panic!("unhandled instruction"),
            }
        }
    }

    fn push(&mut self, obj: Object) {
        self.stack[self.sp] = obj;
        self.sp += 1; // ignoring the potential stack overflow here
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::compile_from_source;

    #[test]
    fn it_works() {
        let input = "1 + 2;";
        let byte_code = compile_from_source(input);

        let mut vm = VM::new(byte_code);
        vm.run();

        assert_eq!(&Object::Integer(2), vm.stack_peek());
    }
}
