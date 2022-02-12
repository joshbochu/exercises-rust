pub fn square_of_sum(n: u32) -> u32 {
    let mut s = 0;
    for i in 1..n+1 {
        s += i
    }
    (s*s)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut s = 0;
    for i in 1..n+1 {
        s += (i*i)
    }
    s
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
