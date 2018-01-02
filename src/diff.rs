pub struct Diff {
    left: String,
    right: String,
}

impl Diff {
    pub fn new(left: String, right: String) -> Diff {
        Diff {
            left: left,
            right: right,
        }
    }

    pub fn exec(&self) -> String {
        println!("{}", self.left);
        println!("{}", self.right);
        "".to_string() // Dummy
    }
}
