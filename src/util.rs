//! spectacle model — KDE screenshot tool.

#[derive(Debug, Clone)]
pub struct Capture {
    pub id: String,
    pub path: String,
    pub region: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub captures: Vec<Capture>,
    pub next_id: u64,
}

impl Default for Model {
    fn default() -> Self { Self { captures: vec![], next_id: 1 } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn capture(&mut self, path: &str, region: &str, timestamp: u64) -> String {
        let id = format!("{:x}", self.next_id);
        self.next_id += 1;
        self.captures.push(Capture { id: id.clone(), path: path.into(), region: region.into(), timestamp });
        id
    }

    pub fn delete(&mut self, id: &str) -> bool {
        if let Some(pos) = self.captures.iter().position(|c| c.id == id) {
            self.captures.remove(pos);
            true
        } else { false }
    }

    pub fn list(&self) -> &[Capture] { &self.captures }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn capture_delete() {
        let mut s = Model::new();
        let id = s.capture("/tmp/shot.png", "full", 1000);
        assert_eq!(s.list().len(), 1);
        assert!(s.delete(&id));
    }
}
