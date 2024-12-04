use crate::cache::Cache;
use askama::Template;
use miette::{miette, IntoDiagnostic, Result};
use pulldown_cmark::{Event, LinkType, Options, Parser, Tag};
use std::{
    fs,
    path::{Path, PathBuf},
};

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

#[derive(Template)]
#[template(path = "page.html", escape = "none")]
struct PageTemplate<'a> {
    title: &'a str,
    navigation: &'a str,
    content: &'a str,
    refresh: bool,
}

pub struct Generator {
    src: PathBuf,
    cache: Cache,
    name: String,
    nav_file_path: PathBuf,
    nav_html: String,
    refresh: bool,
}

impl Generator {
    pub fn new(src: PathBuf, cache_path: PathBuf, name: String, refresh: bool) -> Result<Self> {
        let cache = Cache::new(cache_path.join(&name))?;
        let nav_file_path = src.join("index.nav");
        let nav_html = parse_markdown_file(&nav_file_path)?;

        Ok(Self {
            src,
            cache,
            name,
            nav_file_path,
            nav_html,
            refresh,
        })
    }

    pub fn generate_file<P: AsRef<Path>>(
        &self,
        input_file_path: P,
        mut output_path: PathBuf,
    ) -> Result<()> {
        output_path.set_extension("html");
        let output_dir = output_path
            .parent()
            .ok_or(miette!("Could not get parent"))?;

        if !output_dir.is_dir() {
            fs::create_dir_all(output_dir).into_diagnostic()?;
        }

        if let Ok(file_html) = parse_markdown_file(&input_file_path) {
            let page_content = PageTemplate {
                title: &self.name,
                content: &file_html,
                navigation: &self.nav_html,
                refresh: self.refresh,
            }
            .render()
            .unwrap();

            fs::write(output_path, page_content).into_diagnostic()?;
        } else {
            let t = input_file_path.as_ref();
            println!("Could not parse: {t:?}");
        }

        Ok(())
    }

    fn regenerate(&mut self) {
        let base_component_count = self.src.components().count();

        for file_path in self.cache.files().iter() {
            if file_path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                let relative_path: PathBuf =
                    file_path.components().skip(base_component_count).collect();
                let output_path = self.cache.get_path().join(&relative_path);

                self.generate_file(file_path, output_path).unwrap();
                self.cache.update(file_path);
            }
        }
    }

    pub fn generate(&mut self, files_paths: &[PathBuf]) -> Result<()> {
        let base_component_count = self.src.components().count();

        for file_path in files_paths.iter() {
            if *file_path == self.nav_file_path {
                let nav_html = parse_markdown_file(&self.nav_file_path)?;
                self.nav_html = nav_html;
                self.regenerate();
                break;
            }

            if file_path.extension().and_then(|ext| ext.to_str()) == Some("md")
                && self.cache.modified(file_path)
            {
                let relative_path: PathBuf =
                    file_path.components().skip(base_component_count).collect();
                let output_path = self.cache.get_path().join(&relative_path);

                self.generate_file(file_path, output_path).unwrap();
                self.cache.update(file_path);
            }
        }

        Ok(())
    }
}
