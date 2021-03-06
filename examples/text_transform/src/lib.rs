#![recursion_limit="1024"]

#[macro_use]
extern crate helix;

use std::collections::HashMap;
use helix::Symbol;

ruby! {
    class TextTransform {
        def widen(text: String) -> String {
            text.chars().map(|char| {
                match char {
                    ' ' => '\u{3000}', '!' => '！', '"' => '＂', '#' => '＃', '$' => '＄', '%' => '％', '&' => '＆', '\'' => '＇',
                    '(' => '（', ')' => '）', '*' => '＊', '+' => '＋', ',' => '，', '-' => '－', '.' => '．', '/' => '／',
                    '0' => '０', '1' => '１', '2' => '２', '3' => '３', '4' => '４', '5' => '５', '6' => '６', '7' => '７',
                    '8' => '８', '9' => '９', ':' => '：', ';' => '；', '<' => '＜', '=' => '＝', '>' => '＞', '?' => '？',
                    '@' => '＠', 'A' => 'Ａ', 'B' => 'Ｂ', 'C' => 'Ｃ', 'D' => 'Ｄ', 'E' => 'Ｅ', 'F' => 'Ｆ', 'G' => 'Ｇ',
                    'H' => 'Ｈ', 'I' => 'Ｉ', 'J' => 'Ｊ', 'K' => 'Ｋ', 'L' => 'Ｌ', 'M' => 'Ｍ', 'N' => 'Ｎ', 'O' => 'Ｏ',
                    'P' => 'Ｐ', 'Q' => 'Ｑ', 'R' => 'Ｒ', 'S' => 'Ｓ', 'T' => 'Ｔ', 'U' => 'Ｕ', 'V' => 'Ｖ', 'W' => 'Ｗ',
                    'X' => 'Ｘ', 'Y' => 'Ｙ', 'Z' => 'Ｚ', '[' => '［', '\\' => '＼', ']' => '］', '^' => '＾', '_' => '＿',
                    '`' => '｀', 'a' => 'ａ', 'b' => 'ｂ', 'c' => 'ｃ', 'd' => 'ｄ', 'e' => 'ｅ', 'f' => 'ｆ', 'g' => 'ｇ',
                    'h' => 'ｈ', 'i' => 'ｉ', 'j' => 'ｊ', 'k' => 'ｋ', 'l' => 'ｌ', 'm' => 'ｍ', 'n' => 'ｎ', 'o' => 'ｏ',
                    'p' => 'ｐ', 'q' => 'ｑ', 'r' => 'ｒ', 's' => 'ｓ', 't' => 'ｔ', 'u' => 'ｕ', 'v' => 'ｖ', 'w' => 'ｗ',
                    'x' => 'ｘ', 'y' => 'ｙ', 'z' => 'ｚ', '{' => '｛', '|' => '｜', '}' => '｝', '~' => '～', _ => char,
                }
            }).collect()
        }

        def widen_array(text: Vec<String>) -> Vec<String> {
            text.into_iter().map(TextTransform::widen).collect()
        }

        def widen_hash(text: HashMap<Symbol, String>) -> HashMap<Symbol, String> {
            text.into_iter().map(|(k,v)| (Symbol::from_string(TextTransform::widen(k.to_string())), TextTransform::widen(v))).collect()
        }

        def narrowen(text: String) -> String {
            text.chars().map(|char| {
                match char {
                    '\u{3000}' => ' ', '！' => '!', '＂' => '"', '＃' => '#', '＄' => '$', '％' => '%', '＆' => '&', '＇' => '\'',
                    '（' => '(', '）' => ')', '＊' => '*', '＋' => '+', '，' => ',', '－' => '-', '．' => '.', '／' => '/',
                    '０' => '0', '１' => '1', '２' => '2', '３' => '3', '４' => '4', '５' => '5', '６' => '6', '７' => '7',
                    '８' => '8', '９' => '9', '：' => ':', '；' => ';', '＜' => '<', '＝' => '=', '＞' => '>', '？' => '?',
                    '＠' => '@', 'Ａ' => 'A', 'Ｂ' => 'B', 'Ｃ' => 'C', 'Ｄ' => 'D', 'Ｅ' => 'E', 'Ｆ' => 'F', 'Ｇ' => 'G',
                    'Ｈ' => 'H', 'Ｉ' => 'I', 'Ｊ' => 'J', 'Ｋ' => 'K', 'Ｌ' => 'L', 'Ｍ' => 'M', 'Ｎ' => 'N', 'Ｏ' => 'O',
                    'Ｐ' => 'P', 'Ｑ' => 'Q', 'Ｒ' => 'R', 'Ｓ' => 'S', 'Ｔ' => 'T', 'Ｕ' => 'U', 'Ｖ' => 'V', 'Ｗ' => 'W',
                    'Ｘ' => 'X', 'Ｙ' => 'Y', 'Ｚ' => 'Z', '［' => '[', '＼' => '\\', '］' => ']', '＾' => '^', '＿' => '_',
                    '｀' => '`', 'ａ' => 'a', 'ｂ' => 'b', 'ｃ' => 'c', 'ｄ' => 'd', 'ｅ' => 'e', 'ｆ' => 'f', 'ｇ' => 'g',
                    'ｈ' => 'h', 'ｉ' => 'i', 'ｊ' => 'j', 'ｋ' => 'k', 'ｌ' => 'l', 'ｍ' => 'm', 'ｎ' => 'n', 'ｏ' => 'o',
                    'ｐ' => 'p', 'ｑ' => 'q', 'ｒ' => 'r', 'ｓ' => 's', 'ｔ' => 't', 'ｕ' => 'u', 'ｖ' => 'v', 'ｗ' => 'w',
                    'ｘ' => 'x', 'ｙ' => 'y', 'ｚ' => 'z', '｛' => '{', '｜' => '|', '｝' => '}', '～' => '~', _ => char,
                }
            }).collect()
        }

        def narrowen_array(text: Vec<String>) -> Vec<String> {
            text.into_iter().map(TextTransform::narrowen).collect()
        }

        def narrowen_hash(text: HashMap<Symbol, String>) -> HashMap<Symbol, String> {
            text.into_iter().map(|(k,v)| (Symbol::from_string(TextTransform::narrowen(k.to_string())), TextTransform::narrowen(v))).collect()
        }

        def flip(text: String) -> String {
            text.chars().rev().map(|char| {
                match char {
                    '!' => '¡', '"' => '„', '&' => '⅋', '\'' => '‚', '(' => ')', ')' => '(', ',' => '‘', '.' => '˙',
                    '1' => 'Ɩ', '2' => 'ᄅ', '3' => 'Ɛ', '4' => 'ㄣ', '5' => 'ϛ', '6' => '9', '7' => 'ㄥ',
                    '8' => '8', '9' => '6', ';' => '؛', '<' => '>', '>' => '<', '?' => '¿',
                    'A' => '∀', 'B' => '𐐒', 'C' => 'Ↄ', 'D' => '◖', 'E' => 'Ǝ', 'F' => 'Ⅎ', 'G' => '⅁',
                    'J' => 'ſ', 'K' => 'ʞ', 'L' => '⅂', 'M' => 'W',
                    'P' => 'Ԁ', 'Q' => 'Ό', 'R' => 'ᴚ', 'T' => '⊥', 'U' => '∩', 'V' => 'ᴧ', 'W' => 'M',
                    'Y' => '⅄', '[' => ']', ']' => '[', '^' => 'v', '_' => '‾',
                    '`' => ',', 'a' => 'ɐ', 'b' => 'q', 'c' => 'ɔ', 'd' => 'p', 'e' => 'ǝ', 'f' => 'ɟ', 'g' => 'ƃ',
                    'h' => 'ɥ', 'i' => 'ᴉ', 'j' => 'ɾ', 'k' => 'ʞ', 'm' => 'ɯ', 'n' => 'u',
                    'p' => 'd', 'q' => 'b', 'r' => 'ɹ', 't' => 'ʇ', 'u' => 'n', 'v' => 'ʌ', 'w' => 'ʍ',
                    'y' => 'ʎ', '{' => '}', '}' => '{', _ => char,
                }
            }).collect()
        }

        def flip_array(text: Vec<String>) -> Vec<String> {
            text.into_iter().rev().map(TextTransform::flip).collect()
        }

        def flip_hash(text: HashMap<Symbol, String>) -> HashMap<Symbol, String> {
            text.into_iter().map(|(k,v)| (Symbol::from_string(TextTransform::flip(v)), TextTransform::flip(k.to_string()))).collect()
        }
    }
}
