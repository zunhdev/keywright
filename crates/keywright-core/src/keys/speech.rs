//! Keys' names as VoiceOver should say them (design §12): punctuation and symbols by name,
//! such as `Semicolon` for `;`. VoiceOver skips most punctuation at its usual level, so
//! "Types ;." would be heard as "Types".
//!
//! The names follow Apple's for shortcuts, such as Command-Comma and Command-Minus, in the
//! title case of Keywright's other key names, such as `Forward Delete`.

/// `name` as VoiceOver should say it: each word that's only punctuation or symbols, such as
/// `;` or the `-` of `Keypad -`, by the names of its characters, such as `Keypad Minus`.
/// Letters, digits and words with them, such as `Play/Pause`, stay as they are, and so do
/// symbols without a name here.
#[must_use]
pub fn spoken(name: &str) -> String {
    name.split(' ')
        .map(|word| {
            if word.is_empty() || word.chars().any(char::is_alphanumeric) {
                return word.to_owned();
            }
            word.chars()
                .map(|character| {
                    character_name(character).map_or_else(|| character.to_string(), str::to_owned)
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// What a punctuation mark or symbol is called, if it's one that keys type: on a US
/// keyboard, or on the other input sources people use with Macs.
const fn character_name(character: char) -> Option<&'static str> {
    let name = match character {
        // A US keyboard's.
        '`' => "Grave Accent",
        '~' => "Tilde",
        '!' => "Exclamation Mark",
        '@' => "At Sign",
        '#' => "Number Sign",
        '$' => "Dollar Sign",
        '%' => "Percent Sign",
        '^' => "Caret",
        '&' => "Ampersand",
        '*' => "Asterisk",
        '(' => "Left Parenthesis",
        ')' => "Right Parenthesis",
        '-' => "Minus",
        '_' => "Underscore",
        '=' => "Equals Sign",
        '+' => "Plus Sign",
        '[' => "Left Bracket",
        ']' => "Right Bracket",
        '{' => "Left Brace",
        '}' => "Right Brace",
        '\\' => "Backslash",
        '|' => "Vertical Bar",
        ';' => "Semicolon",
        ':' => "Colon",
        '\'' => "Apostrophe",
        '"' => "Quotation Mark",
        ',' => "Comma",
        '.' => "Period",
        '<' => "Less-Than Sign",
        '>' => "Greater-Than Sign",
        '/' => "Slash",
        '?' => "Question Mark",
        // Other input sources', such as German, French, Spanish, British and Russian.
        '§' => "Section Sign",
        '±' => "Plus-Minus Sign",
        '´' => "Acute Accent",
        '¨' => "Diaeresis",
        '°' => "Degree Sign",
        '£' => "Pound Sign",
        '€' => "Euro Sign",
        '¥' => "Yen Sign",
        '¢' => "Cent Sign",
        '¡' => "Inverted Exclamation Mark",
        '¿' => "Inverted Question Mark",
        '«' => "Left Guillemet",
        '»' => "Right Guillemet",
        '¬' => "Not Sign",
        '¦' => "Broken Bar",
        '·' => "Middle Dot",
        '¸' => "Cedilla",
        '˜' => "Small Tilde",
        '‘' => "Left Single Quotation Mark",
        '’' => "Right Single Quotation Mark",
        '‚' => "Low Single Quotation Mark",
        '“' => "Left Double Quotation Mark",
        '”' => "Right Double Quotation Mark",
        '„' => "Low Double Quotation Mark",
        '–' => "En Dash",
        '—' => "Em Dash",
        '…' => "Ellipsis",
        '•' => "Bullet",
        '¶' => "Pilcrow",
        '×' => "Multiplication Sign",
        '÷' => "Division Sign",
        '№' => "Numero Sign",
        '₽' => "Ruble Sign",
        _ => return None,
    };
    Some(name)
}

#[cfg(test)]
mod tests {
    use super::spoken;

    #[test]
    fn punctuation_is_said_by_name() {
        assert_eq!(spoken(";"), "Semicolon");
        assert_eq!(spoken("Keypad -"), "Keypad Minus");
        assert_eq!(spoken("ISO #"), "ISO Number Sign");
        assert_eq!(spoken("Keypad &&"), "Keypad Ampersand Ampersand");
        assert_eq!(spoken("§"), "Section Sign");
    }

    #[test]
    fn letters_digits_and_words_stay_as_they_are() {
        assert_eq!(spoken("Q"), "Q");
        assert_eq!(spoken("4"), "4");
        assert_eq!(spoken("Ö"), "Ö");
        assert_eq!(spoken("Forward Delete"), "Forward Delete");
        assert_eq!(spoken("Play/Pause"), "Play/Pause");
        // A symbol without a name here is left for VoiceOver.
        assert_eq!(spoken("☃"), "☃");
    }
}
