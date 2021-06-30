#[derive(Default)]
pub struct UserCache {
    map: Vec<(String, i64)>,
}

impl UserCache {
    pub fn insert(&mut self, name: impl Into<String>, id: i64) {
        self.map.push((name.into(), id));
    }

    pub fn contains_id(&self, id: i64) -> bool {
        self.map.iter().any(|&(_, v)| v == id)
    }

    pub fn update_name(&mut self, id: i64, name: impl Into<String>) {
        if let Some((k, _)) = self.map.iter_mut().find(|(_, v)| *v == id) {
            *k = name.into()
        } else {
            self.insert(name, id)
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.map.iter().any(|(k, _)| k.eq_ignore_ascii_case(name))
    }

    pub fn get(&self, id: i64) -> Option<&str> {
        self.map
            .iter()
            .find_map(|(k, v)| (*v == id).then(|| k.as_str()))
    }
}
