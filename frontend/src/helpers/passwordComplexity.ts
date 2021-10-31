// 0 - Too guessable: risky password. (guesses < 10^3)
// 1 - Very guessable: protection from throttled online attacks. (guesses < 10^6)
// 2 - Somewhat guessable: protection from unthrottled online attacks. (guesses < 10^8)
// 3 - Safely unguessable: moderate protection from offline slow-hash scenario. (guesses < 10^10)
// 4 - Very unguessable: strong protection from offline slow-hash scenario. (guesses >= 10^10)
export const MINIMUM_PASSWORD_COMPLEXITY = 3;

/// Used by zxcvbn
export const SITE_SPECIFIC_WORDS = ['evoting', 'voting', 'voter'];
