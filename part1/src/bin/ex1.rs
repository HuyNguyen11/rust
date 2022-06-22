fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    let mut result = true;

    let mut index = 0;
    for i in 0..sub_arr.len() {
        result = false;
        for j in index..org_arr.len() {
            if org_arr[j] == sub_arr[i] {
                result = true;
                index = j;
                break;
            }
        }
    }

    println!("Result is {}", result);
}
