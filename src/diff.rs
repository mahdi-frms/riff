#[derive(Clone,Debug,PartialEq, Eq, PartialOrd, Ord)]
pub enum Line {
    Normal(String),
    Added(String),
    Deleted(String)
}
type Table = Vec<Vec<usize>>;

fn split_lines(content:&String)->Vec<String>{
    content.split('\n').map(|x|String::from(x)).collect()
}

fn init_table(old:&Vec<String>,new:&Vec<String>) -> Table {
    let mut table = Vec::with_capacity(old.len()+1);
    for _ in 0..old.len()+1 {
        let mut row = Vec::with_capacity(new.len()+1);
        for _ in 0..new.len()+1 {
            row.push(0usize);
        }
        table.push(row);
    }
    table
}

fn fill_table(table:&mut Table,old:&Vec<String>,new:&Vec<String>){
    for j in 0..old.len() + 1 {
        for i in 0..new.len() + 1 {
            table[j][i] = if i * j == 0 {
                0
            }
            else if old[j-1] == new[i-1] {
                table[j-1][i-1]+1
            }
            else{
                std::cmp::max(table[j][i-1],table[j-1][i])
            };
        }
    }
}

fn generate_sequence(table:&Table,old:&Vec<String>,new:&Vec<String>)->Vec<Line> {

    let mut j = old.len();
    let mut i = new.len();
    let mut output = vec![];

    while j > 0 && i > 0 {

        if old[j-1] == new[i-1] {
            output.push(Line::Normal(old[j-1].clone()));
            i -= 1;
            j -= 1;
        }
        else if table[j-1][i] > table[j][i-1] {
            output.push(Line::Deleted(old[j-1].clone()));
            j -= 1;
        }
        else{
            output.push(Line::Added(new[i-1].clone()));
            i -= 1;
        }
    }

    while i > 0 {
        output.push(Line::Added(new[i-1].clone()));
        i -= 1;
    }

    while j > 0 {
        output.push(Line::Deleted(old[j-1].clone()));
        j -= 1;
    }

    output.reverse();
    output
}

pub fn diff(old:&String,new:&String) -> Vec<Line>{

    let old : Vec<String> = split_lines(old);
    let new : Vec<String> = split_lines(new);

    let mut table = init_table(&old, &new);
    fill_table(&mut table, &old, &new);
    generate_sequence(&table, &old, &new)
}

#[cfg(test)]
mod test {

    use super::*;

    fn compare(old:&str,new:&str)->Vec<Line> {
        diff(&String::from(old), &String::from(new))
    }

    #[test]
    fn no_difference_between_same_strings(){
        assert_eq!(compare("hello\nworld", "hello\nworld"),vec![
            Line::Normal(String::from("hello")),
            Line::Normal(String::from("world"))
        ]);
    }

    #[test]
    fn middle_difference(){
        assert_eq!(compare("hello\nhi\nworld", "hello\nbye\nworld"),vec![
            Line::Normal(String::from("hello")),
            Line::Deleted(String::from("hi")),
            Line::Added(String::from("bye")),
            Line::Normal(String::from("world"))
        ]);
    }

    #[test]
    fn middle_difference_various_size(){
        assert_eq!(compare("hello\nhi\nmy\nworld", "hello\nbye\nworld"),vec![
            Line::Normal(String::from("hello")),
            Line::Deleted(String::from("hi")),
            Line::Deleted(String::from("my")),
            Line::Added(String::from("bye")),
            Line::Normal(String::from("world"))
        ]);
    }

    #[test]
    fn middle_difference_extra(){
        assert_eq!(compare("hello\nworld", "hello\nbye\nworld"),vec![
            Line::Normal(String::from("hello")),
            Line::Added(String::from("bye")),
            Line::Normal(String::from("world"))
        ]);
    }

    #[test]
    fn middle_difference_removed(){
        assert_eq!(compare("hello\nhi\nworld", "hello\nworld"),vec![
            Line::Normal(String::from("hello")),
            Line::Deleted(String::from("hi")),
            Line::Normal(String::from("world"))
        ]);
    }

    #[test]
    fn after_added(){
        assert_eq!(compare("hello", "hello\nworld"),vec![
            Line::Normal(String::from("hello")),
            Line::Added(String::from("world"))
        ]);
    }

    #[test]
    fn after_removed(){
        assert_eq!(compare("hello\nworld", "hello"),vec![
            Line::Normal(String::from("hello")),
            Line::Deleted(String::from("world"))
        ]);
    }

    #[test]
    fn before_removed(){
        assert_eq!(compare("hello\nworld", "world"),vec![
            Line::Deleted(String::from("hello")),
            Line::Normal(String::from("world"))
        ]);
    }

    #[test]
    fn before_added(){
        assert_eq!(compare("hello\nworld", "hey\nhello\nworld"),vec![
            Line::Added(String::from("hey")),
            Line::Normal(String::from("hello")),
            Line::Normal(String::from("world")),
        ]);
    }

    #[test]
    fn totally_changed(){
        assert_eq!(compare("hello\nworld", "hi\nbuddy"),vec![
            Line::Deleted(String::from("hello")),
            Line::Deleted(String::from("world")),
            Line::Added(String::from("hi")),
            Line::Added(String::from("buddy")),
        ]);
    }
}