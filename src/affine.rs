fn inverted_mod(a: i32) -> Option<i32> {
    let mut mn = (26, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    if mn.0 > 1 { return None; }
    if xy.0 < 0 { xy.0 += 26; }
    Some(xy.0)
}

pub fn encrypt(text: &str, a: i32, b: i32) -> String {
    let m = 26;
    text.to_uppercase().chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| {
            let x = c as i32 - 'A' as i32;
            let y = ((a * x + b) % m + m) % m;
            (y + 'A' as i32) as u8 as char
        })
        .collect()
}

pub fn decrypt(text: &str, a: i32, b: i32) -> Option<String> {
    let m = 26;
    inverted_mod(a).map(|a_inv| {
        text.chars()
            .map(|c| {
                if c.is_alphabetic() {
                    let x = c as i32 - 'a' as i32;
                    let y = ((x - b) * a_inv % m) % m;
                    (y + 'a' as i32) as u8 as char
                } else {
                    c
                }
            })
            .collect()
    })
}

pub fn brute_force_affine(ciphertext: &str) -> Vec<(String, i32, i32)> {
    let m =26;
    let valid_a = [1,3,5,7,9,11,15,17,19,21,23,25];

    let mut results = Vec::new();
    for &a in valid_a.iter() {
        for b in 0..m {
            if let Some(plaintext) = decrypt(ciphertext, a, b,m) {
                results.push((plaintext,a,b));
            }
        }
    }
    results
}


