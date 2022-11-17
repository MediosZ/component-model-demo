wit_bindgen_guest_rust::generate!("../wits/markdown.wit");
use pulldown_cmark::{html, Parser};

struct Markdown;
// remember to export markdown.
export_markdown!(Markdown);

impl markdown::Markdown for Markdown {
    fn render(input: String) -> String {
        let parser = Parser::new(&input);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        return html_output;
    }
}
