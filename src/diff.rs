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

fn generate_sequence(table:&Table,old:&Vec<String>,new:&Vec<String>)->Vec<String> {

    let mut j = old.len();
    let mut i = new.len();
    let mut index = table[j][i];
    let mut output : Vec<String> = vec![String::new();index];

    while j > 0 && i > 0 {

        if old[j-1] == new[i-1] {
            output[index-1] = old[j-1].clone();
            i -= 1;
            j -= 1;
            index -= 1;
        }
        else if table[j-1][i] > table[j][i-1] {
            j -= 1;
        }
        else{
            i -= 1;
        }
    }

    output
}

pub fn diff(old:&String,new:&String) -> Vec<String>{

    let old : Vec<String> = split_lines(old);
    let new : Vec<String> = split_lines(new);

    let mut table = init_table(&old, &new);
    fill_table(&mut table, &old, &new);
    generate_sequence(&table, &old, &new)
}