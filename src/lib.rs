pub fn pattern_count(s: &str, k: &str) -> u32 {
    let mut count: u32 = 0;
    for i in 0..s.len()-k.len()+1 {
        if &s[i..i+k.len()] == k {
            count += 1;
        }
    }
    count
}
