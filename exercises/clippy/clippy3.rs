// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let mut my_vec = vec![1, 2, 3, 4, 5]; // 声明向量为可变
    my_vec.clear(); // 使用 clear() 方法清空向量
    println!("{:?}", my_vec); // 打印清空后的向量
}

