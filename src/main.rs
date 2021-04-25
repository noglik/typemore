mod reader;
mod writer;

fn main() {
    let text = reader::read_std_in();
    writer::read_keys(text)
}
