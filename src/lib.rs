/// That this function returns a vector of slices of
/// `text`, split by `delimiter`.
///
/// In this case, we know that the vector will only ever
/// reference `text`, never `delimiter`.
///
/// This example will always work:
///
/// ```rust
/// use rust::split;
/// let text = String::from("this is a test");
/// let delimiter = String::from(" ");
/// let splitted = split(&text, &delimiter);
/// assert_eq!(splitted, vec!["this", "is", "a", "test"]);
/// ```
///
/// But this example will only work if the lifetimes are correct:
///
/// ```rust
/// use rust::split;
/// let text = String::from("this is a test");
/// let splitted = {
///     let delimiter = String::from(" ");
///     split(&text, &delimiter)
///     // delimiter is dropped here.
/// };
/// // text exists
/// // delimitier is destroyed
/// assert_eq!(splitted, vec!["this", "is", "a", "test"]);
/// ```
pub fn split() {
    todo!();
}

/*
pub fn split(text: &str, delimiter: &str) -> Vec<&str> {
    let mut last_split = 0;
    let mut matches: Vec<&str> = vec![];
    for i in 0..text.len() {
        if i < last_split {
            continue;
        }
        if text[i..].starts_with(delimiter) {
            matches.push(&text[last_split..i]);
            last_split = i + delimiter.len();
        }
    }
    if last_split < text.len() {
        matches.push(&text[last_split..]);
    }

    matches
}
*/
