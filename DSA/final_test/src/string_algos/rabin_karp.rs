pub fn rabin_karp(pattern: &str, text: &str) -> Vec<usize> {
    let d: u32 = 256; 
    let q: u32 = 101; 

    let pattern_bytes = pattern.as_bytes();
    let text_bytes = text.as_bytes();

    let m: usize = pattern.len();
    let n: usize = text.len();

    let mut pattern_hash: u32 = 0; 
    let mut text_hash: u32 = 0;    
    let mut h: u32 = 1;

    let mut matches: Vec<usize> = Vec::new();

    if m > n {
        return matches;
    }

    for _ in 0..m-1 {
        h = (h * d) % q;
    }

    for i in 0..m {
        pattern_hash = (d * pattern_hash + pattern_bytes[i] as u32) % q;
        text_hash = (d * text_hash + text_bytes[i] as u32) % q;
    }

    for i in 0..=n-m {
        if pattern_hash == text_hash {
            let mut match_found = true;
            for j in 0..m {
                if text_bytes[i+j] != pattern_bytes[j] {
                    match_found = false;
                    break;
                }
            }

            if match_found {
                matches.push(i);
            }
        }

        if i < n-m {
            text_hash = (d * (text_hash + q - (text_bytes[i] as u32 * h) % q) % q +
                text_bytes[i+m] as u32) % q;
        }
    }

    matches
}