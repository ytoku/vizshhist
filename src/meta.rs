// metafy/unmetafy function in zsh
// https://github.com/zsh-users/zsh/blob/73d317384c9225e46d66444f93b46f0fbe7084ef/Src/utils.c#L4740
// imeta is defined in inittyptab function
// https://github.com/zsh-users/zsh/blob/73d317384c9225e46d66444f93b46f0fbe7084ef/Src/utils.c#L4091
// Meta, Pound, LAST_NORMAL_TOK, Nularg and Marker are defined in zsh.h
// https://github.com/zsh-users/zsh/blob/73d317384c9225e46d66444f93b46f0fbe7084ef/Src/zsh.h#L157

const META: u8 = 0x83;
const MARKER: u8 = 0xa2;

#[inline]
fn imeta(c: u8) -> bool {
    // same as 0 | META | MARKER | POUND..=NULARG
    matches!(c, 0 | META..=MARKER)
}

pub fn metafy(text: &[u8], bytes: &mut Vec<u8>) {
    bytes.clear();
    bytes.reserve(text.len());
    for &c in text {
        if imeta(c) {
            bytes.push(META);
            bytes.push(c ^ 32);
        } else {
            bytes.push(c);
        }
    }
}

pub fn unmetafy(bytes: &[u8], text: &mut Vec<u8>) {
    text.clear();
    text.reserve(bytes.len());
    let mut p = bytes.iter();
    while let Some(&c1) = p.next() {
        let c = match c1 {
            META => match p.next() {
                Some(&c2) => c2 ^ 32,
                None => META,
            },
            _ => c1,
        };
        text.push(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metafy_ascii() {
        let text = "test";
        let expected = text.as_bytes();
        let mut actual = vec![];
        metafy(text.as_bytes(), &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_metafy_non_ascii() {
        let text = "テスト";
        let expected = vec![
            0xe3, 0x83, 0xa3, 0x83, 0xa6, 0xe3, 0x82, 0xb9, 0xe3, 0x83, 0xa3, 0x83, 0xa8,
        ];
        let mut actual = vec![];
        metafy(text.as_bytes(), &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_unmetafy_ascii() {
        let bytes = "test".as_bytes();
        let expected = bytes;
        let mut actual = vec![];
        unmetafy(bytes, &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_unmetafy_non_ascii() {
        let bytes = &vec![
            0xe3, 0x83, 0xa3, 0x83, 0xa6, 0xe3, 0x82, 0xb9, 0xe3, 0x83, 0xa3, 0x83, 0xa8,
        ];
        let expected = "テスト".as_bytes();
        let mut actual = vec![];
        unmetafy(bytes, &mut actual);
        assert_eq!(actual, expected);
    }
}
