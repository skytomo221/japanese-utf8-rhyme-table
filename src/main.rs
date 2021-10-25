use encoding::all::ISO_2022_JP;
use encoding::{EncoderTrap, Encoding};
use wana_kana::is_kanji::is_kanji;

fn main() {
    for rhyme in 0x80..=0xBF {
        print!("rhyme: xx xx {:x?}: ", rhyme);
        for i in 0x30..=0x9F {
            for j in [0, 1, 2, 3] {
                let c = char::from_u32(i * 0x100 + j * 0x40 + (rhyme - 0x80)).unwrap();
                let s = &c.to_string();
                if is_kanji(s) && ISO_2022_JP.encode(s, EncoderTrap::Strict).is_ok() {
                    print!("{}", c)
                } else if !('\u{3130}' <= c && c <= '\u{31ef}')
                    && !('㈀' <= c && c <= '㈞')
                    && !('㉠' <= c && c <= '㉾')
                    && !('\u{3300}' <= c && c <= '\u{9fff}')
                {
                    print!("{}", c)
                }
            }
        }
        println!()
    }
}
