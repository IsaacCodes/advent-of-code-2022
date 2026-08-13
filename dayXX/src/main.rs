mod v1;
mod v2;

fn main() {
    //CLI args iterator
    let mut args = std::env::args();
    //Get away program name
    let program_name = args.next().unwrap();
    //Match 1 or 2 to version
    let version = args.next();
    match version.as_deref() {
        //Call appropriate version
        Some("1") => v1::v1(),
        Some("2") => v2::v2(),
        _ => println!("Usage: {program_name} [1 or 2]"),
    }
}
