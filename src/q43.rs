use std::collections::HashSet;

pub fn main() {
    let how = 3;

    let list = generate_parenth(how);
    println!("{:?}", list);
}

fn generate_parenth(how: usize) -> HashSet<String> {
    if how == 0 {
        return HashSet::<String>::new();
    }
    if how == 1 {
        return HashSet::from(["()".to_string()]);
    }

    let mut parenth_list = HashSet::<String>::new();

    let remain_parenth_list = generate_parenth(how - 1);

    for parenth in remain_parenth_list {
        let left = "()".to_string() + &parenth;
        let right = parenth.clone() + "()";
        let center = format!("({})", parenth);

        parenth_list.insert(left);
        parenth_list.insert(right);
        parenth_list.insert(center);
    }

    return parenth_list;
}
