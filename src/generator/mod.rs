use crate::crawler::CrawledFile;
use askama::Template;
use miette::{miette, IntoDiagnostic, Result};
use pulldown_cmark::{Event, LinkType, Options, Parser, Tag};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Template)]
#[template(path = "page.html", escape = "none")]
struct PageTemplate<'a> {
    title: &'a str,
    navigation: &'a str,
    content: &'a str,
}

pub struct Generator {
    src: PathBuf,
    cache: PathBuf,
    name: String,
}

impl Generator {
    pub fn new(src: PathBuf, cache: PathBuf, name: String) -> Self {
        Self { src, cache, name }
    }

    pub fn generate(&self, files: Vec<CrawledFile>) -> Result<PathBuf> {
        let project_cache = self.cache.join(self.name.clone());

        if !project_cache.is_dir() {
            fs::create_dir_all(&project_cache).into_diagnostic()?;
        }

        let base_component_count = self.src.components().count();
        let navigation_html = parse_markdown_file(self.src.join("index.nav"))?;

        for file in files.iter() {
            let relative_path: PathBuf =
                file.path.components().skip(base_component_count).collect();
            let mut output_path = project_cache.join(&relative_path);
            output_path.set_extension("html");
            let output_dir = output_path
                .parent()
                .ok_or(miette!("Could not get parent"))?;

            if !output_dir.is_dir() {
                fs::create_dir_all(output_dir).into_diagnostic()?;
            }

            let file_html = parse_markdown_file(&file.path)?;

            let page_content = PageTemplate {
                title: &self.name,
                content: &file_html,
                navigation: &navigation_html,
            }
            .render()
            .unwrap();

            fs::write(output_path, page_content).into_diagnostic()?;
        }

        Ok(project_cache)
    }
}

fn parse_markdown_file<P: AsRef<Path>>(file_path: P) -> Result<String> {
    let input = fs::read_to_string(file_path).into_diagnostic()?;

    let parser = Parser::new_ext(&input, Options::empty());

    let transformed = parser.map(|event| match event {
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            if link_type == LinkType::Inline {
                let new_dest_url = dest_url.replace(".md", ".html");
                Event::Start(Tag::Link {
                    link_type,
                    dest_url: new_dest_url.into(),
                    title,
                    id,
                })
            } else {
                Event::Start(Tag::Link {
                    link_type,
                    dest_url,
                    title,
                    id,
                })
            }
        }
        other => other,
    });

    let mut html_content = String::new();
    pulldown_cmark::html::push_html(&mut html_content, transformed);

    Ok(html_content)
}
