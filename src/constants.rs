const ALPHABET: [char; 59] = [
      'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
      's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и',
      'й', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ',
      'ы', 'ь', 'э', 'ю', 'я',
];


const CAPITAL_ALPHABET: [char; 59] = [
      'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
      'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И',
      'Й', 'К', 'Л', 'М', 'Н', 'О', 'П', 'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ',
      'Ы', 'Ь', 'Э', 'Ю', 'Я',
];


const VOWELS: [char; 15] =
      ['a', 'e', 'i', 'o', 'u', 'а', 'е', 'и', 'о', 'у', 'ы', 'э', 'ю', 'я', 'ё'];


const PUNCTUATIONS: [char; 32] = [
      '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
      '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];


const MARKING: [char; 3] = [' ', '\n', '\t'];


pub const MIN_SENTENCE_LEN: usize = 10;


pub fn is_alphabetic(c: &char) -> bool
{
      ALPHABET.contains(c) || CAPITAL_ALPHABET.contains(c)
}


pub fn is_vowel(c: &char) -> bool
{
      VOWELS.contains(c)
}


pub fn is_punctuation(c: &char) -> bool
{
      PUNCTUATIONS.contains(c)
}


pub fn is_marking(c: &char) -> bool
{
      MARKING.contains(c)
}
