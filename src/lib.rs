// 查找名为 `front_of_house.rs` 的文件，
// 并将该文件的内容放到一个名为 `front_of_house` 的模块里面。
mod front_of_house2;

//使用pub use 重新导出（Re-exports）
//重新导出后，不仅当前模块可以使用 `hosting` 模块，在当前模块之外也可以使用
pub use crate::front_of_house2::hosting2;

pub fn eat_at_restaurant2() {
    hosting2::add_to_waitlist();
    hosting2::add_to_waitlist();
}
pub fn print_test() {
    println!("dsadasdadsadas-------");
}