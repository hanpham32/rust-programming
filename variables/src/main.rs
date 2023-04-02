fn main() {
    // by default, rust variables are immutable

    /* MUTABILITY */
    // including the mut keyword makes the variable mutable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* CONSTANTS */
    // type of value must be annotated
    // naming convention is all caps with underscores
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

    /* SHADOWING */
    // shadowing allows us to change the type of a variable
    // we shadow the variable by using the same name and repeating the let keyword

    /* two differences between shadowing and mut
    (1) we can perform a few transformations on a value but have 
    the variable be immutable after those transformations have been completed.
    (2) we can change the type of the value but reuse the same name. */

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y is: {y}");
    }
    println!("The value of y is: {y}");

}
