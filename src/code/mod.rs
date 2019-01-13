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
    OpGetGlobal(u16), // args: id of global
}

fn convert_u16_to_two_u8s_be(integer: u16) -> [u8; 2] {
    [(integer >> 8) as u8, integer as u8]
}

pub fn convert_two_u8s_be_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

fn make_three_byte_op(code: u8, data: u16) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(&convert_u16_to_two_u8s_be(data));

    output
}

pub fn make_op(op: OpCode) -> Vec<u8> {
    match op {
        OpCode::OpConstant(arg) => make_three_byte_op(0x01, arg),
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
        OpCode::OpJumpNotTrue(address) => make_three_byte_op(0x0E, address),
        OpCode::OpJump(address) => make_three_byte_op(0x0F, address),
        OpCode::OpSetGlobal(global_id) => make_three_byte_op(0x10, global_id),
        OpCode::OpGetGlobal(global_id) => make_three_byte_op(0x11, global_id),
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
