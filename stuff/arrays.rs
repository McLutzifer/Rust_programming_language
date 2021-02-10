fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);

    println!("{}", add(42, 13));
}