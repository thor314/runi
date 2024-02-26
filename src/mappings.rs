use std::collections::HashMap;

lazy_static! {
    static ref SUPERSCRIPT: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
        m.insert('a', "ᵃ");
        m.insert('b', "ᵇ");
        m.insert('c', "ᶜ");
        m.insert('d', "ᵈ");
        m.insert('e', "ᵉ");
        m.insert('f', "ᶠ");
        m.insert('g', "ᵍ");
        m.insert('h', "ʰ");
        m.insert('i', "ⁱ");
        m.insert('j', "ʲ");
        m.insert('k', "ᵏ");
        m.insert('l', "ˡ");
        m.insert('m', "ᵐ");
        m.insert('n', "ⁿ");
        m.insert('o', "ᵒ");
        m.insert('p', "ᵖ");
        m.insert('q', "q");
        m.insert('r', "ʳ");
        m.insert('s', "ˢ");
        m.insert('t', "ᵗ");
        m.insert('u', "ᵘ");
        m.insert('v', "ᵛ");
        m.insert('w', "ʷ");
        m.insert('x', "ˣ");
        m.insert('y', "ʸ");
        m.insert('z', "ᶻ");
        m.insert('A', "ᴬ");
        m.insert('B', "ᴮ");
        m.insert('C', "ᶜ");
        m.insert('D', "ᴰ");
        m.insert('E', "ᴱ");
        m.insert('F', "ᶠ");
        m.insert('G', "ᴳ");
        m.insert('H', "ᴴ");
        m.insert('I', "ᴵ");
        m.insert('J', "ᴶ");
        m.insert('K', "ᴷ");
        m.insert('L', "ᴸ");
        m.insert('M', "ᴹ");
        m.insert('N', "ᴺ");
        m.insert('O', "ᴼ");
        m.insert('P', "ᴾ");
        m.insert('Q', "Q");
        m.insert('R', "ᴿ");
        m.insert('S', "ˢ");
        m.insert('T', "ᵀ");
        m.insert('U', "ᵁ");
        m.insert('V', "ⱽ");
        m.insert('W', "ᵂ");
        m.insert('X', "ˣ");
        m.insert('Y', "ʸ");
        m.insert('Z', "ᶻ");
        m.insert('0', "⁰");
        m.insert('1', "¹");
        m.insert('2', "²");
        m.insert('3', "³");
        m.insert('4', "⁴");
        m.insert('5', "⁵");
        m.insert('6', "⁶");
        m.insert('7', "⁷");
        m.insert('8', "⁸");
        m.insert('9', "⁹");
        m
    };

    static ref SUPERSCRIPT: HashMap<char, &'static str> = {
        let mut m = HashMap::new();
    'a' => 'ₐ',
    'b' => 'b',
    'c' => 'c',
    'd' => 'd',
    'e' => 'ₑ',
    'f' => 'f',
    'g' => 'g',
    'h' => 'ₕ',
    'i' => 'ᵢ',
    'j' => 'ⱼ',
    'k' => 'ₖ',
    'l' => 'ₗ',
    'm' => 'ₘ',
    'n' => 'ₙ',
    'o' => 'ₒ',
    'p' => 'ₚ',
    'q' => 'q',
    'r' => 'ᵣ',
    's' => 'ₛ',
    't' => 'ₜ',
    'u' => 'ᵤ',
    'v' => 'ᵥ',
    'w' => 'w',
    'x' => 'ₓ',
    'y' => 'y',
    'z' => 'z',
    'A' => 'ₐ',
    'B' => 'B',
    'C' => 'C',
    'D' => 'D',
    'E' => 'ₑ',
    'F' => 'F',
    'G' => 'G',
    'H' => 'ₕ',
    'I' => 'ᵢ',
    'J' => 'ⱼ',
    'K' => 'ₖ',
    'L' => 'ₗ',
    'M' => 'ₘ',
    'N' => 'ₙ',
    'O' => 'ₒ',
    'P' => 'ₚ',
    'Q' => 'Q',
    'R' => 'ᵣ',
    'S' => 'ₛ',
    'T' => 'ₜ',
    'U' => 'ᵤ',
    'V' => 'ᵥ',
    'W' => 'W',
    'X' => 'ₓ',
    'Y' => 'Y',
    'Z' => 'Z',
    '0' => '₀',
    '1' => '₁',
    '2' => '₂',
    '3' => '₃',
    '4' => '₄',
    '5' => '₅',
    '6' => '₆',
    '7' => '₇',
    '8' => '₈',
    '9' => '₉'
  };
}