use crate::crawler::CrawledFile;
use miette::{miette, IntoDiagnostic, Result};
use pulldown_cmark::{Options, Parser};
use std::{fs, path::PathBuf};

const STYLES: &str = "
    <link rel='stylesheet' href='/assets/styling.css'>
    <link rel='preconnect' href='https://fonts.googleapis.com'>
    <link rel='preconnect' href='https://fonts.gstatic.com' crossorigin>
    <link href='https://fonts.googleapis.com/css2?family=Inter:wght@100..900&family=Roboto:ital,wght@0,100;0,300;0,400;0,500;0,700;0,900;1,100;1,300;1,400;1,500;1,700;1,900&display=swap' rel='stylesheet'>
";

fn create_head() -> String {
    let mut head = String::from("");
    head.push_str("<head>");
    head.push_str("<title>Thesis notes</title>");
    head.push_str(STYLES);
    head.push_str("</head>");

    head
}

fn create_navbar() -> String {
    let mut navbar = String::from("");
    let navbar_header = "<p id='navbar-header'>Thesis notes</p>";

    navbar.push_str("<div id='navbar'>");
    navbar.push_str(navbar_header);
    navbar.push_str("</div>");

    navbar
}

fn create_content(elements: String) -> String {
    let mut content = String::from("");
    content.push_str("<div id='content'>");
    content.push_str(&elements);
    content.push_str("</div>");

    content
}

fn create_page(elements: String) -> String {
    let mut page = String::from("");
    let head = create_head();
    let navbar = create_navbar();
    let content = create_content(elements);

    page.push_str(&head);
    page.push_str("<body>");
    page.push_str("<main>");
    page.push_str(&navbar);
    page.push_str(&content);
    page.push_str("</main>");
    page.push_str("</body>");

    page
}

pub struct Compiler {
    src: PathBuf,
    dst: PathBuf,
}

impl Compiler {
    pub fn new(src: PathBuf, dst: PathBuf) -> Self {
        Self { src, dst }
    }

    pub fn compile(self, files: Vec<CrawledFile>) -> Result<()> {
        if !self.dst.is_dir() {
            fs::create_dir_all(&self.dst).into_diagnostic()?;
        }

        let base_component_count = self.src.components().count();

        for file in files.iter() {
            let input = fs::read_to_string(&file.path).into_diagnostic()?;
            let relative_path: PathBuf =
                file.path.components().skip(base_component_count).collect();
            let mut output_path = self.dst.join(&relative_path);
            output_path.set_extension("html");

            let output_dir = output_path
                .parent()
                .ok_or(miette!("Could not get parent"))?;

            if !output_dir.is_dir() {
                fs::create_dir_all(output_dir).into_diagnostic()?;
            }

            let parser = Parser::new_ext(&input, Options::empty());

            let mut html_content = String::new();
            pulldown_cmark::html::push_html(&mut html_content, parser);

            let page_content = create_page(html_content);

            fs::write(output_path, page_content).into_diagnostic()?;
        }

        Ok(())
    }
}
