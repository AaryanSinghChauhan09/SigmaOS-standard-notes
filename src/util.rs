//! standard-notes model — encrypted note-taking app.

#[derive(Debug, Clone)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub encrypted: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub notes: Vec<Note>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { notes: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn create_note(&mut self, title: &str, content: &str) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.notes.push(Note { id: id.clone(), title: title.into(), content: content.into(), encrypted: true });
        id
    }

    pub fn update_note(&mut self, id: &str, title: &str, content: &str) -> bool {
        if let Some(n) = self.notes.iter_mut().find(|n| n.id == id) {
            n.title = title.into();
            n.content = content.into();
            true
        } else { false }
    }

    pub fn list_notes(&self) -> &[Note] { &self.notes }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn note_crud() {
        let mut s = Model::new();
        let id = s.create_note("Secret", "Content");
        assert!(s.update_note(&id, "Updated", "New content"));
    }
}
