// As the name implies, unsigned integers (u8, u16, u32, u64) cannot be negative
#[test]
fn unsigned_ints() {
    assert_eq!(u8::MIN, 0);
}

// Unsigned integers can be reduced only as far as their minimum value of 0
#[test]
fn sub_unsigned_int() {
    let mut num: u8 = 10;
    num -= 10;
    assert!(num <= u8::MIN);
}

// Signed integers(i8, i16, i32, i64), on the other hand, can be negative
#[test]
fn signed_ints() {
    assert!(i8::min_value() < 0);
}

// Signed integers can be reduced below zero, as far as their minimum value.
// hint: The maximum magnitude for a negative integer is greater than
// that of a positive integer
#[test]
fn sub_signed_int() {
    let mut num: i8 = 0;
    let negative: i8 = -1;
    num += negative;
    assert!(num > i8::MIN);
}

// Addition of positive integers works much the same for signed and unsigned numbers
#[test]
fn add_numbers() {
    let mut sig: i8 = 0;
    sig += 127;
    println!("{} - {}", i8::MIN, i8::MAX);
    println!("{} - {}", u8::MIN, u8::MAX);
    let mut unsig: u8 = 0;
    unsig += 255;
    assert!(sig == i8::max_value() && unsig == u8::max_value());
}

// Like any variable in Rust, integers are immutable unless declared otherwise
#[test]
fn mutating_ints() {
    let mut num: i8 = 1;
    num += 2;
    assert!(num == 3);
}

// While regular immutable variables cannot be changed, mutable versions of them can be
#[test]
fn referencing_values() {
    let num: i8 = 1;
    let mut mut_num = num;
    mut_num += 1;
    assert!(num != mut_num);
}
