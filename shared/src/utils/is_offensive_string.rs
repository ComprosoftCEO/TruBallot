use censor::Censor;
use lazy_static::lazy_static;

// Additional words to censor
const EXTRA_WORDS: &[&str] = &[
  "anal", "anus", "arse", "ball", "barf", "bastard", "biatch", "bloody", "blow", "bollock", "bollok", "boner",
  "booger", "bugger", "bum", "butt", "clitoris", "cock", "coon", "crap", "crib", "cunt", "dildo", "dyke", "fag",
  "fart", "feck", "fellate", "fellatio", "felching", "fudge", "flange", "homo", "jerk", "jizz", "knob", "labia",
  "lmao", "lmfao", "muff", "nigga", "omg", "pee", "piss", "poo", "poop", "prick", "pube", "puke", "queer", "scrotum",
  "slut", "smegma", "spunk", "tit", "tosser", "turd", "twat", "wank", "wtf",
];

lazy_static! {
  static ref CUSTOM_CENSOR: Censor = Censor::Custom(EXTRA_WORDS.iter().map(|s| String::from(*s)).collect());
}

//
// Censor the string that it contains no offensive or vulgar words
//  Returns true if the string fails the test
//
pub fn is_offensive_string(string: &str) -> bool {
  Censor::Standard.check(string)
    || Censor::Sex.check(string)
    || Censor::Zealous.check(string)
    || CUSTOM_CENSOR.check(string)
}
