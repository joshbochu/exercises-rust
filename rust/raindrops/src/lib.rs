pub fn raindrops(n: u32) -> String {
    let mut sounds = vec![];
    if n % 3 == 0 {
        sounds.push("Pling")
    }
    if n % 5 == 0 {
        sounds.push("Plang")
    }
    if n % 7 == 0 {
        sounds.push("Plong")
    }
    match sounds.first() {
        None => return n.to_string(),
        Some(_v) => return sounds.join("")
    }
}
