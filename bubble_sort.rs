use std::io::{self, Write};

fn main() {
    let mut tab = vec![12, -34, 12, 124, 3245, 9, 6];
    bubble_sort_i32(&mut tab);
    write_arr_i32(&tab);
}

fn write_arr_i32(arr: &[i32]) {
    print!("Tablica: ");
    io::stdout().flush().expect("Failed to flush");

    for i in 0..arr.len() {
        print!(" {} ", arr[i]);
        io::stdout().flush().expect("Failed to flush");
    }
}

fn bubble_sort_i32(arr: &mut [i32]) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}