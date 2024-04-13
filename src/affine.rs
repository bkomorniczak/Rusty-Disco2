const M: i32 = 26;

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
    text.to_uppercase().chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| {
            let x = c as i32 - 'A' as i32;
            let y = ((a * x + b) % M + M) % M;
            (y + 'A' as i32) as u8 as char
        })
        .collect()
}


pub fn decrypt(text: &str, a: i32, b: i32) -> Option<String> {
    inverted_mod(a).map(|a_inv| {
        text.chars()
            .map(|c| {
                if c.is_alphabetic() {
                    let x = c as i32 - 'A' as i32;
                    let y = ((x - b) * a_inv % M + M) % M;
                    (y + 'A' as i32) as u8 as char
                } else {
                    c
                }
            })
            .collect()
    })
}



pub fn has_modular_inverse(a: i32) -> bool {
    gcd(a, M) == 1
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}