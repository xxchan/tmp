use clap::command;

fn main() {
    let command = command!("My Super Program")
        .about("Stupid")
        .version("114514")
        .propagate_version(true);
    let matches = command.get_matches();
}
