fn main() {
    let address = 2;
    let value = 0;

    let mut op: u8 = 0b0110_0011;
    println!("{:08b}", op);
    op = op.rotate_right(address);
    println!("{:08b}", op);
    op = (op >> 1 << 1) + value;
    println!("{:08b}", op);
    op = op.rotate_left(address);
    println!("{:08b}", op)
}

