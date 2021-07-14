mod diff;

fn main() {

    let old = 
"www
hello
hei
heil";

    let new = 
"vg
hello
hei
kuft
heil";

    for l in diff::diff(&String::from(old), &String::from(new)) {
        println!("{:?}",l);
    }
}