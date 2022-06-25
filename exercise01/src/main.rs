fn main() {
    //     Bài tập 1: Cho 2 mảng, kiểm tra mảng này có phải
    // là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
    //     Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
    //             let sub_arr = [6,8,10];

    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    let mut i = 0;
    let mut j = 0;

    if org_arr.len() < sub_arr.len() {
        print!("false");
    }

    while i < org_arr.len() && j < sub_arr.len() {
        // i = 0, j = 0
        if org_arr[i] == sub_arr[j] {
            i += 1;
            j += 1;

            if j == sub_arr.len() {
                print!("true");
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
}
