extern crate m68k;

#[cfg(test)]
mod test_m68k {
    use m68k::addressing_mode::{AddressingMode, DataSize};
    use m68k::decoder::decode;
    use m68k::instruction_set::Instruction;
    use m68k::vm::VirtualMachine;

    // #[test]
    // fn test_decode() {
    //     let prg = vec![
    //         0x4E, 0x71, 0x4E, 0x71, 0x60, 0xFA, 0x01, 0x11, 0x66, 0xFC, 0x74, 0x25, 0x10, 0xDD,
    //         0x51, 0xCA, 0xFF, 0xFC, 0x34, 0x80, 0x32, 0x80,
    //     ];
    //     let mut pc = 0;
    //     loop {
    //         let h = prg[pc];
    //         pc += 1;
    //         let l = prg[pc];
    //         pc += 1;
    //         let op = (h << 8) | l;
    //         print!("{:X} {:016b}: ", op, op);
    //         let decoded = decode(op);
    //         println!("{:?}", decoded);
    //         if pc > prg.len() {
    //             break;
    //         }
    //     }
    // }

    #[test]
    fn test_decode_divu_w() {
        let opcode = 0b1000_000011_000001;
        let instruction = decode(opcode);
        assert_eq!(
            instruction,
            Instruction::DIVU(
                DataSize::Word,
                AddressingMode::DataDirect(1),
                AddressingMode::DataDirect(0),
            )
        );
    }

    #[test]
    fn test_decode_or_z() {
        // or.z a,Dd
        //             1000 ddd0zz aaaaaa
        let opcode = 0b1000_000000_000001;
        let instruction = decode(opcode);
        assert_eq!(
            instruction,
            Instruction::OR(
                DataSize::Byte,
                AddressingMode::DataDirect(1),
                AddressingMode::DataDirect(0),
            )
        );

        // or.z Ds,a
        //             1000 sss1zz aaaaaa
        let opcode = 0b1000_001100_010000;
        let instruction = decode(opcode);
        assert_eq!(
            instruction,
            Instruction::OR(
                DataSize::Byte,
                AddressingMode::DataDirect(1),
                AddressingMode::AddressIndirect(0),
            )
        );
    }

    #[test]
    fn test_decode_btst() {
        let opcode = 0b0000_100000_010010;
        let instruction = decode(opcode);
        assert_eq!(
            instruction,
            Instruction::BTST(
                DataSize::Byte,
                AddressingMode::Immediate,
                AddressingMode::AddressIndirect(0b010)
            )
        );

        let opcode = 0b0000_011100_000010;
        let instruction = decode(opcode);
        assert_eq!(
            instruction,
            Instruction::BTST(
                DataSize::LongWord,
                AddressingMode::DataDirect(0b011),
                AddressingMode::DataDirect(0b010)
            )
        );
    }

}
