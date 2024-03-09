use mksubrip::core;

struct LineEdit {
    pub sig_value_changed: core::Signal<String>,
    text: String,
}

impl LineEdit {
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();

        self.sig_value_changed.emit(&self.text);
    }
}

struct Screen {}

impl Screen {
    pub fn print(&mut self, text: &String) {
        println!("Screen::print {}", text);
    }
}

fn print(text: &String) {
    println!("print {}", text);
}

fn main() {
    let mut line = LineEdit {
        sig_value_changed: core::Signal::new(),
        text: String::new(),
    };
    let screen = core::Shared::new(Screen {});

    line.sig_value_changed.connect_func(print);
    line.sig_value_changed
        .connect_method(screen.clone(), Screen::print);

    line.set_text("Hello world!!!");
}
