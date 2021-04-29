fn main() {

    // Arrays - This is how you make an array with default value
    // when you define the length using the syntax below, you can ONLY use a constant
    // integer, you CANNOT use a variable.
    // value of each elementVV VV --   length of array
    let numbers = [0; 5];   //  [0, 0, 0, 0, 0, 0]

    // Initiating an array with different value elements
                          //0  1  2  3  4 
    let numbers = [1, 2, 3, 4, 5];

    // ARRAYS ARE STATIC LENGTH, THEY DO NOT GROW OR SHRINK
    // the moment you create an array of length 5, that array is ALWAYS of length 5

    println!("{}", numbers[3]);


    // Vectors - A collection that is of DYNAMIC Length, which means you can add and remove
    // elements as many times as you want.  you can start with a length of 0, and end up with a 
    // length of... however much ram you have....
    // ALSO, this is the most commonly used collection type.  if you have an EMPTY vec, YOU HAVE 
    // to declare the type... unless it's inferred later in your code
    let mut names = vec![];

    // inserts at the END of the vector
    names.push("Jake");

    // removes Option from the end of the vector
    names.pop();

    // inserts an element at the specified index of said vector
    names.insert(0, "Wendy");
    
    // removes an element at the specified index of said vector
    names.remove(0);

    let mut other_names = vec!["Aj", "Jimmy", "David"];

    // Like Arrays, Vectors are statically type, it can ONLY contain one type
    // other_names.push(2222);

    names.append(&mut other_names);


    // Iterators... we will have to "iterate" over this lesson a few times, hmm hmm hmm, ...kill me...later
    // You can iterate over ANY collection ever in rust. 

    // An Iterator is a Trait that requires just ONE function attached to it... the function signature look like this
    //
    //   next<T>() -> Option<T>,  Where "T" is the collection TYPE you are Iterating over
    // 
    //  all next DOES is RETURN the NEXT element in your Iterator
    //
    let numbers = [1, 2, 3, 4, 5];
    let mut number_iterator = numbers.iter();
    let x = number_iterator.next(); // x == Some(1)
    println!("next element in iterator: {:?}", x);

    let x = number_iterator.next(); // x == Some(2)
    println!("next element in iterator: {:?}", x);

    let x = number_iterator.next(); // x == Some(3)
    println!("next element in iterator: {:?}", x);

    let x = number_iterator.next(); // x == Some(4)
    println!("next element in iterator: {:?}", x);

    let x = number_iterator.next(); // x == Some(5)
    println!("next element in iterator: {:?}", x);

    let x = number_iterator.next(); // x == None
    println!("next element in iterator: {:?}", x);

    let x = number_iterator.next(); // x == None
    println!("next element in iterator: {:?}", x);


    let instructor = "James".to_string();

    let mut chars_iterator = instructor.chars();
    let character = chars_iterator.next(); // character == Some(J)
    println!("next letter in characters iterator: {:?}", character);

    let character = chars_iterator.next(); // character == Some(a)
    println!("next letter in characters iterator: {:?}", character);

    let character = chars_iterator.next(); // character == Some(m)
    println!("next letter in characters iterator: {:?}", character);

    let character = chars_iterator.next(); // character == Some(e)
    println!("next letter in characters iterator: {:?}", character);

    let character = chars_iterator.next(); // character == Some(s)
    println!("next letter in characters iterator: {:?}", character);

    let character = chars_iterator.next(); // character == None
    println!("next letter in characters iterator: {:?}", character);

    let character = chars_iterator.next(); // character == None
    println!("next letter in characters iterator: {:?}", character);

    let mut jake = "Jake".chars();


    loop {
        match jake.next() {
            Some(letter) => println!("{}", letter),
            None => break
        }
    }

    // A For loop in RUST, IS SYNTACTIC SUGAR for the loop + match above this code
    // this for loop WILL compile down to the match statement above.
    // the take away is, the For loop is actually ITERATING over your Iterator.
    //
    // A for loop is a Consumer, a consumer is called a consumer because it Calls the next()
    // function, until it returns None.
    // a consumer will keep iterating until there is 'None' to iterate over.
    for letter in "Jimmy".chars() {
        println!("{}", letter);
    }

    let mut number_iter = (0..=100)
        .map(|x| x*3)
        .filter(|x| x % 2 == 0)
        .take(4);

    let x =  number_iter.next();
    println!("x: {:?}", x);

    let x =  number_iter.next();
    println!("x: {:?}", x);

    let x =  number_iter.next();
    println!("x: {:?}", x);

    let x =  number_iter.next();
    println!("x: {:?}", x);

    let x =  number_iter.next();
    println!("x: {:?}", x);


    let mut number_iter = (0..=100)
        .map(|x| x*3)
        .filter(|x| x % 2 == 0)
        .take(4);

    for x in number_iter {
        println!("x: {:?}", x);
    }

    let mut number_iter = (0..=100)
        .map(|x| x * 3)
        .filter(|x| x % 2 == 0)
        .take(4);

    let max = number_iter.max();

    println!("max value is: {}", max.unwrap());


    // 1. Arrays, which are Statically length collections, once declared, they are ALWAYS 
    //    that size.  for example [1, 2, 3, 4, 5], this is an array of length 5, it is ALWAYS
    //    that length
    //
    // 2 Vec,  which are dynamically sized collections, which means you can add and remove elements
    //   at will and you will NOT run into an Index out of range, they are of varying, or DYNAMIC
    //   length
    //
    // 3  Iterators, an iterator is a trait, if a Type implements "Iterator", then that TYPE
    //    will have ACCESS to the next<T>() -> Option<T> function.  the next function returns the NEXT element
    //    in that Iterator.
    //
    //    When working with iterators, you can chain multiple iterator Functions together to create
    //    a unique Iterator, that when next is called "next()", it will CALL each function IN ORDER in the 
    //    Iterator Function chain.  
    // 
    //    A consumer is something that Takes OWNERSHIP and CONSUMES that iterator, a Consumer will call 
    //    next() For you, until there are "None" elements left.

}

fn print_names(names: Vec<&str>) {
    println!("{:?}", names)
}
