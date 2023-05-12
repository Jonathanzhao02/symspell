#![no_main]
use libfuzzer_sys::fuzz_target;
use symspell::*;

fuzz_target!(|input: (String, u8)| {
    let word = input.0.replace(|c: char| c == ' ', "");
    let mut symspell: SymSpell<UnicodeStringStrategy> = SymSpell::default();
    symspell.load_dictionary_line(&format!("{} {}", word, input.1)[..], 0, 1, " ");
});