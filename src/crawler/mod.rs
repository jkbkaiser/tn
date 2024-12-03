use miette::{IntoDiagnostic, Result};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct CrawledFile {
    pub path: PathBuf,
}

#[derive(Debug)]
pub enum CrawledEntry {
    File(CrawledFile),
    Dir(CrawledDir),
}

#[derive(Debug)]
pub struct CrawledDir {
    pub entries: Vec<CrawledEntry>,
}

fn recursive_crawl<P: AsRef<Path>>(path: P) -> Result<CrawledDir> {
    let mut elements = Vec::new();
    let dir = fs::read_dir(path.as_ref()).into_diagnostic()?;

    for entry in dir {
        let entry = entry.into_diagnostic()?;
        let path = entry.path();
        let metadata = entry.metadata().into_diagnostic()?;

        if metadata.is_file() && path.extension() == Some("md".as_ref()) {
            let crawled_file = CrawledFile {
                path: path.canonicalize().into_diagnostic()?,
            };
            elements.push(CrawledEntry::File(crawled_file));
        } else if metadata.is_dir() {
            let crawled_dir = recursive_crawl(path.canonicalize().into_diagnostic()?)?;
            elements.push(CrawledEntry::Dir(crawled_dir));
        }
    }

    Ok(CrawledDir { entries: elements })
}

fn flatten(dir: CrawledDir) -> Vec<CrawledFile> {
    let mut elements = Vec::new();

    for entry in dir.entries {
        match entry {
            CrawledEntry::File(crawled_file) => elements.push(crawled_file),
            CrawledEntry::Dir(crawled_dir) => elements.extend(flatten(crawled_dir)),
        }
    }

    elements
}

pub fn crawl<P: AsRef<Path>>(path: P) -> Result<Vec<CrawledFile>> {
    Ok(flatten(recursive_crawl(path)?))
}
