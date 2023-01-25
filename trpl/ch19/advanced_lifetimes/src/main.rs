trait Red {
    fn show(&self) -> String;
}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {
    fn show(&self) -> String {
        format!("{}cm", self.diameter)
    }
}

fn main() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;

    println!("{}", obj.show());
}