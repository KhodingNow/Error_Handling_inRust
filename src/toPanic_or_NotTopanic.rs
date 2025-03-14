// Cases where you have more Information that the compiler:
// It would be appropriate to call 'unwrap' or 'expect' when you have some other logic that ensures the 'Result' will have an 'Ok' value, 
// but the logic is not something the compiler understands.
// You 'll still have a 'Result' value that you need to handle: whatever operation you're calling still has the possibility of failling in general, even though it's logically impossible in this situation.
// If you can ensure by manually inspecting the code that you'll never have an "Err" variant, it's perfectly acceptable to call 'unwrap', and even better to document the reason you think you'll never have an 'Err' variant in the 'expect' text.

//EXAMPLE:

use std::net::IpAddr;

let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");

// We are calling an IpAddr instance by parsing a hardcoded string. We can see that 127.0.0.1 is a valid IP address, so it's acceptable to use 'expect'here.
// However, having a hardcoded, valid string doesn't change the return type of the 'parse' method: we still get a 'Result' value.
// and the compiler will still make us handle the 'Result' as if the 'Err' variant is a possibility because the compiler is not smart enough to see that this IP address is always a valid IP address.
// If the IP address came from a user rather than being hardcoded into the program and therefore DID have a possibility of failure, we'd definitely want to handle the "Result'
// in a more robust way instead.
// 