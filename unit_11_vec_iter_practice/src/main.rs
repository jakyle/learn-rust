fn main() {
    println!("Hello, world!");

   // let _ = running_sum(vec![41, 69, 420]);
    let result = num_jewels_in_stones("aA", "aAAbbbb");

    let name = "James".to_string();

    let name_ref = &name;

    deref_ownership_example(&name);

    println!("original string: {}", name);
    println!("should be: {}, got: {}", 3, result);

    let accounts = vec![vec![1,5], vec![7,3], vec![3,5]];

    let result = maximum_wealth(&accounts);

    println!("should be: {}, got: {}", 10, result);

}

fn deref_ownership_example(name: &str) {
    println!("{}", name);
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
//    acc = 0,  x = 1    return next() -> 1
// -> acc = 1,  x = 2    return next() -> 3
// -> acc = 3,  x = 3    return next() -> 6
// -> acc = 6,  x = 4    return next() -> 10
// -> acc = 10, x = 5    return next() -> 15

// a Reducer, is simillar to the .map(), where map maps from a -> b
// a reducer will get (a1, a2, ... alast) -> accumulator


fn drainenstien_sum(mut nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![];

    for i in 0..nums.len() {  // i = 2, x = 3
        let mut drained_numbers: Vec<i32> =  nums.drain(0..i+1).collect(); // [1],

        let final_sum: i32 = drained_numbers.iter().sum(); // 6
        result.push(final_sum);

        &drained_numbers.append(&mut nums); // [1, 3, 3, 4]
        nums = drained_numbers;        
    }   
    result

}








/*
You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.

Letters are case sensitive, so "a" is considered a different type of stone from "A".

Example 1:

Input: jewels = "aA", stones = "aAAbbbb"
Output: 3

Example 2:

Input: jewels = "z", stones = "ZZ"
Output: 0


Constraints:

    1 <= jewels.length, stones.length <= 50
    jewels and stones consist of only English letters.
    All the characters of jewels are unique.

*/
pub fn num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    stones
        .chars()
        .filter(|stone| jewels.contains(*stone))
        .count() as i32
}
// deferencing temporarily REMOVES the '&' where & == reference
// ie &char ->  *&char == char  

pub fn ryan_num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    let mut output = 0;
    for x in jewels.chars() {
        for y in stones.chars() {
            if x == y {
                output += 1;
            } 
        }
    }
    output
}


pub fn ryan_refactor_num_jewels_in_stones(jewels: &str, stones: &str) -> i32 {
    jewels
        .chars()
        .filter(|&jewel| stones.contains(jewel))
        .count() as i32
}



/*
You are given an m x n integer grid accounts where accounts[i][j] is the amount of money the i​​​​​​​​​​​th​​​​ customer has in the j​​​​​​​​​​​th​​​​ bank. Return the wealth that the richest customer has.

A customer's wealth is the amount of money they have in all their bank accounts. The richest customer is the customer that has the maximum wealth.


Example 1:

Input: accounts = [[1,2,3],[3,2,1]]
Output: 6
Explanation:
1st customer has wealth = 1 + 2 + 3 = 6
2nd customer has wealth = 3 + 2 + 1 = 6
Both customers are considered the richest with a wealth of 6 each, so return 6.

Example 2:

Input: accounts = [[1,5],[7,3],[3,5]]
Output: 10
Explanation: 
1st customer has wealth = 6
2nd customer has wealth = 10 
3rd customer has wealth = 8
The 2nd customer is the richest with a wealth of 10.

Example 3:

Input: accounts = [[2,8,7],[7,1,3],[1,9,5]]
Output: 17

Constraints:

    m == accounts.length
    n == accounts[i].length
    1 <= m, n <= 50
    1 <= accounts[i][j] <= 100
*/



pub fn maximum_wealth(accounts: &[Vec<i32>]) -> i32 {  // [17, 10, 15]
    accounts
        .iter()
        .map(|account| account.iter().sum())
        .max()
        .unwrap_or(0)
}

pub fn maximum_weal_2(accounts: Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for account in &accounts { 
        let account_total: i32 = account.iter().sum(); //  account_total = 15
        sum = account_total.max(sum);  // sum = the Greater of the two, either 17 or 15
    }

    sum
}


//⠀⠀⠀⠀⠀⢀⣴⡾⠿⠿⠿⠿⢶⣦⣄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⢠⣿⠁⠀⠀⠀⣀⣀⣀⣈⣻⣷⡄⠀⠀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⣾⡇⠀⠀⣾⣟⠛⠋⠉⠉⠙⠛⢷⣄⠀⠀⠀⠀⠀⠀⠀
//⢀⣤⣴⣶⣿⠀⠀⢸⣿⣿⣧⠀⠀⠀⠀⢀⣀⢹⡆⠀⠀⠀⠀⠀⠀
//⢸⡏⠀⢸⣿⠀⠀⠀⢿⣿⣿⣷⣶⣶⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀
//⣼⡇⠀⢸⣿⠀⠀⠀⠈⠻⠿⣿⣿⠿⠿⠛⢻⡇⠀⠀⠀⠀⠀⠀⠀
//⣿⡇⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣼⣷⣶⣶⣶⣤⡀⠀⠀
//⣿⡇⠀⢸⣿⠀⠀⠀⠀⠀⠀⣀⣴⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀
//⢻⡇⠀⢸⣿⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⡿⠿⣿⣿⣿⣿⡇
//⠈⠻⠷⠾⣿⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⢸⣿⣿⣿⣇
//⠀⠀⠀⠀⣿⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⢸⣿⣿⣿⡿
//⠀⠀⠀⠀⢿⣧⣀⣠⣴⡿⠙⠛⠿⠿⠿⠿⠉⠀⠀⢠⣿⣿⣿⣿⠇
//⠀⠀⠀⠀⠀⢈⣩⣭⣥⣤⣤⣤⣤⣤⣤⣤⣤⣤⣶⣿⣿⣿⣿⠏⠀
//⠀⠀⠀⠀⣴⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠋⠀⠀
//⠀⠀⠀⢸⣿⣿⣿⡟⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠋⠁⠀⠀⠀⠀
//⠀⠀⠀⢸⣿⣿⣿⣷⣄⣀⣀⣀⣀⣀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀
//⠀⠀⠀⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠀⠀⠀
//⠀⠀⠀⠀⠀⠈⠛⠿⠿⣿⣿⣿⣿⣿⠿⠿⢿⣿⣿⣿⣿⣿⡄⠀⠀
//⠀⠀⠀⠀⠀⠀⢀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⢀⣹⣿⣿⣿⡇⠀⠀
//⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠀⠀   ~~~
//⠀⠀⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⠁⠀⠀⠀ /    
//⠀⠀⠀⠀⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠉⠉⠁⢤⣤⣤⣤⣤⣤⣤⡀/
//⠀⠀⠀⠀⢿⣿⣿⣿⣷⣶⣶⣶⣶⣾⣿⣿⣿⣆⢻⣿⣿⣿⣿⣿⡇
//⠀⠀⠀⠀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⠻⣿⣿⣿⡿
//⠀⠀⠀⠀⠀⠀⠈⠙⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠉⠀⠙⠛⠉⠀