pub struct Skya {
    data: Vec<Pair>,
    keys: Vec<String>,
}

pub fn load_skya(text: &str) -> Skya {
    let pairs: Vec<&str> = text.split("||").collect();
    let mut data: Skya = Skya::new();
    for pair in pairs {
        let units: Vec<&str> = pair.split("::").collect();
        if units.len() != 2 {
            panic!("Invalid Skya");
        } else {
            data.push(Pair {key: String::from(*units.get(0).unwrap()), value: String::from(*units.get(1).unwrap())});
        }
    }
    data
}

#[derive(Clone)]
pub struct Pair {
    key: String,
    value: String,
}

pub fn dump_skya(data: Skya) -> String {
    let mut rstr = String::from("");
    for pair in data {
        rstr.push_str(&pair.key);
        rstr.push_str("::");
        rstr.push_str(&pair.value);
        rstr.push_str("||")
    }
    if rstr.len() > 2 {
        for _ in 0..2 {
            rstr.pop();
        }
    }
    rstr
}

impl Pair {
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn new(key: &str, value: &str) -> Self {
        Pair { key: String::from(key), value: String::from(value) }
    }
}

impl Skya {
    fn new() -> Self {
        Skya { data: Vec::new(), keys: Vec::new() }
    }
    fn push(&mut self, pair: Pair) {
        if self.contains(pair.key()) {
            self.remove(pair.key())
        }
        self.keys.push(String::from(pair.key()));
        self.data.push(pair);
    }
    pub fn add(&mut self, key: &str, value: &str) {
        self.push( Pair::new(key, value));
    }
    // Shorthand for Skya.keys.contains
    fn contains(&self, key: &str) -> bool {
        self.keys.contains(&String::from(key))
    }
    pub fn remove(&mut self, key: &str) {
        let s = self.data.clone();
        for (i, p) in s.into_iter().enumerate() {
            if p.key() == key {
                self.data.remove(i);
            }
        }
    }
    pub fn get(&self, key: &str) -> Option<String> {
        if self.contains(key) {
            let mut found = None;
            for p in &self.data {
                if p.key() == key {
                    found = Some(String::from(p.value()));
                    break;
                }
            }
            found
        } else {
            None
        }
    }
 }

impl IntoIterator for Skya {
    type Item = Pair;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}