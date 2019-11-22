fn main() {
    {
        let x = 5;
        {
            let y = &x;
        }
    }

    {
        let x = "abc";
    }

    {
        let mut r;
        r = 5;
        r += 1;
        println!("r: {}", r); // 6
        // r = "abc"; // MISMATCHED TYPES
    }

    /*
    {
        let r;
        {
            let x = 5;
            r = &x; // X DOES NOT LIVE LONG ENOUGH
        }
        println!("r: {}", r);
    }
    */
    {
        let string1 = String::from("abcd");
        {
            let string2 = "xyz";
            let result = longest(string1.as_str(), string2);
            println!("The longest string is {}", result);
        }
    }

    /*
    {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str()); // NOT LIVE LONG ENOUGH
        }
        println!("The longest string is {}", result);
    }
    */
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
