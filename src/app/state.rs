use super::{App, Idea};

pub trait Handler<'a> {
    fn add_idea(&mut self, title: &str, description: &str);
    fn remove_idea(&mut self, index: usize);
    fn save(&mut self);
    fn clear_buffer(&mut self);
    fn quit(&mut self);
    fn save_edit(&mut self);
    fn load_buffer(&mut self) -> bool;
}

impl<'a> Handler<'a> for App<'a> {
    fn add_idea(&mut self, title: &str, description: &str) {
        self.ideas.push(Idea::new(title, description));
    }

    fn remove_idea(&mut self, index: usize) {
        self.ideas.remove(index);
    }

    fn save(&mut self) {
        let string = serde_json::to_string_pretty(&self.ideas).unwrap();
        std::fs::write(&self.path, string).unwrap();
    }

    fn clear_buffer(&mut self) {
        self.buffer[0].clear();
        self.buffer[1].clear();
    }

    fn quit(&mut self) {
        self.exit = true;
    }

    fn save_edit(&mut self) {
        let index = self.active_index;
        if self.ideas.len() > 0 {
            self.ideas[index].title = self.buffer[0].clone();
            self.ideas[index].description = self.buffer[1].clone();
        }
    }
    fn load_buffer(&mut self) -> bool {
        let idea = self.ideas.get(self.active_index);
        if let Some(dea) = idea {
            self.buffer[0] = dea.title.clone();
            self.buffer[1] = dea.description.clone();
            return true;
        }
        false
    }
}
