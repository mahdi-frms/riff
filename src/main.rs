mod diff;

fn main() {

    let old = 
"hello
hei
heil";

    let new = 
"hello
hei
kuft
heil";

    for l in diff::diff(&String::from(old), &String::from(new)) {
        println!("{:?}",l);
    }
}