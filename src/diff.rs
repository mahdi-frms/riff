#[derive(Clone,Debug,PartialEq, Eq, PartialOrd, Ord)]
pub enum Line<T> {
    Normal(T),
    Added(T),
    Deleted(T)
}
type Table = Vec<Vec<usize>>;

fn split_lines(content:&str)->Vec<&str>{
    content.split('\n').collect()
}

fn init_table<T>(old:&Vec<T>,new:&Vec<T>) -> Table {
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

fn fill_table<T> (table:&mut Table,old:&Vec<T>,new:&Vec<T>)
where T : Eq{
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

fn generate_sequence<T>(table:&Table,old:&Vec<T>,new:&Vec<T>)->Vec<Line<T>> 
where T : Clone + Eq{

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

pub fn diff<T>(old:&Vec<T>,new:&Vec<T>) -> Vec<Line<T>>
where T : Clone + Eq {

    let mut table = init_table(&old, &new);
    fill_table(&mut table, old, new);
    generate_sequence(&table, old, new)
}

#[cfg(test)]
mod test {

    use super::*;

    fn compare(old:&str,new:&str,arr:Vec<Line<&str>>){
        let old = split_lines(old);
        let new = split_lines(new);
        assert_eq!(diff(&old, &new),arr);
    }

    #[test]
    fn no_difference_between_same_strings(){
        compare("hello\nworld",  "hello\nworld",vec![
            Line::Normal("hello"),
            Line::Normal("world")
        ]);
    }

    #[test]
    fn middle_difference(){
        compare("hello\nhi\nworld",  "hello\nbye\nworld",vec![
            Line::Normal("hello"),
            Line::Deleted("hi"),
            Line::Added("bye"),
            Line::Normal("world")
        ]);
    }

    #[test]
    fn middle_difference_various_size(){
        compare("hello\nhi\nmy\nworld",  "hello\nbye\nworld",vec![
            Line::Normal("hello"),
            Line::Deleted("hi"),
            Line::Deleted("my"),
            Line::Added("bye"),
            Line::Normal("world")
        ]);
    }

    #[test]
    fn middle_difference_extra(){
        compare("hello\nworld",  "hello\nbye\nworld",vec![
            Line::Normal("hello"),
            Line::Added("bye"),
            Line::Normal("world")
        ]);
    }

    #[test]
    fn middle_difference_removed(){
        compare("hello\nhi\nworld",  "hello\nworld",vec![
            Line::Normal("hello"),
            Line::Deleted("hi"),
            Line::Normal("world")
        ]);
    }

    #[test]
    fn after_added(){
        compare("hello",  "hello\nworld",vec![
            Line::Normal("hello"),
            Line::Added("world")
        ]);
    }

    #[test]
    fn after_removed(){
        compare("hello\nworld",  "hello",vec![
            Line::Normal("hello"),
            Line::Deleted("world")
        ]);
    }

    #[test]
    fn before_removed(){
        compare("hello\nworld",  "world",vec![
            Line::Deleted("hello"),
            Line::Normal("world")
        ]);
    }

    #[test]
    fn before_added(){
        compare("hello\nworld",  "hey\nhello\nworld",vec![
            Line::Added("hey"),
            Line::Normal("hello"),
            Line::Normal("world"),
        ]);
    }

    #[test]
    fn totally_changed(){
        compare("hello\nworld",  "hi\nbuddy",vec![
            Line::Deleted("hello"),
            Line::Deleted("world"),
            Line::Added("hi"),
            Line::Added("buddy"),
        ]);
    }
}