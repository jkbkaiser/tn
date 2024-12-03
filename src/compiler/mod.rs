use crate::crawler::CrawledFile;
use miette::{miette, IntoDiagnostic, Result};
use pulldown_cmark::{Options, Parser};
use std::{fs, path::PathBuf};

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

            let mut html_output = String::new();

            let test = "<style>
body {
  background-color: linen;
}

h1 {
  color: maroon;
  margin-left: 40px;
}
</style>";

            html_output.push_str(test);

            pulldown_cmark::html::push_html(&mut html_output, parser);

            fs::write(output_path, html_output).into_diagnostic()?;
        }

        Ok(())
    }
}
