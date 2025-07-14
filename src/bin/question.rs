#![allow(unused)]

fn f1() -> Result<u32, String> {
    println!("f1");
    Ok(1)
}

fn f2() -> Result<u32, String> {
    println!("f2");
    Ok(2)
}

// This is long and hard to follow
fn f_match() -> Result<u32, String> {
    let res_1 = f1();
    let x1 = match res_1 {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };
    let res_2 = f2();
    let x2 = match res_2 {
        Ok(x) => x,
        Err(err) => {
            return Err(err);
        }
    };
    Ok(x1 + x2)
}

// This does the same as above
fn f_question() -> Result<u32, String> {
    let x1 = f1()?;
    let x2 = f2()?;
    // Could pattern match and convert if Err type is not the same
    Ok(x1 + x2)
}

// Question operator
// Error type must be the same as function
// If f1 didn't return a String the compiler would raise an exception
fn main() -> Result<(), String> {
    // Automatically handle errors
    // let res = f1();
    // match res {
    //     Ok(x) => println!("{x}"),
    //     Err(err) => println!("err {err}"),
    // }
    let z = f_question();
    match z {
        Ok(x) => println!("z = {x}"),
        Err(err) => println!("err {err}"),
    }
    let x = f1()?;
    println!("x = {x}");

    Ok(())
}
