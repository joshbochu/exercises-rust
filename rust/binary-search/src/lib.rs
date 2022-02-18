pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let mut lo:i32 = 0;
    let mut hi:i32 = array.len() as i32 - 1;
    while lo <= hi {
        let mut mid = (lo + hi) / 2;
        let mid_key = array[mid as usize];
        if mid_key < key {
            lo = mid+1;
        } else if mid_key > key {
            hi = mid-1;
        } else {
            return Some(mid as usize);
        }
    }
    return None
}
