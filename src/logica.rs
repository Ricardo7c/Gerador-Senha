use sha2::{Digest, Sha256};


pub fn phrase_to_password(
    phrase: &str,
    length: usize,
    include_symbols: bool,
    salt: &str,
) -> Result<String, String> {
    if length < 4 {
        return Err("O tamanho mínimo da senha é 4 caracteres.".to_string());
    }

    let lowers = b"abcdefghijklmnopqrstuvwxyz";
    let uppers = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let digits = b"0123456789";
    let symbols = b"!@#$%^&*()-_+=";

    let mut combined = Vec::new();
    combined.extend_from_slice(lowers);
    combined.extend_from_slice(uppers);
    combined.extend_from_slice(digits);

    if include_symbols {
        combined.extend_from_slice(symbols);
    }

    let mut hasher = Sha256::new();
    hasher.update(salt.as_bytes());
    hasher.update(phrase.as_bytes());
    let digest = hasher.finalize();

    let classes: Vec<&[u8]> = if include_symbols {
        vec![lowers, uppers, digits, symbols]
    } else {
        vec![lowers, uppers, digits]
    };

    let mut pwd_chars = Vec::new();

    for (i, class) in classes.iter().enumerate() {
        let byte = digest[i];
        let index = byte as usize % class.len();
        pwd_chars.push(class[index]);
    }

    let mut idx = classes.len();
    while pwd_chars.len() < length {
        let byte = digest[idx % digest.len()];
        let index = byte as usize % combined.len();
        pwd_chars.push(combined[index]);
        idx += 1;
    }

    let rot = digest[classes.len()] as usize % length;
    pwd_chars.rotate_left(rot);

    Ok(String::from_utf8(pwd_chars).unwrap())
}

