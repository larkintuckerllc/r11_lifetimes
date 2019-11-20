fn main() {
    {
        let r;
        // println!("r: {}", r); // UN-INITIALIZED
        {
            let x = 5;
            r = &x;
        }
    }
    /*
    {
        let r;
        {
            let x = 5;
            r = &x; // NOT LIVE LONG ENOUGH
        }
        println!("r: {}", r);
    }
    */
}
