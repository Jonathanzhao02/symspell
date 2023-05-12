#![no_main]
use libfuzzer_sys::fuzz_target;
use symspell::*;

fuzz_target!(|input: (String, String, u8)| {
    let word0 = input.0.replace(|c: char| c == ' ', "");
    let word1 = input.1.replace(|c: char| c == ' ', "");
    let mut symspell: SymSpell<UnicodeStringStrategy> = SymSpell::default();
    symspell.load_bigram_dictionary_line(&format!("{} {} {}", word0, word1, input.2)[..], 0, 2, " ");
});