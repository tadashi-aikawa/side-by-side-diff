use similar::{ChangeTag, TextDiff};
use unicode_width::UnicodeWidthStr;

pub fn create_side_by_side_diff(text1: &str, text2: &str, max_width: usize) -> String {
    TextDiff::from_lines(text1, text2)
        .iter_all_changes()
        .map(|change| {
            let content = change.to_string().trim_end_matches('\n').to_string();
            let width = max_width - (content.width() - content.chars().count());
            match change.tag() {
                ChangeTag::Delete => format!(
                    "{:>6} | {:<width$} | {:>6} | {:<blank_width$} |",
                    change.old_index().unwrap() + 1,
                    content,
                    "",
                    "",
                    blank_width = max_width,
                    width = width,
                ),
                ChangeTag::Insert => format!(
                    "{:>6} | {:<blank_width$} | {:>6} | {:<width$} |",
                    "",
                    "",
                    change.new_index().unwrap() + 1,
                    content,
                    blank_width = max_width,
                    width = width
                ),
                ChangeTag::Equal => format!(
                    "{:>6} | {token:<width$} | {:>6} | {token:<width$} |",
                    change.old_index().unwrap() + 1,
                    change.new_index().unwrap() + 1,
                    token = content,
                    width = width
                ),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = create_side_by_side_diff(
            "
あああ
bbb
ccc
ddd
いいいいいーeee
かかお
FFFFFFFFFF
ggg"
            .trim_matches('\n'),
            "
あああ
bbb
いいいいいーeee
かかか
ffffffffff
ggg"
            .trim_matches('\n'),
            20,
        );

        let expected = "
     1 | あああ               |      1 | あああ               |
     2 | bbb                  |      2 | bbb                  |
     3 | ccc                  |        |                      |
     4 | ddd                  |        |                      |
     5 | いいいいいーeee      |      3 | いいいいいーeee      |
     6 | かかお               |        |                      |
     7 | FFFFFFFFFF           |        |                      |
       |                      |      4 | かかか               |
       |                      |      5 | ffffffffff           |
     8 | ggg                  |      6 | ggg                  |
"
        .trim_matches('\n');

        assert_eq!(expected, actual);
    }
}
