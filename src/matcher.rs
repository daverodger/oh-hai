use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::bookmark::Bookmark;

struct CommandScore(i64, Bookmark);

pub fn sort(commands: Vec<Bookmark>, pattern: &str) -> Vec<Bookmark> {
    let mut score_map: Vec<CommandScore> = Vec::new();
    let matcher = SkimMatcherV2::default();
    for mut b in commands {
        let mut title_score = None;
        let mut title_index = vec![];
        let mut command_score = None;
        let mut command_index = vec![];
        if let Some(tuple) = matcher.fuzzy_indices(&b.title, pattern) {
            title_score = Some(tuple.0);
            title_index = tuple.1;
        }
        if let Some(tuple) = matcher.fuzzy_indices(&b.command, pattern) {
            command_score = Some(tuple.0);
            command_index = tuple.1;
        }
        match (title_score, command_score) {
            (Some(score), None) | (None, Some(score)) => {
                b.title_highlights = title_index;
                b.command_highlights = command_index;
                score_map.push(CommandScore(score, b));
            }
            (Some(score1), Some(score2)) => {
                b.title_highlights = title_index;
                b.command_highlights = command_index;
                score_map.push(CommandScore(score1 + score2, b))
            }
            _ => (),
        }
    }
    score_map.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    score_map.into_iter().map(|x| x.1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_reorder() {
        let a = Bookmark::new("banana".to_string(), "i am a banana".to_string());
        let b = Bookmark::new("apple".to_string(), "not a banana".to_string());
        let c = Bookmark::new("gun".to_string(), "bang".to_string());
        let x = vec![a, b, c];
        println!("{:#?}", &x);
        let y = sort(x.clone(), "app");
        println!("{:#?}", &y);
        assert_eq!(&y[0].title, &x[1].title);
        let y = sort(x.clone(), "nan");
        assert_eq!(&y[0].title, &x[0].title);
        let y = sort(x.clone(), "gun");
        assert_eq!(&y[0].title, &x[2].title);
        let matcher = SkimMatcherV2::default();
        assert!(dbg!(matcher.fuzzy_match(&x[0].command, "z")).is_none());
    }
}
