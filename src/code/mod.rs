pub enum OpCode {
    OpConstant(u16), // args: pointer to constant table
    OpPop,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpTrue,
    OpFalse,
    OpEquals,
    OpNotEquals,
    OpGreaterThan,
    OpMinus,
    OpBang,
    OpJumpNotTrue(u16), // args: byte address to jump to
    OpJump(u16), // args: byte address to jump to
    OpSetGlobal(u16), // args: id of global
}

fn convert_u16_to_two_u8s_be(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}
pub fn convert_two_u8s_be_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

pub fn make_op(op: OpCode) -> Vec<u8> {
    match op {
        OpCode::OpConstant(arg) => {
            let op_code = 0x01;
            let mut output = vec![op_code];
            output.extend(&convert_u16_to_two_u8s_be(arg));

            output
        },
        OpCode::OpPop => vec![0x02],
        OpCode::OpAdd => vec![0x03],
        OpCode::OpSub => vec![0x04],
        OpCode::OpMul => vec![0x05],
        OpCode::OpDiv => vec![0x06],
        OpCode::OpTrue => vec![0x07],
        OpCode::OpFalse => vec![0x08],
        OpCode::OpEquals => vec![0x09],
        OpCode::OpNotEquals => vec![0x0A],
        OpCode::OpGreaterThan => vec![0x0B],
        OpCode::OpMinus => vec![0x0C],
        OpCode::OpBang => vec![0x0D],
        OpCode::OpJumpNotTrue(address) => {
            let op_code = 0x0E;
            let mut output = vec![op_code];
            output.extend(&convert_u16_to_two_u8s_be(address));

            output
        },
        OpCode::OpJump(address) => {
            let op_code = 0x0F;
            let mut output = vec![op_code];
            output.extend(&convert_u16_to_two_u8s_be(address));

            output
        },
        OpCode::OpSetGlobal(global_id) => {
            let op_code = 0x10;
            let mut output = vec![op_code];
            output.extend(&convert_u16_to_two_u8s_be(global_id));

            output
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_op_constant() {
        assert_eq!(
            vec![0x01, 255, 254],
            make_op(OpCode::OpConstant(65534))
        );
    }

    #[test]
    fn make_op_pop() {
        assert_eq!(
            vec![0x02],
            make_op(OpCode::OpPop)
        );
    }

    #[test]
    fn make_op_add() {
        assert_eq!(
            vec![0x03],
            make_op(OpCode::OpAdd)
        );
    }
}
