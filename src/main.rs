use std::process::exit;

use diff::diff;

mod diff;

fn main() {

    let help = "usage:\nriff [file1] [file2]";
    let args : Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        println!("{}",help);
        exit(0);
    }

    if args.len() != 3 {
        eprintln!("{}",help);
        exit(1);
    }

    let args : Vec<String> = args.iter().skip(1).map(|address|{
        match std::fs::read_to_string(address) {
            Ok(text)=>{
                text
            },
            Err(_)=>{
                eprintln!("Error: could not open {}",address);
                exit(1);
            }
        }
    }).collect();

    let old = diff::split_lines(args[0].as_str());
    let new = diff::split_lines(args[1].as_str());
    let comp = diff(&old, &new);
    for line in comp {
        match line {
            diff::Line::Added(text)=> println!("+|{}",text),
            diff::Line::Deleted(text)=> println!("-|{}",text),
            diff::Line::Normal(text)=> println!(" |{}",text),
        }
    }   
}