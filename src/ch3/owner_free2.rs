fn main() {
    {
        let s1 = String::from("真実はワインの中にある");
        let s3 = String::from("美人と葡萄は手がかかる");
        {
            let s2 = s1;
            // println!("{}", s1);
            println!("{}", s2);
            println!("{}", s3);
        }
        // println!("{}", s1);
        // println!("{}", s2);
        println!("{}", s3);
    }
    // println!("{}", s1)
    // println!("{}", s2)
    // println!("{}", s3)
}
