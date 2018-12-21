pub enum OpCode {
    OpConstant(u16), // args: pointer to constant table
    OpPop,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpTrue,
    OpFalse,
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
