fn main() {
    
    
    // Unrecoverable Error with panic
    // There are two ways to cause panic in practice: by taking action that causes the code to panic (e.g, accessing an array past the end)
    // or by explicitly calling the 'panic!' macro. On both ocassion, the panics will print a failure message, unwind (clean up memory), clean up the stack, cleans up the data and quit.
    // Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.
    //  the OS needs to claean up the (as part of cleaning up the memory the program was using) memeory the program was usin. If in your project, you need to make the resultant binary as small as possible, you can switch from unwinding to aborting upon a panic
    // by adding - 'panic = "abort" ' to the appropriate [profile] section in the 'Cargo.toml' file.
    // e.g = [profile, release]
    //       panic = 'abort'

    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

        // Recoverable Errors with 'Result'

   // use std::fs::File;
   // fn main() {
        // let greeting_file_result = File::open("hello.txt");

        // let greeting_file = match greeting_file_result {
        //     Ok(file) => file,
        //     Err(error) => panic!("Problem opening the file: {error:?}"),
        // };

        // This code will 'panic' because hello.txt does not exist. We can create the file, ok. What if we dont have permission to the file?
        // We still want the code to panic. For this we add another 'match' expression.

        // let greeting_file_result = File::open("hello.txt");

        // let greeting_file = match greeting_file_result {
        //     Ok(file) => file,
        //     Err(error) => match error.kind() {
        //         ErrorKind::NotFound => match File::create("hello.txt") {
        //             Ok(fc) => fc,
        //         Err(e) => panic!("Problem creating the file: {e:?}"),
        //         },
        //         other_error => {
        //             panic!("Problem opening the file: {other_error:?}");
        //         }
        //     },

            // The type of the value that 'File::open' return inside the Err variant is 'io::Error', which is a struct provided by the standard library.
            // This struct has a method 'kind' that we can call to get an 'io::ErrorKind' value.The enum 'io::ErrorKind' is provided by the standard library and has variants representing the different kinds of errors that might result from an 'io' operation
            // The variant we want to use is 'ErrorKind::NotFound', which indicates the file we're trying to open doesn't exist yet.
            // So, we match on 'greeting_file_result', but we also have an inner match on 'error.kind()'
            
            // The condition we want to check in the inner match is the value returned by 'error.kind()' is the 'NotFound' variant of the 'ErrorKind' enum. If it is, we try to create the file with 'File::create'.
            // However, because 'File::create' could also fail, we need a second arm in the inner 'match' expression. When the file can't be created, a different 'error' message is printed.
            // The sond arm of the outer 'match' stays the same, so the program panics on any error besides the missing file error.  

            /// >>>> THAT WAS A LOT of 'match' - we can use CLOSURES with 'Result<T, E>' for a short cut of the above.

        //     use std::fs::File;
        //     use std::io::ErrorKind;
            
        //     fn main() {
        //         let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        //             if error.kind() == ErrorKind::NotFound {
        //                 File::create("hello.txt").unwrap_or_else(|error| {
        //                     panic!("Problem creating the file: {error:?}");
        //                 })
        //             } else {
        //                 panic!("Problem opening the file: {error:?}");
        //             }
        //         });
        //     }

        // };


        // Shotcuts for Panic on Error: unwrap and expect

        // Using 'match' works well enough, but can be verbose and NOT communicate intent very well. The 'Result<T, E>' has many helper methods defined on it to do various, more specific tasks. 
        // The 'unwrap' method is a shotcut method implemented just like the 'match' expression. If the 'Result' value is the 'Ok' variant, 'unwrap' will return the value inside 'Ok' 
        // If the 'Result' is the 'Err' variant, unwrap will call the 'panic!' macro.

        // EXAMPLE:

        use std::fs::File;
        fn main() {
            let greeting_file = File::open("hello.txt").unwrap();
        }



    //}
}




