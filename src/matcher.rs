use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use crate::bookmark::Bookmark;

struct CommandScore(i64, Bookmark);

// Consumes and returns a sorted Vec<Bookmark> from FuzzyMatcher scores based on the input pattern
pub fn sort(commands: Vec<Bookmark>, pattern: &str) -> Vec<Bookmark> {
    let mut bookmark_scores: Vec<CommandScore> = Vec::new(); // Holds fuzzy scores and their associated Bookmark
    let matcher = SkimMatcherV2::default();
    for mut b in commands {
        let mut title_score = None;
        let mut title_index = vec![];
        let mut command_score = None;
        let mut command_index = vec![];

        // Calculate scores and update local vars
        if let Some(tuple) = matcher.fuzzy_indices(&b.title, pattern) {
            title_score = Some(tuple.0);
            title_index = tuple.1;
        }
        if let Some(tuple) = matcher.fuzzy_indices(&b.command, pattern) {
            command_score = Some(tuple.0);
            command_index = tuple.1;
        }

        // Update bookmark highlights
        b.title_highlights = title_index;
        b.command_highlights = command_index;

        // Sum scores and add to list
        let combined_score = title_score.unwrap_or_default() + command_score.unwrap_or_default();
        bookmark_scores.push(CommandScore(combined_score, b));
    }

    // Eliminate zero scores, sort by best match, and return only the Bookmarks
    bookmark_scores = bookmark_scores
        .into_iter()
        .filter(|b| b.0.is_positive())
        .collect();
    bookmark_scores.sort_by(|a, b| a.0.cmp(&b.0).reverse());
    bookmark_scores.into_iter().map(|x| x.1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_reorder() {
        let a = Bookmark::new("banana", "i am a banana");
        let b = Bookmark::new("apple", "not a banana");
        let c = Bookmark::new("gun", "bang");
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
