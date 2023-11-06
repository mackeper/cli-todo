mod arg_parser;
mod operations;

use arg_parser::parse_operation;

fn print_usage(program_name: &str) {
    let operations = [
        ("list (l)", "Display all current items."),
        ("add (a) [item]", "Add a new item."),
        ("remove (r) [id]", "Remove an item by its ID."),
        ("done (d) [id]", "Toggle its done state by ID."),
        ("clear", "Remove all items."),
    ];

    let max_command_length = operations
        .iter()
        .map(|(cmd, _)| cmd.len())
        .max()
        .unwrap_or(0);

    println!("Usage: {} [operation] [arguments]", program_name);
    println!("Operations:");
    for (command, description) in operations {
        println!(
            "    {:width$} : {}",
            command,
            description,
            width = max_command_length
        );
    }
}
 
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program_name = &args[0];

    if args.len() < 2 {
        print_usage(program_name);
        return;
    }

    match parse_operation(&args[1], &args[2..]) {
        Ok(op) => {
            if let Err(err) = operations::execute(op) {
                println!("{}", err);
            }
        }
        Err(err) => {
            println!("{}", err);
            print_usage(program_name);
        }
    }
}
