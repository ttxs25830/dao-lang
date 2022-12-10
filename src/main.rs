use std::env;

mod ats;

fn get_path() -> String {
    env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from(""))
        .to_owned()
}
fn main() {
    let path = get_path();
    if path != "" {
        // 读取文件
    } else {
        // 进入交互模式
    }
}
