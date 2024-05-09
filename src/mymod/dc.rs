pub struct Dc {
    s: String,
}

#[allow(unused)]
impl Dc {
    pub fn new(s: String) -> Self {
        Dc { s: s }
    }

    pub fn read(self) {
        println!("{}", self.s)
    }
}
