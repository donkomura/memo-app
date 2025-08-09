use crate::domain::model::Note;

impl Note {
    pub fn is_owner(&self, user_id: i64) -> bool {
        self.author_id == user_id
    }
}
