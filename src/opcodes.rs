use crate::emu::*;
use crate::helper::*;
// NOxBDIZC
//access
fn immediate(nes: &mut Nes) -> u8 {
    nes.nextOp() 
}

fn zero_page(nes: &mut Nes) ->u8{
    nes.nextOp() 
}
fn zero_page_x(nes: &mut Nes) ->u8{
    (nes.nextOp() +nes.x) 
}
fn zero_page_y(nes: &mut Nes) -> u8{
    (nes.nextOp() + nes.y) as u8
}
fn absolute(nes: &mut Nes) -> u16{
    nes.nextabs()

}
fn absolute_x(nes: &mut Nes) -> u16{
    let address = nes.nextabs();
    if page_crossed(address, nes.x) {
        nes.page_cross = 1;
    }
    else {
        nes.page_cross = 0;
    }
    (address + nes.x as u16)
}
fn absolute_y(nes: &mut Nes) -> u16{
    let address = nes.nextabs();
    if page_crossed(address, nes.y) {
        nes.page_cross = 1;
    }
    else {
        nes.page_cross = 0;
    }
    (address + nes.y as u16)
}
fn indexed_indirect_x(nes: &mut Nes) -> u16{
    let add = nes.nextOp() + nes.x;
    let bit1 = nes.read_memory(add as u16);
    let bit2 = nes.read_memory((add+1) as u16);
    endian(bit1, bit2) as u16
}
fn indirect_indexed_y(nes: &mut Nes) -> u16{
    let add = nes.nextOp();
    let bit1 = nes.read_memory(add as u16);
    let bit2 = nes.read_memory((add+1) as u16);
    let address = endian(bit1,bit2);
    if page_crossed(address, nes.y) {
        nes.page_cross = 1;
    }
    else {
        nes.page_cross = 0;
    }
    (endian(bit1, bit2) + nes.y as u16) as u16

}
fn relative(nes: &mut Nes) -> u16 {
    ((nes.nextOp() as i8) as i16) as u16
}
fn indirect(nes:&mut Nes) -> u16 {
    let address = nes.nextabs();
    let bit1 = nes.read_memory(address);
    let mut offset = 0;
    if page_crossed(address as u16, 1){
        offset = 0x100;
    }
    let bit2 = nes.read_memory(address+1 - offset);
    let ind = endian(bit1, bit2);

    //eprintln!("{:#x} found at address {:#x}", ind, address);
    ind
}




// adc add with carry
fn add_with_carry(nes : &mut Nes, b:u8){
    let carry = get_bit(nes.p, 0);
    let a = nes.acc;
    nes.reset_flags(0b11000011);
    let c = a+b+carry;
    
    if c < a {
        //carry 
        nes.set_flags(1);

    }
    if c == 0 {
        // zero
        nes.set_flags(0b10);
    }
    if is_negative(c) {
        // negative 
       nes.set_flags(0b10000000); 

        if !is_negative(a) && !is_negative(b) {
            // overflow
            nes.set_flags(0b1000000);
        }
    }
    else if is_negative(a) && is_negative(b) {
            // overflow
            nes.set_flags(0b1000000);
    }
    nes.acc = c;   
   
}
//immediate
fn op69(nes : &mut Nes ) -> u8{
    let immediate = immediate(nes);
    add_with_carry(nes, immediate);
    2
}
//zeropage
fn op65(nes : &mut Nes)-> u8{
    let address = zero_page(nes);
    let mem_value = nes.read_memory(address as u16);
    add_with_carry(nes,mem_value); ;
    3
}
//zeropagex
fn op75(nes: &mut Nes) -> u8{
    let address = zero_page_x(nes);
    let mem_value = nes.read_memory(address as u16);
    add_with_carry(nes,mem_value); 
    4
}
// absolute 
fn op6D(nes: &mut Nes) -> u8{
    let address = absolute(nes);
    let mem_value = nes.read_memory(address as u16);
    add_with_carry(nes,mem_value); 
    4
}
// absolute x
fn op7D(nes: &mut Nes) -> u8{
    let address = absolute_x(nes);
    let b = nes.read_memory(address as u16);
    add_with_carry(nes, b);
    4 + nes.page_cross
    
}
//absolute y 
fn op79(nes: &mut Nes) -> u8{
    let address = absolute_y(nes);
    let b = nes.read_memory(address as u16);
    add_with_carry(nes, b);
    4 + nes.page_cross

}
//index_inderect x
fn op61(nes: &mut Nes) -> u8{
    let address = indexed_indirect_x(nes);
    let a = nes.read_memory(address as u16);
    add_with_carry(nes, a);
    6

}
//inderect_index
fn op71(nes: &mut Nes) -> u8{
   let address = indirect_indexed_y(nes);
   let a = nes.read_memory(address as u16);
   add_with_carry(nes, a);
   5+ nes.page_cross

}


//logical and

fn logical_and(nes: &mut Nes,a:u8){
    let c = a & nes.acc;
    nes.reset_flags(0b10000010);
    if c == 0 { 
        nes.set_flags(0b10)
    }
    if get_bit(c, 7) == 1 {
        nes.set_flags(0b10000000);
    }
    nes.acc = c;
}
//imediate 
fn op29(nes: &mut Nes) -> u8{
    let immediate = immediate(nes);
    logical_and(nes, immediate);
    2
}
//zeropage
fn op25(nes: &mut Nes) -> u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    logical_and(nes, a);
    3
}
//zeropagex
fn op35(nes: &mut Nes)-> u8{
    let address = zero_page_x(nes);
    let a = nes.read_memory(address as u16);
    logical_and(nes, a);
    4
}
//absolute
fn op2D(nes: &mut Nes) -> u8{
    let address = absolute(nes);
    let a = nes.read_memory(address as u16);
    logical_and(nes, a);
    4
}
//absolute x
fn op3D(nes: &mut Nes) -> u8{
    let address = absolute_x(nes);
    let a = nes.read_memory(address as u16);
    logical_and(nes, a);
    4 + nes.page_cross
}
//absolute y 
fn op39(nes: &mut Nes) -> u8{
    let address = absolute_y(nes);
    let a = nes.read_memory(address as u16);
    logical_and(nes, a);
    4 + nes.page_cross
}
//index_inderect x
fn op21(nes: &mut Nes) -> u8{
    let address = indexed_indirect_x(nes);
    let a = nes.read_memory(address as u16);
    logical_and(nes, a);
    6
}
//inderect_index y
fn op31(nes: &mut Nes) -> u8{
    let address = indirect_indexed_y(nes);
    let a = nes.read_memory(address as u16);
    logical_and(nes, a);
    5 + nes.page_cross
}

//arithmatic shift left
fn arithmatic_shift_left(nes: &mut Nes, a:u8) -> u8{
    let c = a<<1;
    nes.reset_flags(0b10000011);
    let old_bit_7 = get_bit(a, 7);
    let new_bit_7 = get_bit(c, 7);
    if c == 0 {
        nes.set_flags(0b10);
    }
    if new_bit_7 == 1 {
        nes.set_flags(1<<7);
    }
    nes.set_flags(old_bit_7);

    c

}

//accumalator
fn op0A(nes: &mut Nes)-> u8{
    nes.acc = arithmatic_shift_left(nes, nes.acc);
    2
}
// zero page
fn op06(nes: &mut Nes)->u8{
    let address = zero_page(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = arithmatic_shift_left(nes, mem_value);
    nes.write_memory(address as u16 ,value);
    5

}
//zero page x
fn op16(nes: &mut Nes)->u8{
    let address = zero_page_x(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = arithmatic_shift_left(nes, mem_value);
    nes.write_memory(address as u16 ,value);
    6
}
//absolute
fn op0E(nes: &mut Nes)->u8{
    let address = absolute(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = arithmatic_shift_left(nes, mem_value);
    nes.write_memory(address as u16 ,value);
    6
}
//absolute x
fn op1E(nes: &mut Nes)-> u8{
    let address = absolute_x(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = arithmatic_shift_left(nes, mem_value);
    nes.write_memory(address as u16 ,value);
    7
}

// branch cycles
fn branch (nes:&mut Nes, jump:u16) -> u8{
    let a = nes.pc;
    nes.pc += jump;
    if page_crossed(a, jump as u8) {
        return 5;
    }
    3
}
// branch if carry clear
fn op90(nes: &mut Nes) -> u8 {
    let jump = relative(nes);
    let carry = get_bit(nes.p, 0);
    if carry == 0 {
        return branch(nes, jump);
    }
    2
}
//branch if carry set
fn opB0(nes: &mut Nes)-> u8{
    let carry = get_bit(nes.p, 0);
    let jump = relative(nes);
    if carry == 1{
       return branch(nes, jump);
    }
    2
}
//branch if equal
fn opF0(nes: &mut Nes) -> u8{
    let zero = get_bit(nes.p, 1);
    let jump = relative(nes);
    if zero == 1 {
        return branch(nes, jump);
    }
    2
}
// bit test
fn bit_test(nes: &mut Nes, a: u8){
    nes.reset_flags(0b11000010);
    nes.set_flags(a & 0b11000000);
    if a & nes.acc == 0 {
        nes.set_flags(0b10);
    }
}
//zero page 
fn op24(nes: &mut Nes)-> u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    bit_test(nes, a);
    3

}

//absolute 
fn op2C(nes: &mut Nes) -> u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    bit_test(nes, a);
    4
}

// branch if minus 

fn op30(nes: &mut Nes) -> u8{
    let negative = get_bit(nes.p, 7);
    let jump = relative(nes);
    if negative == 1 {
        return branch(nes, jump);
    }
    2
}

//branch if not equal 
fn opD0(nes: &mut Nes) -> u8{
    let zero = get_bit(nes.p, 1);
    let jump = relative(nes);
    if zero == 0 {
        return branch(nes, jump);
    }
    2
}

//branch if positive 
fn op10(nes: &mut Nes) -> u8{
    let negative = get_bit(nes.p, 7);
    let jump = relative(nes);
    if negative == 0 {
        return branch(nes, jump);
    }
    2

}

//break
fn op00(nes: &mut Nes) -> u8{
    let bytes = unendian(nes.pc);
    nes.push_to_stack(bytes[1]);
    nes.push_to_stack(bytes[0]);
    nes.push_to_stack(nes.p);
    let byte1 = nes.read_memory(0xFFFE);
    let byte2 = nes.read_memory(0xFFFF);
    nes.pc = endian(byte1, byte2);
    nes.set_flags(0b10000);
    7
}


//branch if overflow clear 
fn op50(nes: &mut Nes) -> u8{
    let overflow = get_bit(nes.p, 6);
    let jump = relative(nes);
    if overflow == 0{
        return branch(nes, jump);
    }
    2
}
//branch if overflow set 
fn op70(nes: &mut Nes) -> u8{
    let overflow = get_bit(nes.p, 6);
    let jump = relative(nes);
    if overflow == 1{
        return branch(nes, jump);
    }
    2
}

//clear carry flaf 
fn op18(nes: &mut Nes) -> u8{
    nes.reset_flags(0b1);
    2
}
//clear decimal mode 
fn opD8(nes: &mut Nes) -> u8{
    nes.reset_flags(0b1000);
    2
}

//clear interupt disable 
fn op58(nes: &mut Nes) -> u8{
    nes.reset_flags(0b100);
    2
}

//clear overflow flag 
fn opB8(nes: &mut Nes) -> u8{
    nes.reset_flags(0b1000000);
    2
}

// compare 
fn compare(nes: &mut Nes, a:u8, b:u8){
    nes.reset_flags(0b10000011);
    let c = a-b;
    if a>=b {
        nes.set_flags(1);
    }
    if c == 0 {
        nes.set_flags(0b10);
    }
    if (c as i8) < 0 {
        nes.set_flags(0b10000000);
    }
}
fn compare_acc(nes: &mut Nes, a:u8){
    compare(nes, nes.acc, a);
}
fn compare_x(nes: &mut Nes,a:u8){
    compare(nes, nes.x, a);
}
fn compare_y(nes: &mut Nes,a:u8){
    compare(nes, nes.y, a);
}
//cmp acc 

//immediate 
fn opC9(nes: &mut Nes)->u8{
    let immediate = immediate(nes);
    compare_acc(nes, immediate);
    2
}

//zeropage 
fn opC5(nes: &mut Nes)->u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    compare_acc(nes, a);
    3
}

//zeropagex 
fn opD5(nes: &mut Nes)->u8{
    let address = zero_page_x(nes);
    let a = nes.read_memory(address as u16);
    compare_acc(nes, a);
    4

}
//absolute
fn opCD(nes: &mut Nes)->u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    compare_acc(nes, a);
    4
}

//absolute x
fn opDD(nes: &mut Nes) -> u8{
    let address = absolute_x(nes);
    let a = nes.read_memory(address);
    compare_acc(nes, a);
    4+nes.page_cross
}

//absolute_y
fn opD9(nes: &mut Nes) -> u8{
    let address = absolute_y(nes);
    let a = nes.read_memory(address);
    compare_acc(nes, a);
    4+nes.page_cross
}
//index_inderect 
fn opC1(nes: &mut Nes)-> u8{
    let address = indexed_indirect_x(nes);
    let a = nes.read_memory(address);
    compare_acc(nes, a);
    6
}

//inderect_index
fn opD1(nes: &mut Nes) -> u8{
    let address = indirect_indexed_y(nes);
    let a = nes.read_memory(address);
    compare_acc(nes, a);
    5+nes.page_cross
}

//cmp x
//immediate 
fn opE0(nes: &mut Nes)->u8{
    let a = immediate(nes);
    compare_x(nes, a);
    2
}
//zeropage
fn opE4(nes: &mut Nes)->u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    compare_x(nes, a);
    3
}
//absolute 
fn opEC(nes: &mut Nes)->u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    compare_x(nes, a);
    4
}

//cmp y
////immediate
fn opC0(nes: &mut Nes)->u8{
    let a = immediate(nes);
    compare_y(nes, a);
    2
}
//zeropage
fn opC4(nes: &mut Nes)->u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    compare_y(nes, a);
    3
}
//absolute 
fn opCC(nes: &mut Nes)->u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    compare_y(nes, a);
    4
}

//dec memory 

fn decrement_mem(nes: &mut Nes,address: u16){
    let c = nes.read_memory(address as u16) -1;
    nes.reset_flags(0b10000010);
    if (c as i8)< 0 {
        nes.set_flags(0b10000000);
    }
    if c == 0 {
        nes.set_flags(0b10);
    }
    nes.write_memory(address as u16,c);
}

//zeropage 
fn opC6(nes: &mut Nes)->u8{
    let address = zero_page(nes);
    decrement_mem(nes, address as u16);
    5
}
//zeropage x
fn opD6(nes: &mut Nes)->u8{
    let address = zero_page_x(nes);
    decrement_mem(nes, address as u16);
    6
}
//absolute
fn opCE(nes: &mut Nes)->u8{
    let address = absolute(nes);
    decrement_mem(nes, address);
    6
}
//absolute x 
fn opDE(nes: &mut Nes)->u8{
    let address = absolute_x(nes);
    decrement_mem(nes, address);
    7
}

//dec x register 
fn opCA(nes: &mut Nes) -> u8{
    nes.reset_flags(0b10000010);
    let c = nes.x -1;
    if c == 0 {
        nes.set_flags(0b10);
    }
    if (c as i8) < 0{
        nes.set_flags(0b10000000);
    }
    nes.x = c;
    2
}

//dec y register 
fn op88(nes: &mut Nes) -> u8{
    nes.reset_flags(0b10000010);
    let c = nes.y -1;
    if c == 0 {
        nes.set_flags(0b10);
    }
    if (c as i8) < 0{
        nes.set_flags(0b10000000);
    }
    nes.y = c;
    2
}

//exclusive or 
fn eor(nes: &mut Nes, a:u8){
    nes.reset_flags(0b10000010);
    let c = nes.acc^a;
    if c == 0{
        nes.set_flags(0b10);
    }
    if (c as i8) < 0 {
        nes.set_flags(0b10000000);
    }
    nes.acc = c;
}
//immediate 
fn op49(nes: &mut Nes) -> u8{
    let a = immediate(nes);
    eor(nes, a);
    2
}
//zeropage 
fn op45(nes: &mut Nes) -> u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    eor(nes, a);
    3
}

//zeropagex
fn op55(nes: &mut Nes)->u8{
    let address = zero_page_x(nes);
    let a = nes.read_memory(address as u16);
    eor(nes, a);
    4
}
//absolute 
fn op4D(nes: &mut Nes) -> u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    eor(nes,a);
    4
}
//absolute x 
fn op5D(nes: &mut Nes) -> u8{
    let address = absolute_x(nes);
    let a = nes.read_memory(address);
    eor(nes,a);
    4 + nes.page_cross
}
//absolute_y
fn op59(nes: &mut Nes) -> u8{
    let address = absolute_y(nes);
    let a = nes.read_memory(address);
    eor(nes,a);
    4 + nes.page_cross
}
//index_inderect
fn op41(nes: &mut Nes)->u8{
    let address = indexed_indirect_x(nes);
    let a = nes.read_memory(address);
    eor(nes, a);
    6
}
//inderect_index 
fn op51(nes:&mut Nes) -> u8{
    let address = indirect_indexed_y(nes);
    let a = nes.read_memory(address);
    eor(nes,a);
    5 + nes.page_cross
}

//increment memory 
fn increment_mem(nes:&mut Nes, address: u16){
    nes.reset_flags(0b10000010);
    let c = nes.read_memory(address as u16)+1;
    //eprintln!("incrementing {:#x} at address {:#x}", c-1 , address);
    if c == 0 {
        nes.set_flags(0b10);
    }
    if (c as i8) < 0 {
        nes.set_flags(0b10000000);

    }
    nes.write_memory(address as u16, c);
}
//zeropage
fn opE6(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    increment_mem(nes, address as u16);
    5
}

//zeropagex 
fn opF6(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    increment_mem(nes, address as u16);
    6
}

//absolute 
fn opEE(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    increment_mem(nes, address);
    6
}
//absolute x 
fn opFE(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    increment_mem(nes, address);
    7
}
//inc x
fn opE8(nes:&mut Nes) -> u8{
    nes.reset_flags(0b10000010);
    let c = nes.x +1;
        if c == 0 {
        nes.set_flags(0b10);
    }
    if (c as i8) < 0 {
        nes.set_flags(0b10000000);

    }
    nes.x = c;
    2

}
//inc y
fn opC8(nes:&mut Nes) -> u8{
    nes.reset_flags(0b10000010);
    let c = nes.y +1;
        if c == 0 {
        nes.set_flags(0b10);
    }
    if (c as i8) < 0 {
        nes.set_flags(0b10000000);

    }
    nes.y = c;
    2

}
// jump 

//absolute 
fn op4C(nes:&mut Nes)-> u8{
    let address = absolute(nes);
    nes.pc = address as u16;
    3
}
//indirect 
fn op6C(nes:&mut Nes) -> u8{
    nes.pc = indirect(nes);
    5
}

//jump to subroutine 
fn op20(nes:&mut Nes) -> u8{
    let address = absolute(nes) as u16;
    let bytes = unendian(nes.pc -1);
    nes.push_to_stack(bytes[1]);
    nes.push_to_stack(bytes[0]);
    nes.pc = address;
    6
}

//load accumalator

fn load_acc(nes:&mut Nes,a:u8){
    nes.acc = a;
    nes.reset_flags(0b10000010);
    if a == 0 { 
        nes.set_flags(0b10);
    }
    if (a as i8) < 0 {
        nes.set_flags(0b10000000);
    }

}
//imediate
fn opA9(nes:&mut Nes)->u8{
    let a = immediate(nes);
    load_acc(nes, a);
    2
}
//zero page 
fn opA5(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    load_acc(nes, a);
    3
}
//zero page x 
fn opB5(nes:&mut Nes)->u8{
    let address = zero_page_x(nes);
    let a = nes.read_memory(address as u16);
    load_acc(nes, a);
    4
}
//absolute 
fn opAD(nes:&mut Nes)->u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    load_acc(nes,a);
    4
}
//absolute_x
fn opBD(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    let a = nes.read_memory(address);
    load_acc(nes, a);
    4+nes.page_cross
}
//absolute_y
fn opB9(nes:&mut Nes) -> u8{
    let address = absolute_y(nes);
    let a = nes.read_memory(address);
    load_acc(nes, a);
    4 + nes.page_cross
}
//indexed_indirect_x
fn opA1(nes:&mut Nes) -> u8{
    let address = indexed_indirect_x(nes);
    let a = nes.read_memory(address);
    load_acc(nes, a);
    6
}
//indirect_indexed_y
fn opB1(nes:&mut Nes) -> u8{
    let address = indirect_indexed_y(nes);
    let a = nes.read_memory(address);
    load_acc(nes, a);
    5 + nes.page_cross
}

//load x
fn load_x(nes:&mut Nes,a:u8){
    nes.x = a;
    nes.reset_flags(0b10000010);
    if a == 0 { 
        nes.set_flags(0b10);
    }
    if (a as i8) < 0 {
        nes.set_flags(0b10000000);
    }

}

//immediate
fn opA2(nes:&mut Nes) -> u8{
    let a = immediate(nes);
    load_x(nes, a);
    2
}
//zeropage
fn opA6(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    load_x(nes, a);
    3
}
//zero_page_y
fn opB6(nes:&mut Nes) -> u8{
    let address = zero_page_y(nes);
    let a = nes.read_memory(address as u16);
    load_x(nes, a);
    4
}
//absolute
fn opAE(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    load_x(nes, a);
    4
}
//absolute_y
fn opBE(nes:&mut Nes) -> u8{
    let address = absolute_y(nes);
    let a = nes.read_memory(address);
    load_x(nes, a);
    4 + nes.page_cross
}
//load y
fn load_y(nes:&mut Nes,a:u8){
    nes.y = a;
    nes.reset_flags(0b10000010);
    if a == 0 { 
        nes.set_flags(0b10);
    }
    if is_negative(a) {
        nes.set_flags(0b10000000);
    }

}

//immediate
fn opA0(nes:&mut Nes) -> u8{
    let a = immediate(nes);
    load_y(nes, a);
    2
}
//zeropage
fn opA4(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    load_y(nes, a);
    3
}
//zero_page_x
fn opB4(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    let a = nes.read_memory(address as u16);
    load_y(nes, a);
    4
}
//absolute
fn opAC(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    load_y(nes, a);
    4
}
//absolute_x
fn opBC(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    let a = nes.read_memory(address);
    load_y(nes, a);
    4 + nes.page_cross
}

// logical shift right
fn logical_shift_right(nes:&mut Nes, a:u8) -> u8{
    nes.reset_flags(0b10000011);
    let old_bit = a & 1;
    let c = a>>1;
    nes.set_flags(old_bit);

    if c == 0 {
        nes.set_flags(0b10);
    }
    if(c as i8) < 0 {
        nes.set_flags(0b10000000);
    }

    c
}
//accumalator
fn op4A(nes:&mut Nes) -> u8{
    nes.acc = logical_shift_right(nes, nes.acc);
    2
}
//zeropage
fn op46(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = logical_shift_right(nes, mem_value);
    nes.write_memory(address as u16,value);
    5
}
//zero_page_x
fn op56(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = logical_shift_right(nes, mem_value);
    nes.write_memory(address as u16,value);
    6
}
//absolute
fn op4E(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = logical_shift_right(nes, mem_value);
    nes.write_memory(address as u16,value);
    6
}
//absolute x
fn op5E(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = logical_shift_right(nes, mem_value);
    nes.write_memory(address as u16,value);
    7
}
// nop 
fn opEA(nes:&mut Nes) -> u8{
    2
}
// or 
fn logical_or(nes:&mut Nes,a:u8){
    let c = nes.acc | a;
    nes.reset_flags(0b10000010);
    if c == 0 {
        nes.set_flags(0b10);
    }
    if (c as i8) < 0 {
        nes.set_flags(0b10000000);
    }
    nes.acc = c;
}
//imediate
fn op09(nes:&mut Nes) -> u8{
    let a = immediate(nes);
    logical_or(nes, a);
    2
}
//zeropage
fn op05(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    logical_or(nes, a);
    3
}
//zero_page_x
fn op15(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    let a = nes.read_memory(address as u16);
    logical_or(nes, a);
    4
}
//absolute
fn op0D(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    logical_or(nes, a);
    4
}
//absolute_x
fn op1D(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    let a = nes.read_memory(address);
    logical_or(nes, a);
    4 + nes.page_cross
}
//absolute_y
fn op19(nes:&mut Nes) -> u8{
    let address = absolute_y(nes);
    let a = nes.read_memory(address);
    logical_or(nes, a);
    4 + nes.page_cross
}
//indexed_indirect_x
fn op01(nes:&mut Nes) -> u8{
    let address = indexed_indirect_x(nes);
    let a = nes.read_memory(address);
    logical_or(nes, a);
    6
}
//indirect_indexed_y
fn op11(nes:&mut Nes) -> u8{
    let address = indirect_indexed_y(nes);
    let a = nes.read_memory(address);
    logical_or(nes, a);
    5 + nes.page_cross
}
//push accumalator 
fn op48(nes:&mut Nes) -> u8{
    nes.push_to_stack(nes.acc);
    3

}
//push processor status
fn op08(nes:&mut Nes) -> u8{
    nes.push_to_stack(nes.p);
    3
}
//pull accumulator
fn op68(nes:&mut Nes) -> u8{
    nes.acc = nes.pop_from_stack();
    nes.reset_flags(0b10000010);
    if nes.acc == 0 {
        nes.set_flags(0b10);
    }
    if (nes.acc as i8)<0 {
        nes.set_flags(0b10000000);
    }
    4
}
//pull processor status
fn op28(nes:&mut Nes) -> u8{
    nes.p = (nes.pop_from_stack() & 0xEF) | 0b00100000;
    4
}
//rotate left
fn rotate_left(nes:&mut Nes, a:u8)-> u8{
    let old_carry = get_bit(nes.p,0);
    let new_carry = get_bit(a, 7); 
    nes.reset_flags(0b10000011);
    let mut c = a<<1;
    c = c | (old_carry);
    nes.set_flags(new_carry);
    if is_negative(c) {
        nes.set_flags(0b10000000);
    }
    if c == 0 {
        nes.set_flags(0b10);
    }
    c

}

//accumulator
fn op2A(nes:&mut Nes) -> u8{
    nes.acc = rotate_left(nes, nes.acc);
    2
}
//zero_page
fn op26(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = rotate_left(nes, mem_value);
    nes.write_memory(address as u16, value);
    5
}
//zero_page_x
fn op36(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = rotate_left(nes, mem_value);
    nes.write_memory(address as u16, value);
    6
}
//absolute
fn op2E(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = rotate_left(nes, mem_value);
    nes.write_memory(address as u16, value);
    6
}
//absolute x
fn op3E(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = rotate_left(nes, mem_value);
    nes.write_memory(address as u16, value);
    7
}
//rotate right
fn rotate_right(nes:&mut Nes, a:u8)-> u8{
    let old_carry = get_bit(nes.p,0);
    let new_carry = get_bit(a, 0); 
    nes.reset_flags(0b10000011);
    let mut c = a>>1;
    c = c | (old_carry<<7);
    nes.set_flags(new_carry);
    if is_negative(c) {
        nes.set_flags(0b10000000);
    }
    if c == 0 {
        nes.set_flags(0b10);
    }
    c

}
// accumulator
fn op6A(nes:&mut Nes) -> u8{
    nes.acc = rotate_right(nes, nes.acc);
    2

}
//zeropage
fn op66(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = rotate_right(nes, mem_value);
    nes.write_memory(address as u16, value);
    5

}
//zeropagex
fn op76(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = rotate_right(nes, mem_value);
    nes.write_memory(address as u16, value);
    6

}
//absolute
fn op6E(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = rotate_right(nes, mem_value);
    nes.write_memory(address as u16, value);
    6

}
//absolute x
fn op7E(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    let mem_value = nes.read_memory(address as u16);
    let value = rotate_right(nes, mem_value);
    nes.write_memory(address as u16, value);
    7

}

//return from interupt
fn op40(nes:&mut Nes) -> u8{
   nes.p = (nes.pop_from_stack() & 0xEF) | 0b00100000;
   let byte1 = nes.pop_from_stack();
   let byte2 = nes.pop_from_stack();
   nes.pc = endian(byte1, byte2);
   6
}
//return from subroutine 
fn op60(nes:&mut Nes) -> u8{
    let byte1 = nes.pop_from_stack();
    let byte2 = nes.pop_from_stack();
    nes.pc = endian(byte1, byte2) + 1;
    6
}
//subtract with carry 
fn subtract_with_carry(nes:&mut Nes, a:u8){
    let carry = get_bit(nes.p, 0);
    let c = nes.acc - a - (1-carry);
    nes.reset_flags(0b11000000);
    nes.set_flags(0b1);
    if c == 0 {
        nes.set_flags(0b10);
    }
    if is_negative(c) {
        nes.set_flags(0b10000000);
    }

    if is_negative(a) && !is_negative(nes.acc) && is_negative(c) {
        nes.set_flags(0b1000000);
    }
    if !is_negative(a) && is_negative(nes.acc) && !is_negative(c) {
        nes.set_flags(0b1000000);
    }
    let acc1 = nes.acc as u16;
    let a1 = a as u16;
    let c1 = acc1 - a1 -(1-carry as u16); 
    if c1 > 255 {
        nes.reset_flags(0b1);
    }
    nes.acc = c;
}
//imediate
fn opE9(nes:&mut Nes) -> u8{
    let a = immediate(nes);
    subtract_with_carry(nes, a);
    2
}
//zeropage
fn opE5(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    let a = nes.read_memory(address as u16);
    subtract_with_carry(nes, a);
    3
}
//zeropagex
fn opF5(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    let a = nes.read_memory(address as u16);
    subtract_with_carry(nes, a);
    4

}

//absolute
fn opED(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    let a = nes.read_memory(address);
    subtract_with_carry(nes, a);
    4
}
//absolute x
fn opFD(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    let a = nes.read_memory(address);
    subtract_with_carry(nes, a);
    4 + nes.page_cross

}
//absolute y
fn opF9(nes:&mut Nes) -> u8{
    let address = absolute_y(nes);
    let a = nes.read_memory(address);
    subtract_with_carry(nes, a);
    4 + nes.page_cross

}
//indexed_indirect_x
fn opE1(nes:&mut Nes) -> u8{
    let address = indexed_indirect_x(nes);
    let a = nes.read_memory(address);
    subtract_with_carry(nes, a);
    6
}
//indirect_indexed_y
fn opF1(nes:&mut Nes) -> u8{
    let address = indirect_indexed_y(nes);
    let a = nes.read_memory(address);
    subtract_with_carry(nes, a);
    5 + nes.page_cross
}
// set carry flag
fn op38(nes:&mut Nes) -> u8{
    nes.set_flags(0b1);
    2
}
// set decimal flag
fn opF8(nes:&mut Nes) -> u8{
    nes.set_flags(0b1000);
    2
}

//set interrupt disable 
fn op78(nes:&mut Nes) -> u8{
    nes.set_flags(0b100);
    2
}
//store accumulator
fn store_acc(nes:&mut Nes,address: u16){
    //eprintln!("storing {:#x} at address {:#x}",nes.acc,address);
    nes.write_memory(address, nes.acc);
}
//zeropage
fn op85(nes:&mut Nes) -> u8{
   let address = zero_page(nes); 
    store_acc(nes, address as u16);
    3
}
//zeropagex
fn op95(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    store_acc(nes, address as u16);
    4
}
//absolute
fn op8D(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    store_acc(nes, address);
    4
}
//absolute_x
fn op9D(nes:&mut Nes) -> u8{
    let address = absolute_x(nes);
    store_acc(nes, address);
    5
}
//absolute_y
fn op99(nes:&mut Nes) -> u8{
    let address = absolute_y(nes);
    store_acc(nes, address);
    5
}
//indexed_indirect_x
fn op81(nes:&mut Nes) -> u8{
    let address = indexed_indirect_x(nes);
    store_acc(nes, address);
    6
}
//indirect_indexed_y
fn op91(nes:&mut Nes) -> u8{
    let address = indirect_indexed_y(nes);
    store_acc(nes, address);
    6
}
//store x
fn store_x(nes:&mut Nes,address: u16){
    nes.write_memory(address,nes.x);

}
//zeropage
fn op86(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    store_x(nes, address as u16);
    3
}
//zero_page_y
fn op96(nes:&mut Nes) -> u8{
    let address = zero_page_y(nes);
    store_x(nes, address as u16);
    4
}
//absolute
fn op8E(nes:&mut Nes) -> u8{
    let address = absolute(nes);
    store_x(nes, address);
    4
}
//store y
fn store_y(nes:&mut Nes,address: u16){
    nes.write_memory(address,nes.y);

}
//zeropage
fn op84(nes:&mut Nes) -> u8{
    let address = zero_page(nes);
    store_y(nes, address as u16);
    3
}
//zero_page_x
fn op94(nes:&mut Nes) -> u8{
    let address = zero_page_x(nes);
    store_y(nes, address as u16);
    4
}
//absolute
fn op8C(nes:&mut Nes)-> u8{
    let address = absolute(nes);
    store_y(nes, address);
    4
}

// transfer registers
fn transfer_flags(nes:&mut Nes, a:u8){
    nes.reset_flags(0b10000010);
    if a == 0 {
        nes.set_flags(0b10);
    }
    if is_negative(a) {
        nes.set_flags(0b10000000);
    }
}

//transfer acc to x
fn opAA(nes:&mut Nes) -> u8{
    nes.x = nes.acc;
    transfer_flags(nes, nes.x);
    2
}
//transfer acc to y
fn opA8(nes:&mut Nes) -> u8{
    nes.y = nes.acc;
    transfer_flags(nes, nes.y);
    2
}

//transfer sp to x
fn opBA(nes:&mut Nes) -> u8{
    nes.x = nes.sp;
    transfer_flags(nes, nes.x);
    2
}
// transfer x to acc
fn op8A(nes:&mut Nes) -> u8{
    nes.acc = nes.x;
    transfer_flags(nes, nes.acc);
    2
}
// transfer x to sp
fn op9A(nes:&mut Nes) -> u8{
    nes.sp = nes.x ;
    2
}
//transfer y to acc 
fn op98(nes:&mut Nes) -> u8{
    nes.acc = nes.y;
    transfer_flags(nes, nes.acc);
    2
    
}

// unoficial op codes 
// unoficial nops 
fn op04(nes:&mut Nes) -> u8{
    nes.nextOp();
    3
}
fn op44(nes:&mut Nes) -> u8{
    nes.nextOp();
    3
}
fn op64(nes:&mut Nes) -> u8{
    nes.nextOp();
    3
}
fn op0C(nes:&mut Nes) -> u8{
    nes.nextabs();
    4
}
fn op14(nes:&mut Nes) -> u8{
    nes.nextOp();
    4
}
fn op34(nes:&mut Nes) -> u8{
    nes.nextOp();
    4
}
fn op54(nes:&mut Nes) -> u8{
    nes.nextOp();
    4
}
fn op74(nes:&mut Nes) -> u8{
    nes.nextOp();
    4
}
fn opD4(nes:&mut Nes) -> u8{
    nes.nextOp();
    4
}
fn opF4(nes:&mut Nes) -> u8{
    nes.nextOp();
    4
}
fn op80(nes:&mut Nes) -> u8{
    nes.nextOp();
    2
}
fn op1C(nes:&mut Nes) -> u8{
    nes.nextabs();
    5
}



pub fn run_op_code(nes:&mut Nes, op:u8) -> u8{
    //print!("op:{:#x} \t",op);
   let cycles =  match op {
        0x69 => op69(nes),    0x65 => op65(nes),    0x75 => op75(nes),    0x6D => op6D(nes),    
        0x79 => op79(nes),    0x61 => op61(nes),    0x71 => op71(nes),    0x29 => op29(nes),    
        0x25 => op25(nes),    0x35 => op35(nes),    0x2D => op2D(nes),    0x3D => op3D(nes),    
        0x39 => op39(nes),    0x21 => op21(nes),    0x31 => op31(nes),    0x0A => op0A(nes),    
        0x06 => op06(nes),    0x16 => op16(nes),    0x0E => op0E(nes),    0x1E => op1E(nes),    
        0x90 => op90(nes),    0xB0 => opB0(nes),    0xF0 => opF0(nes),    0x24 => op24(nes),    
        0x2C => op2C(nes),    0x30 => op30(nes),    0xD0 => opD0(nes),    0x10 => op10(nes),    
        0x00 => op00(nes),    0x50 => op50(nes),    0x70 => op70(nes),    0x18 => op18(nes),    
        0xD8 => opD8(nes),    0x58 => op58(nes),    0xB8 => opB8(nes),    0xC9 => opC9(nes),    
        0xC5 => opC5(nes),    0xD5 => opD5(nes),    0xCD => opCD(nes),    0xDD => opDD(nes),    
        0xD9 => opD9(nes),    0xC1 => opC1(nes),    0xD1 => opD1(nes),    0xE0 => opE0(nes),    
        0xE4 => opE4(nes),    0xEC => opEC(nes),    0xC0 => opC0(nes),    0xC4 => opC4(nes),    
        0xCC => opCC(nes),    0xC6 => opC6(nes),    0xD6 => opD6(nes),    0xCE => opCE(nes),    
        0xDE => opDE(nes),    0xCA => opCA(nes),    0x88 => op88(nes),    0x49 => op49(nes),    
        0x45 => op45(nes),    0x55 => op55(nes),    0x4D => op4D(nes),    0x5D => op5D(nes),    
        0x59 => op59(nes),    0x41 => op41(nes),    0x51 => op51(nes),    0xE6 => opE6(nes),    
        0xF6 => opF6(nes),    0xEE => opEE(nes),    0xFE => opFE(nes),    0xE8 => opE8(nes),    
        0xC8 => opC8(nes),    0x4C => op4C(nes),    0x6C => op6C(nes),    0x20 => op20(nes),    
        0xA9 => opA9(nes),    0xA5 => opA5(nes),    0xB5 => opB5(nes),    0xAD => opAD(nes),    
        0xBD => opBD(nes),    0xB9 => opB9(nes),    0xA1 => opA1(nes),    0xB1 => opB1(nes),    
        0xA2 => opA2(nes),    0xA6 => opA6(nes),    0xB6 => opB6(nes),    0xAE => opAE(nes),    
        0xBE => opBE(nes),    0xA0 => opA0(nes),    0xA4 => opA4(nes),    0xB4 => opB4(nes),    
        0xAC => opAC(nes),    0xBC => opBC(nes),    0x4A => op4A(nes),    0x46 => op46(nes),    
        0x56 => op56(nes),    0x4E => op4E(nes),    0x5E => op5E(nes),    0xEA => opEA(nes),    
        0x09 => op09(nes),    0x05 => op05(nes),    0x15 => op15(nes),    0x0D => op0D(nes),    
        0x1D => op1D(nes),    0x19 => op19(nes),    0x01 => op01(nes),    0x11 => op11(nes),    
        0x48 => op48(nes),    0x08 => op08(nes),    0x68 => op68(nes),    0x28 => op28(nes),    
        0x2A => op2A(nes),    0x26 => op26(nes),    0x36 => op36(nes),    0x2E => op2E(nes),    
        0x3E => op3E(nes),    0x6A => op6A(nes),    0x66 => op66(nes),    0x76 => op76(nes),    
        0x6E => op6E(nes),    0x7E => op7E(nes),    0x40 => op40(nes),    0x60 => op60(nes),    
        0xE9 => opE9(nes),    0xE5 => opE5(nes),    0xF5 => opF5(nes),    0xED => opED(nes),    
        0xFD => opFD(nes),    0xF9 => opF9(nes),    0xE1 => opE1(nes),    0xF1 => opF1(nes),    
        0x38 => op38(nes),    0xF8 => opF8(nes),    0x78 => op78(nes),    0x85 => op85(nes),    
        0x95 => op95(nes),    0x8D => op8D(nes),    0x9D => op9D(nes),    0x99 => op99(nes),    
        0x81 => op81(nes),    0x91 => op91(nes),    0x86 => op86(nes),    0x96 => op96(nes),    
        0x8E => op8E(nes),    0x84 => op84(nes),    0x94 => op94(nes),    0x8C => op8C(nes),    
        0xAA => opAA(nes),    0xA8 => opA8(nes),    0xBA => opBA(nes),    0x8A => op8A(nes),    
        0x9A => op9A(nes),    0x98 => op98(nes),    0x7D => op7D(nes),    0x04 => op04(nes),
        0x44 => op44(nes),    0x64 => op64(nes),    0x0C => op0C(nes),    0x14 => op14(nes),
        0x34 => op34(nes),    0x54 => op54(nes),    0x74 => op74(nes),    0xD4 => opD4(nes),
        0xF4 => opF4(nes),    0x1A|0x3A|0x5A|0x7A|0xDA|0xFA => opEA(nes),
        0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => op80(nes),
        0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => op1C(nes),

        _ => panic!("op code {:#x} not found", op)

    };

   cycles
}
