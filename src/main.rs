// See https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=7ed82c3ddaa89baa307465401fb11328

use encoding::all::ISO_2022_JP;
use encoding::{EncoderTrap, Encoding};
use unicode_blocks::{
    find_unicode_block, CJK_COMPATIBILITY, ENCLOSED_CJK_LETTERS_AND_MONTHS, HIRAGANA, KATAKANA,
};
use unicode_general_category::{get_general_category, GeneralCategory};

fn can_encode_jis_code(c: char) -> bool {
    ISO_2022_JP
        .encode(&c.to_string(), EncoderTrap::Strict)
        .is_ok()
}

fn main() {
    for rhyme in 0x80..=0xBF {
        print!("rhyme: xx xx {:x?}: ", rhyme);
        for i in 0x30..=0x9F {
            for j in [0, 1, 2, 3] {
                let c = char::from_u32(i * 0x100 + j * 0x40 + (rhyme - 0x80)).unwrap();
                let block = find_unicode_block(c).unwrap();
                if get_general_category(c) != GeneralCategory::Unassigned
                    && (can_encode_jis_code(c)
                        || block == HIRAGANA
                        || block == KATAKANA
                        || (block == ENCLOSED_CJK_LETTERS_AND_MONTHS
                            && !('㈀' <= c && c <= '㈞')
                            && !('㉠' <= c && c <= '㉾'))
                        || block == CJK_COMPATIBILITY)
                {
                    print!("{}", c)
                }
            }
        }
        println!()
    }
}
