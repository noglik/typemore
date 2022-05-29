mod reader;
mod writer;
mod story;

fn main() {
    let text = reader::read_std_in();
    let story = story::Story::new(&text);
    let mut view = writer::View::new(story);
    view.read();
    view.cleanup();
    // gotta finish it with empty line, to avoid % at the end
    println!();
}
