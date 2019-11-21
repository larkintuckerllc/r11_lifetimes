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
}
