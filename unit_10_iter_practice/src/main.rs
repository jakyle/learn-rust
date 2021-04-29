fn main() {
    // next<T>() -> Option<T>
    // Iterator Functions vs. Consumers
    // Consumers Consume and Take ownership of the iterator in question
    // Iterator functions automatically runs when you call next on it.
    // ITERATORS ARE LAZY... I repeat, ITERATORS ARE LAZY.
    // this means the iterator will only "iterate" IF you call next 
    // or you have a consumer like a for loop or a function like max()
    // or min() chain at the end of your iterator.
    //
    // this iterator, and the functions being chained on to it WILL NOT
    // get called UNTIL a consumer calls the "next()" function.
    let numbers = [1, 2, 3, 4, 5]
        .iter()
        .enumerate() // iterator function
        .map(|(i, x)| x * x); // iterator function
    // ^^^ below we cover map and enumerate and iter
    //
    //
    // Iterator Functions
    //
    //
    // MAP ITERATOR
    // =====================================================================
    // will get each element in the iterator, and transform said element
    // and return that NEW transformed item based on the closure
    // that is passed into it.
    //
    // definition: e will be mapped to b where e is the current element and 
    // b is what the element is being transformed to
    // a -> b
    let mut numbers_iter_map = [1, 2, 3, 4, 5].iter().map(|a| i32::pow(*a, 2));
    //   [1, 2, 3, 4, 5] -> [ 1, 4, 9, 16, 25 ]

    let x = numbers_iter_map.next(); // x == Some(1)
    let x = numbers_iter_map.next(); // x == Some(4)
    let x = numbers_iter_map.next(); // x == Some(9)
    let x = numbers_iter_map.next(); // x == Some(16)

    // Also mapping from a -> b, where a is of type: i32, and b is of type Point
    let mut point_iter = [1, 2, 3, 4, 5]
        .iter()
        .map(|&x| Point { x, y: x });

    let point = point_iter.next(); // point == Point { x == 1, y == 1 }
    let point = point_iter.next(); // point == Point { x == 2, y == 2 }
    let point = point_iter.next(); // point == Point { x == 3, y == 3 }

    let mut code_iter = "Jake".bytes().map(|c| (c + 2) as char);

    for c in code_iter {
        println!("{}", c); // "L, c, m, g", will be printed on each Line in the console 
    }

    // CLOSURES
    // ================================================================================================
    // a peek at Closures
    // A CLOSURE, is a FUNCTION
    // closure syntax.  | x: i32 | x * x;
    // the '|' or "pipe" is the "opening parenthesis" or '(' of our function
    // "x: 32" is the parameter of our closure, or the parameter of our Function
    // same syntax for defining our parameters in a function you create, ie fn do_thing(x: 32)...
    //  {
    // x*x
    //}
    //  This is the BODY of our closure, just like the body of a function.  because a closure is just 
    // a function, the closure itself ALSO has a body that runs lines of codes.
    //
    // Closures are Types, and CAN be assigned to a variable as defined below.
    let mapper = | x: &i32 | {
        x * x
    };

    // IF you are providing an closure as an argument to a function, you do NOT have to
    // provide the type.  this is because the type is IMPLIED. this is implied because the
    // the function that takes IN a closure as an argument KNOWS exactly what TYPE of
    // closure it needs to take in, therefore when WE pass in a closure, we do NOT
    // need to define the type.
    let mut number_iter_map = [1, 2, 3, 4, 5]
        .iter()
        .map(|x| x + 1) // Closure in this map function is "|x| x + 1", we do NOT have to provide the type because it is implied by .map()
        .map(|x| x + 2); // Closure in this map function is "|x| x+2", we do NOT have to provide the type because it is implied by .map()

    let x = number_iter_map.next(); // iterating over: 1
    let x = number_iter_map.next(); // iterating over: 2

    
    for x in [1, 2, 3, 4, 5].iter().map(|x| x+1).map(|x| x + 2) {
        println!("{}", x)
    } 

    // FILTER ITERATOR
    // =======================================================================
    // Filter is a Iterator Function that takes in a Predicate as an argument
    // a Predicate in programming, is SIMPLY a FUnction that RETURNS a bool
    // IF the filter function passed in Evaluates to True, it will return
    // that Element in the iterator
    // if it's FALSE, it will discard that element, and THEN process the 
    // next element instead
    // in other words, Filter will call next UNTIL it evaluates to true or
    // until the consumer exhaust the iterator.
    //
    // definition: Some(e) will ONLY be passed as the output of next() IF
    // e passes the predicate passed into filter()
    // e -> IF e == true -> e
    let mut number_iter_filter = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .filter(|&x| x % 2 == 0);

    let x = number_iter_filter.next(); // x == Some(2)
    let x = number_iter_filter.next(); // x == Some(4)
    let x = number_iter_filter.next(); // x == Some(6)
    let x = number_iter_filter.next(); // x == Some(8)
    let x = number_iter_filter.next(); // x == Some(10)
    let x = number_iter_filter.next(); // x == None


    // filter will ONLY pass the value of tw, this is the only NUMBER that is even
    let mut number_iter_filter = [1, 3, 5, 7, 2]
        .iter()
        .filter(|&x| x % 2 == 0);
    
    let x = number_iter_filter.next(); 
    println!("filtered down: {:?}", x); // Some(2)

    let x = number_iter_filter.next();
    println!("filtered down: {:?}", x); // None

    // ENUMERATE ITERATOR
    // ================================================================
    // Enumerate will pass in the CURRENT INDEX of your iterator WITH 
    // the item you are Iterating over!
    // when you call enumerate, it will Wrap the Next in a tuple.
    // the first value of the tuple, will be the CURRENT index of that
    // iterator, the second value in the tuple, will be the element
    // that is currently being iterated over.
    // 
    // definition: e will be mapped to a tuple of i and e, where i is the
    // current index, and e is the current element being iterated over.
    // e -> (i, e)
    let mut number_iter_enumerate = [16, 17, 18, 19, 20]
        .iter()
        .enumerate(); //  x -> (index, x)

    let (i, x) = number_iter_enumerate.next().unwrap(); // i == 0, x == 16
    let (i, x) = number_iter_enumerate.next().unwrap(); // i == 1, x == 17
    let (i, x) = number_iter_enumerate.next().unwrap(); // i == 2, x == 18
    let (i, x) = number_iter_enumerate.next().unwrap(); // i == 3, x == 19
    let (i, x) = number_iter_enumerate.next().unwrap(); // i == 4, x == 20
    let x = number_iter_enumerate.next();       // x == None

    for (i, x) in [16, 17, 18, 19, 20].iter().enumerate() {
        println!("index: {}, value: {}", i, x);
    }

    // as a reminder, a Range is an iterator as well
    // this will printline!() each number between 2 and 
    // 33, excluding 33
    for x in 2..=33 {
        println!("{}", x);
    }

    // how to iterate over the INDEX of a vector
    let mut names = vec!["Jake".to_string(), "Ryan".to_string(), "Wendy".to_string()]; // ["Jakeay", "Ryanay", "Wendyay"]

    // IF you need to change the value of the CURRENT element you are iterating over,
    // instead of calling .iter(), call .iter_mut() instead! refer to the different types
    // of iterators below for a further explanation.
    for name in names.iter_mut() {
        name.push_str("ay");
    }

    // TYPES OF ITERATORS!!!
    //==================================
    // Three types of iterators
    // iter() ->  iterator of References
    // iter_mut() -> a mutable reference iterator
    // into_iter() -> calling into iterator will take FULL ownership over 
    //              what you are iterating over, and discard the THING
    //              you are iterating over. 


    // iter gets a regular immutable reference of each element
    // that is being iterated over.
    // in other words, you will be able to use the 
    // numbers variable AFTER the loop, because the loop
    // is just &, or borrowing the thing we are 
    // iterating over.
    let numbers = [1, 2, 3, 4];
    for x in numbers.iter() {
        println!("{}", x);
    }

    // iter_mut gets a mutable reference of each element
    // that is being iterated over.
    // in other words, you will be able to use the
    // numbers variable AFTER the loop, because the loop
    // is just &mut, or mutably Borrowing the the thing
    // we are iterating over.
    let mut numbers = [1, 2, 3, 4];
    for x in numbers.iter_mut() {
        *x += 1;
    }

    // into_iter() is taking FULL ownership over the numbers_owned
    // variable, which means we CANNOT reference numbers_owned
    // we are NOT passing in a reference to numbers to the loop, we are 
    // passing FULL OWNERSHIP to the vector.
    let numbers_owned = vec![1, 2, 3, 4];
    for x in numbers_owned.into_iter() {
        println!("{}", x);
    }

}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}