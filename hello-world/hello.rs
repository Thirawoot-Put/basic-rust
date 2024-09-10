fn main() {
    // this is single line comment in rust!!!
    /*
     * this is multiple line comment in rust!!!
     *
     * */

    println!("Hello World!");
    println!("I'm a Rustacean!");

    println!("{} days", 1);
    println!(
        "Hi, I'm {0}. I learining about {1}. {0} will be {1} developer.",
        "Tle", "Rust"
    );
    println!("Base 10: {}", 1234);
    println!("Base 2(binary): {:b}", 1234);
    println!("Base 8(octal): {:o}", 1234);
    println!("Base hexadecimal: {:x}", 1234);
    println!("{num:>3}", num = 1);
    println!("{num:0>3}", num = 1);
    println!("{num:0<3}", num = 1);
    println!("{num:>width$}", num = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    // This will not compile because `Structure` not implement
    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    let num: f64 = 4.0;
    let width: usize = 5;
    println!("{num:>width$}");
}
