fn main() {
    // by default variabels are not changeable 
    // let x = 4; // implicitly assigned
    // let x: u32 = 4; assigned with type

    /*

    let mut x = 4;

    println!("x is: {}", x);
    x = 5;
    println!("x is: {}", x);

    */

    /*
    
    let x = 4;
    println!("x is: {}", x);

    { // new scope
        let x = x - 2; // x can be used from exterior scope
        println!("x is: {}", x);
    }

    let x = "hello";
    println!("x is: {}", x);

    */
    
    const SECONDS_IN_MINUTES: u32 = 10; // constants MUST have a type | can only be defined ONCE
    println!("{}", SECONDS_IN_MINUTES);
}
