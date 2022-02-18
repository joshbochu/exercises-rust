pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::from(""),
        _ => {
            let mut lines = vec![];
            for i in (0..list.len()-1) {
                lines.push(format!("For want of a {} the {} was lost.\n", list[i], list[i+1]))
            }
            lines.push(format!("And all for the want of a {}.", list[0]));
            lines.join("")
        }
    }
}
