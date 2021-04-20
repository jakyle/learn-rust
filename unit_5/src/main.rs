fn main() {
    println!("Hello, world!");

    let coco = Dog {
        has_killed_before: false,
        name: "Coco".to_string()
    };
    
    // a static function, we do NOT have to have a dog INSTANCE! in memory in order to call this function
    let mut otis = Dog::new("Otis".to_string(), false);

    println!("dog's name: {}", coco.name);

    println!("{}", otis.bark());
    // Cannot call Dog::self without a 'self', because Dog is just a Blue print, where 'self' refers to an INSTANCE of a dog.
    // println!("{}", Dog::bark());

    // the drop_dog method takes drop_dog(self) as a parameter, NOT a reference or a mut Reference, but ITself,
    // which is to say, drop_dog takes OWNERSHIP over self.
    println!("{}", otis.drop_dog());  


    // we CANNOT call this method anymore, because, otis no longer exist into memory, this is because
    // drop_dog() took OWNERSHIP of otis and dropped it out of memory at the end of the dop_dog() method.
    // otis.make_innocent();

}

// STRUCT REVIEW

struct Dog {
    has_killed_before: bool,
    name: String
}

impl Dog {

    fn new(name: String, has_killed_before: bool) -> Self {  // so the Self IS the same type as the Type that is being implemented
        Self {  // will be of type Dog becasue "Dog is being implemented"
            has_killed_before,
            name
        }
    }

    fn new_innocent_dog(name: String) -> Self {
        Self {
            has_killed_before: true,
            name
        }
    }

    fn drop_dog(self) -> String {
        format!("Say bye bye to {}!", self.name)
    }

    // A method, is a function that is implementing struct, and takes in self as a parameter
    fn bark(&self) -> String {
        format!("Bark Bark, my name is: {}", self.name)
    }

    fn make_innocent(&mut self) {
        self.has_killed_before = false;
    }

}
