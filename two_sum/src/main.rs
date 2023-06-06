
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    for (index1, element) in nums.iter().enumerate() {
        for (index2, element2) in nums.iter().enumerate() {
            if (index1 != index2) && (element + element2 == target) {
                vec.push(index1 as i32);
                vec.push(index2 as i32);
                return vec;
            }
        }
    }
    vec
}

fn main() {
    println!("Hello, world!");
}
