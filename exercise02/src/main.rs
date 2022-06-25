use std::io::{stdin};
fn main() {
    // Bài tập 2 : Cho 1 chuỗi str Slice như dưới đây. Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho.
    // Nhập từ bàn phím : “is is” => In ra kết quả số lượng “is is” là 5
    println!("Input: ");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to read line");
    input = input.trim().to_string();


    println!("{:?}", input.len());
}
