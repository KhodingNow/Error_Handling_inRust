A ShortCut for propagating Errors:the ? Operator

The '?' placed after the 'Result' value is defined to work in almost the same way as the 'match' expressions we define to handle the 'Result' values. If the value of the 'Result' is an 'Ok', the inside the 'Ok' will get returned from this expression, and the program will continue.

If the value is an 'Err', the 'Err' will be returned from the whole function as if we had used the 'returned' keyword so the error value get propagated to the calling code.

There is a difference between what the match expression does and what the '?' operator does: error values that have the '?' operator called on them go through the 'from' function, defined in the "From' traitin the standard library, which is used to convert values from one type into another.
When the '?' operator calls the 'from' function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

For example, we couold change the 'read_username_from_file'  function to return a custom error type named 'OurError' that we define. If we also define 'impl From<io::Error> for 'OurError' to construct an instance of 'OurError' from an 'io::Error', then the '?' operator calls in the body of 'read_username_from_file' will call 'from' and convert the error types without needing to add any more code to the function. 
The '?' operator eliminates a lot of biolerplate and makes this function's implementation simpler. We could even shorten this code by further chaining method calls immediately after the '?' :

    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }