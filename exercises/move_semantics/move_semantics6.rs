// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(data.clone()); // 克隆 data，使其保留所有权
    println!("Last char: {}", last_char);

    let mut data = data; // 获取 data 的所有权
    string_uppercase(&mut data); // 传递 data 的可变引用
}

// 不应该获取所有权，返回最后一个字符
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// 应该获取所有权，将字符串转换为大写
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase(); // 修改 data 的值为大写形式

    println!("{}", data);
}

