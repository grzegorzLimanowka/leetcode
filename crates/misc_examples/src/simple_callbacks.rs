//! Simple impl of emitting a callbacks and executing them

#![allow(unused)]

trait WatchTrait {
    fn update(&self);
}

struct Watch {
    name: String,
}

impl Watch {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl WatchTrait for Watch {
    fn update(&self) {
        println!(">>>>>>>>>>>>>>>>>> {}", self.name);
    }
}

trait Observer4T {
    fn emit(&mut self, name: String, callback: Box<dyn WatchTrait + 'static>);
    fn on(&mut self, name: String);
}

struct Observer4 {
    list: Vec<(String, Vec<Box<dyn WatchTrait + 'static>>)>,
}

impl Observer4 {
    fn new() -> Self {
        Self { list: Vec::new() }
    }
}

impl Observer4T for Observer4 {
    fn emit(&mut self, name: String, callback: Box<dyn WatchTrait + 'static>) {
        let mut opt_callback = Some(callback);

        for item in self.list.iter_mut() {
            if item.0 == name {
                if let Some(cb) = opt_callback.take() {
                    item.1.push(cb);
                }
            }
        }

        if let Some(cb) = opt_callback.take() {
            let mut k = Vec::new();
            k.push(cb);
            let l = (name, k);
            self.list.push(l);
        }
    }

    fn on(&mut self, name: String) {
        for item in self.list.iter() {
            if item.0 == name {
                for t in item.1.iter() {
                    t.update()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Observer4, Observer4T, Watch};

    #[test]
    fn testing() {
        let mut o = Observer4::new();

        o.emit("jan".to_owned(), Box::new(Watch::new("name".to_string())));
        o.emit("jan".to_owned(), Box::new(Watch::new("3232".to_owned())));
        o.emit(
            "staszek".to_owned(),
            Box::new(Watch::new("3232".to_owned())),
        );
        o.emit(
            "staszek".to_owned(),
            Box::new(Watch::new("3234".to_owned())),
        );

        o.on("jan".to_owned());
    }
}
