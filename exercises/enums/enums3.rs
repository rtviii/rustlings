
enum Message {
    ChangeColor(u8, u8, u8),
    Echo(String),
    Move { x: u8, y: u8 },
    Quit,
}
struct Point {
    x: u8,
    y: u8,
}
struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}
impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }
    fn quit(&mut self) {
        self.quit = true;
    }
    fn echo(&self, s: String) {
        println!("{}", s);
    }
    fn move_position(&mut self, p: Point) {
        self.position = p;
    }
    fn process(&mut self, message: Message) {
        // TODO: create a match expression to process the different message variants
        match message {
            Message::ChangeColor(x, y, z) => self.color = (x, y, z),
            Message::Echo(x) => println!("{}", x),
            Message::Move { x, y } => self.position = Point { x: x, y: y },
            Message::Quit => self.quit(),
        }
    }
}

