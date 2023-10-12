use std::io;

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
    
    /*
    
    const SECONDS_IN_MINUTES: u32 = 10; // constants MUST have a type | can only be defined ONCE
    println!("{}", SECONDS_IN_MINUTES);

    */

    /*
    DATATYPES:

    primitive Types:

        scalar types:

            signed Integer:
                i8 = 8 bits -2^7 -> 2^7 - 1 = -128 -> 127
                i16 = 16 bits -2^15 -> 2^15 - 1 = -32.768 -> 32.767
                i32 = 32 bits -2^31 -> 2^31 - 1 = -2.147.483.648 -> 2.147.483.647
                i64 = 64 bits -2^63 -> 2^63 - 1 = -9.223.372.036.854.775.808 -> 9.223.372.036.854.775.807
                i128 = 128 bits -2^127 -> 2^127 - 1 = -85.070.591.730.234.615.865.843.651.857.942.052.864 -> 85.070.591.730.234.615.865.843.651.857.942.052.863

            unsigned Integer:
                u8 = 8 bits 0 -> 2^8 - 1 = 0 -> 255
                u16 = 16 bits 0 -> 2^16 - 1 = 0 -> 65.535
                u32 = 32 bits 0 -> 2^32 - 1 = 0 -> 4.294.967.295
                u64 = 64 bits 0 -> 2^64 - 1 = 0 -> 18.446.744.073.709.551.615
                u128 = 128 bits 0 -> 2^128 - 1 = 0 -> 340.282.366.920.938.463.463.374.607.431.768.211.455
            
            floats:
                f32 = 10^-37 -> 10^38
                f64 = 10^-307 -> 10^308
            
            bool:
                false = 0
                true = 1
            
            char:
                char = 'x'
    
        compound types:
            tuples:
                type cannot be changed
                items cannot be added
                let tup: (i32, bool, char) = (1, true, 's');
                let tup: (i8, bool, char) = (1, true, 's');
                println!("{}", tup.0);

            arrays:
                only the same type
                items cannot be added
                no automatic value assignment
                let arr: [i32; 5] = [1, 2, 3, 4, 5];
                println!("{}", arr[0])


    */


    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    println!("{}", input);



}
