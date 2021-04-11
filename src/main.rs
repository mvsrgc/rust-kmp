fn lps_table(needle: &str) -> Vec<usize> {
    let mut lps_table = vec![0; needle.len()];

    let mut i = 1;
    let mut len = 0;

    while i < needle.len() {
        if needle.chars().nth(i) == needle.chars().nth(len) {
            len += 1;
            lps_table[i] = len;
            i += 1;
        } else {
            if len != 0 {
                len = lps_table[len - 1];
            } else {
                lps_table[i] = len;
                i += 1;
            }
        }
    }

    lps_table
}
fn main() {
    let needle = "ababaca";

    println!("{:?}", lps_table(needle));
}
