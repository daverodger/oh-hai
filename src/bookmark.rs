use serde::{Deserialize, Serialize};
use nanoid::nanoid;

#[derive(Serialize, Deserialize)]
pub struct Bookmark {
    pub title: String,
    pub command: String,
    id: nanoid!()
}
