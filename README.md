# Bible Lib
A simple Rust crate for accessing Bible texts.

## Features
- Get verses, verse ranges and chapters from a selected bible version.
- Included bible translations (can each be enabled / disabled via [Cargo Features]):
  - American King James Version (AKJV)
  - American Standard Version (ASV)
  - English Standard Version (ESV)
  - King James Version (KJV)
- Custom bible translations (see [Custom Translations])
- Random verse selection (can be disabled via [Cargo Features])
- Detect bible verses in a string (disabled by default, see [Cargo Features])

## Cargo Features
- **akjv** American King James Version (enabled by default)
- **asv** American Standard Version (enabled by default)
- **esv** English Standard Version (enabled by default)
- **kjv** King James Version (enabled by default)
- **random** random verse selection (enabled by default)
- **detection** detect bible verses in a string (disabled by default)

## Future Plans
- Add more bible translations
- Add support for reverse lookup (search for a phrase and get the verse(s) containing it)
- Add support for different output formats (e.g. JSON, XML) (instead of just plain text)

## Custom Translations
Each verse must be layed out as its own line as follows:  
`Book Chapter#:Verse# Text`  
see examples [here](./bible_translations)

## Examples

Get a specific verse:
```rust
use bible_lib::*;

// load the bible with the desired translation
let bible = Bible::new(Translation::AmericanKingJames);

// get a specific verse (John 3:16 in this case)
// this will not include superscripts
let requested_verse: String = bible.get_verse("John", 3, 16, false).unwrap();

// print the verse text
println!("John 3:16: {}", requested_verse.text);
```
Get a range of verses:
```rust
use bible_lib::*;

// load the bible with the desired translation
let bible = Bible::new(Translation::EnglishStandard);

// get a range of verses (Luke 23:39-43 in this case)
// this will return a concatenated string of all verses in the range, with superscripts
let verses: String = bible.get_verse(BibleLookup::new_range("Luke", 23, 39, 43), false).unwrap();

// print the verses
println!("Luke 23:39-43: {}", verses);
```
Get a whole chapter:
```rust
use bible_lib::*;

// load the bible with the desired translation
let bible = Bible::new(Translation::AmericanStandard);

// get a whole chapter (Isaiah 53 in this case)
// this will return a concatenated string of all verses in the chapter, with superscripts
let chapter_text: String = bible.get_chapter("Isaiah", 53, true).unwrap();

// print the chapter
println!("Isaiah 53: {}", chapter);
```
Get a random verse with the `random` feature enabled:
```rust
use bible_lib::*;

let bible = Bible::new(Translation::default()).unwrap();

// get a random verse
let random_verse = bible.random_verse();

// get the text
let verse_text = bible.get_verse(random_verse.clone(), false).unwrap();

// print the random verse
println!("Random Verse: {} - {}", random_verse, verse_text);
```
Detect bible verses in a string with the `detection` feature enabled:
```rust
use bible_lib::*;

let bible = Bible::new(Translation::default()).unwrap();

let text = "Show me John 3:16";
let verses = BibleLookup::detect_from_string(text);

// this will return a list of all bible verses found in the string,
// so we can iterate over them and print them
for verse in verses {
    let verse_text = bible.get_verse(verse.clone()).unwrap();
    println!("Found verse: {} - {}", verse, verse_text);
}
// (since there is only one verse, this will print only once)
```
