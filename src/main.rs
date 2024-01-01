use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    //let name_arg = &args[0];
    //let function_arg = &args[1];

    match args[1].as_str() {
        "create" => println!("created a link repo!"),
        _ => println!(
            "Unknown command. Use {} help for more information.",
            args[0]
        ),
    }
}
