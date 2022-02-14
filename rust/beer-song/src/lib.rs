pub fn verse(n: u32) -> String {
    if n > 0 {
        let v = match n - 1 {
            0 => format!("{} bottle of beer on the wall, {} bottle of beer.\n\
                            Take it down and pass it around, no more bottles of beer on the wall.\n", n, n),
            1 => format!("{} bottles of beer on the wall, {} bottles of beer.\n\
                            Take one down and pass it around, 1 bottle of beer on the wall.\n", n, n),
            _ => format!("{} bottles of beer on the wall, {} bottles of beer.\n\
                            Take one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
        };
        v
    } else {
        let last = format!("No more bottles of beer on the wall, no more bottles of beer.\n\
                Go to the store and buy some more, 99 bottles of beer on the wall.\n");
            last
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start).rev().map(verse).collect::<Vec<String>>().join("\n")
}
