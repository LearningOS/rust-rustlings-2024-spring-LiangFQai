// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let mut x = 100;
    let y =  x;
    let z =  x;
    let x_ref = &mut x; // 获取 x 的可变引用
    *x_ref += y; // 修改 x 的值
    *x_ref += 1000; // 继续修改 x 的值
    assert_eq!(*x_ref, 1200); // 检查 x 的值是否为 1200
}

