fn insertion_sort_recursive(arr: &mut [usize], iters: &mut usize) {
    if arr.len() <= 1 {
        return;
    }
    *iters += 1;
    println!("iters: {iters}");
    let len = arr.len();
    insertion_sort_recursive(&mut arr[0..len - 1], iters);
    insert(arr, iters);
}

fn insert(arr: &mut [usize], iters: &mut usize) {
    println!("insert! {arr:?}, {iters:?}");
    let len = arr.len();
    let mut i = len - 2;
    let last = arr[len - 1];
    print!("last: {last}... ");
    while arr[i] > last {
        let val = arr[i];
        print!("{val}({i}) ");
        arr[i + 1] = arr[i];
        if i == 0 { break; }
        i -= 1;
    }
    println!();
    arr[i + 1] = last;
}

fn main() {
    let mut arr: [usize; 7] = [3, 2, 59, 34, 26, 11, 5];
    let mut iters: usize = 0;
    insertion_sort_recursive(&mut arr, &mut iters);
    println!("sorted: {arr:?}");
}