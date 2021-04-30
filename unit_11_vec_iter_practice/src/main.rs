fn main() {
    println!("Hello, world!");

   // let _ = running_sum(vec![41, 69, 420]);
    let result = drainenstien_sum(vec![1, 2, 3, 4, 5]);

    println!("{:?}", result);
}



// Given an vector nums. We define a running sum of an vector as runningSum[i] = sum(nums[0]…nums[i]).

// Return the running sum of nums.
// Example 1:

// Input: nums = [1,2,3,4]
// Output: [1,3,6,10]
// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
// Example 2:

// Input: nums = [1,1,1,1,1]
// Output: [1,2,3,4,5]
// Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].

// Example 3:

// Input: nums = [3,1,2,10,1]
// Output: [3,4,6,16,17]
fn running_sum(nums: Vec<i32>) -> Vec<i32> {  // [1, 2, 3, 4, 5]
    let mut sum = 0;
    let mut result = vec![];  // [1, 3]
    for x in nums.iter() { // x = 2
        sum += x; // sum = sum + x;
        result.push(sum);
    }

    result
}


fn running_sum_idiomatic(nums: Vec<i32>) -> Vec<i32> {
    nums  // [1, 2, 3, 4, 5]
        .iter()  // iter, allows us to Iterate over our nums
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect::<Vec<i32>>() // collect is a Consumer, in this case it consumes
        // and "collects" your elements into a new type, in this case, we 
        // are collecting into a Vec<i32>
}
// acc = 0, x = 1        return next() -> 1
// ->  acc = 1, x = 2    return next() -> 3
// -> acc = 3, x = 3     return next() -> 6
// -> acc = 6, x = 4     return next() -> 10
// -> acc = 10, x = 5    return next() -> 15

// a Reducer, is simillar to the .map(), where map maps from a -> b
// a reducer will get (a1, a2, ... alast) -> accumulator


fn drainenstien_sum(mut nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    for i in 0..nums.len() {  // i = 2, x = 3
        let mut drained_numbers: Vec<_> =  nums.drain(0..i+1).collect(); // [1],

        let final_sum: i32 = drained_numbers.iter().sum(); // 6
        result.push(final_sum);

        &drained_numbers.append(&mut nums); // [1, 3, 3, 4]
        nums = drained_numbers;        
    }   
    result

}

// [1, 2, 3]
fn sum(mut nums: Vec<i32>) -> i32 {
    nums.iter()
        .fold(0, |mut sum, x| {
            sum += x;
            sum
        })
}