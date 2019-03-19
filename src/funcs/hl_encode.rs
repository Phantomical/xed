use crate::*;
use smallvec::SmallVec;
use xed_sys2::xed_interface::*;

/// Create a memory displacement
///
/// # Parameters
/// - `displacement`: The value of the displacement
/// - `displacement_bits`: The width of the displacement in bits.
///    Typically 8 or 32.
pub fn disp(displacement: u64, displacement_bits: u32) -> EncDisplacement {
    unsafe { xed_disp(displacement, displacement_bits) }
}

/// A first immediate operand (known as IMM0)
///
/// # Parameters
/// - `v`: An immediate operand.
/// - `width_bits`: The immediate width in bits.
pub fn imm0(v: u64, width_bits: u32) -> EncoderOperand {
    unsafe { xed_imm0(v, width_bits).into() }
}

/// The 2nd immedate operand (known as IMM1) for rare
/// instructions that require it.
///
/// # Parameters
/// - `v`: The 2nd immediate (byte-width) operand.
pub fn imm1(v: u8) -> EncoderOperand {
    unsafe { xed_imm1(v).into() }
}

/// Instruction with an array of operands.
///
/// The maximum number is `XED_ENCODER_OPERANDS_MAX`. The
/// array's contents are copied.
///
/// # Parameters
/// - `mode`: The `State` included the machine mode
///   and stack address width.
/// - `iclass`: The `IClass`.
/// - `effective_operand_width`: In bits.
/// - `operand_array`: An array of `EncoderOperand` objects.
pub fn inst(
    mode: State,
    iclass: IClass,
    effective_operand_width: u32,
    operands: &[EncoderOperand],
) -> EncoderInstruction {
    unsafe {
        let mut val: xed_encoder_instruction_t = std::mem::uninitialized();
        let mut args = SmallVec::<[xed_encoder_operand_t; 16]>::new();
        args.reserve_exact(operands.len());

        for operand in operands {
            args.push((*operand).into());
        }

        xed_inst(
            &mut val as *mut _,
            mode.into_inner(),
            iclass.into(),
            effective_operand_width,
            operands.len() as u32,
            args.as_ptr(),
        );

        val.into()
    }
}

/// Instruction with no operands.
///
/// # Parameters
/// - `mode`: The [`State`](crate::State) to be filled in.
/// - `iclass`: The [`IClass`](crate::IClass).
/// - `effective_operand_width`: In bits.
pub fn inst0(mode: State, iclass: IClass, effective_operand_width: u32) -> EncoderInstruction {
    unsafe {
        let mut val: xed_encoder_instruction_t = std::mem::uninitialized();

        xed_inst0(
            &mut val as &mut _,
            mode.into_inner(),
            iclass.into(),
            effective_operand_width,
        );

        val.into()
    }
}

/// Instruction with one operand.
///
/// # Parameters
/// - `mode`: The [`State`](crate::State) to be filled in.
/// - `iclass`: The [`IClass`](crate::IClass).
/// - `effective_operand_width`: In bits.
/// - `op0`: The operand.
pub fn inst1(
    mode: State,
    iclass: IClass,
    effective_operand_width: u32,
    op0: EncoderOperand,
) -> EncoderInstruction {
    unsafe {
        let mut val: xed_encoder_instruction_t = std::mem::uninitialized();

        xed_inst1(
            &mut val as &mut _,
            mode.into_inner(),
            iclass.into(),
            effective_operand_width,
            op0.into_inner(),
        );

        val.into()
    }
}

/// Instruction with two operands.
///
/// # Parameters
/// - `mode`: The [`State`](crate::State) to be filled in.
/// - `iclass`: The [`IClass`](crate::IClass).
/// - `effective_operand_width`: In bits.
/// - `op0`: The 1st operand.
/// - `op1`: The 2nd operand.
pub fn inst2(
    mode: State,
    iclass: IClass,
    effective_operand_width: u32,
    op0: EncoderOperand,
    op1: EncoderOperand,
) -> EncoderInstruction {
    unsafe {
        let mut val: xed_encoder_instruction_t = std::mem::uninitialized();

        xed_inst2(
            &mut val as &mut _,
            mode.into_inner(),
            iclass.into(),
            effective_operand_width,
            op0.into_inner(),
            op1.into_inner(),
        );

        val.into()
    }
}

/// Instruction with three operands.
///
/// # Parameters
/// - `mode`: The [`State`](crate::State) to be filled in.
/// - `iclass`: The [`IClass`](crate::IClass).
/// - `effective_operand_width`: In bits.
/// - `op0`: The 1st operand.
/// - `op1`: The 2nd operand.
/// - `op2`: The 3rd operand.
pub fn inst3(
    mode: State,
    iclass: IClass,
    effective_operand_width: u32,
    op0: EncoderOperand,
    op1: EncoderOperand,
    op2: EncoderOperand,
) -> EncoderInstruction {
    unsafe {
        let mut val: xed_encoder_instruction_t = std::mem::uninitialized();

        xed_inst3(
            &mut val as &mut _,
            mode.into_inner(),
            iclass.into(),
            effective_operand_width,
            op0.into_inner(),
            op1.into_inner(),
            op2.into_inner(),
        );

        val.into()
    }
}

/// Instruction with four operands.
///
/// # Parameters
/// - `mode`: The [`State`](crate::State) to be filled in.
/// - `iclass`: The [`IClass`](crate::IClass).
/// - `effective_operand_width`: In bits.
/// - `op0`: The 1st operand.
/// - `op1`: The 2nd operand.
/// - `op2`: The 3rd operand.
/// - `op3`: The 4th operand.
pub fn inst4(
    mode: State,
    iclass: IClass,
    effective_operand_width: u32,
    op0: EncoderOperand,
    op1: EncoderOperand,
    op2: EncoderOperand,
    op3: EncoderOperand,
) -> EncoderInstruction {
    unsafe {
        let mut val: xed_encoder_instruction_t = std::mem::uninitialized();

        xed_inst4(
            &mut val as &mut _,
            mode.into_inner(),
            iclass.into(),
            effective_operand_width,
            op0.into_inner(),
            op1.into_inner(),
            op2.into_inner(),
            op3.into_inner(),
        );

        val.into()
    }
}

/// Instruction with five operands.
///
/// # Parameters
/// - `mode`: The [`State`](crate::State) to be filled in.
/// - `iclass`: The [`IClass`](crate::IClass).
/// - `effective_operand_width`: In bits.
/// - `op0`: The 1st operand.
/// - `op1`: The 2nd operand.
/// - `op2`: The 3rd operand.
/// - `op3`: The 4th operand.
/// - `op4`: The 5th operand.
pub fn inst5(
    mode: State,
    iclass: IClass,
    effective_operand_width: u32,
    op0: EncoderOperand,
    op1: EncoderOperand,
    op2: EncoderOperand,
    op3: EncoderOperand,
    op4: EncoderOperand,
) -> EncoderInstruction {
    unsafe {
        let mut val: xed_encoder_instruction_t = std::mem::uninitialized();

        xed_inst5(
            &mut val as &mut _,
            mode.into_inner(),
            iclass.into(),
            effective_operand_width,
            op0.into_inner(),
            op1.into_inner(),
            op2.into_inner(),
            op3.into_inner(),
            op4.into_inner(),
        );

        val.into()
    }
}
