use chrono::NaiveDate;
use comrak::{markdown_to_html, ComrakOptions};
use std::cmp::Ordering;
use std::fs::{read_dir, File};
use std::io::prelude::*;
use std::io::BufReader;
use tracing::{error, info};

pub enum Template {
    Header,
    Footer,
}

impl Template {
    /// reads a template file and loads into memory
    pub fn load(&self) -> Result<String, std::io::Error> {
        let f = File::open(match self {
            Self::Header => {
                info!("opening file: templates/header.html");
                "templates/header.html".to_string()
            }
            Self::Footer => {
                info!("opening file: templates/footer.html");
                "templates/footer.html".to_string()
            }
        })?;
        let mut buf = BufReader::new(f);
        let mut contents = String::new();
        buf.read_to_string(&mut contents)?;
        Ok(contents)
    }
}

#[derive(Debug)]
pub enum Blog {
    Menu,
    Home,
    About,
    Contact,
    Articles(String),
}

impl Blog {
    /// reads a file and loads into memory
    pub fn load(&self) -> Result<String, std::io::Error> {
        let f = File::open(match self {
            Self::Menu => {
                info!("opening file: static/menu.md");
                "static/menu.md".to_string()
            }
            Self::Home => {
                info!("opening file: static/home.md");
                "static/home.md".to_string()
            }
            Self::About => {
                info!("opening file: static/about.md");
                "static/about.md".to_string()
            }
            Self::Contact => {
                info!("opening file: static/contact.md");
                "static/contact.md".to_string()
            }
            Self::Articles(p) => {
                info!("opening file: {}", p);
                format!("./articles/{}", p)
            }
        })?;
        let mut buf = BufReader::new(f);
        let mut contents = String::new();
        buf.read_to_string(&mut contents)?;
        Ok(markdown_to_html(&contents, &ComrakOptions::default()))
    }

    /// builds the page up from template files
    /// seperate match for home page, as it is unique.
    /// Everything else uses the same format.
    pub fn build(&self) -> Result<String, std::io::Error> {
        match self {
            Self::Home => {
                info!("opening file: {:?}", self);
                let articles = Article::generate()?;
                let mut series = String::new();
                for article in articles {
                    series += &format!(
                        r#"<a href="{}"><h2>{}</h2></a><p>{}</p><p>{}...</p>"#,
                        article.uri,
                        article.title,
                        article.date,
                        &article.content[0..200]
                    )
                    .to_owned();
                }
                let page = format!(
                    "{}{}{}{}",
                    Template::Header.load()?,
                    self.load()?,
                    series,
                    Template::Footer.load()?
                );
                Ok(page)
            }
            _ => {
                info!("opening file: {:?}", self);
                let page = format!(
                    "{}{}{}",
                    Template::Header.load()?,
                    self.load()?,
                    Template::Footer.load()?
                );
                Ok(page)
            }
        }
    }
}

#[derive(Debug, Eq)]
pub struct Article {
    pub title: String,
    pub date: String,
    pub content: String,
    pub uri: String,
}

impl Article {
    /// reads through articles folder and initializes a vector of type Article
    pub fn generate() -> Result<Vec<Article>, std::io::Error> {
        info!("generating articles");
        let paths = read_dir("./articles").expect("Unable to read directory");
        let mut articles = Vec::with_capacity(10);
        for path in paths {
            articles.push(Article::summarize(
                &path.unwrap().path().display().to_string(),
            )?);
        }
        articles.sort();
        articles.reverse();
        Ok(articles)
    }

    /// composes the article page, using templates or
    /// provides a standard error message page
    pub fn build(page: &str) -> Result<String, anyhow::Error> {
        let header = Template::Header.load().expect("Unable to load template");
        let footer = Template::Footer.load().expect("Unable to load template");
        let article = Article::summarize(&format!("{}", page));
        match article {
            Ok(page) => {
                info!("article | response: found");
                Ok(format!(
                    "{}<h1>{}</h1>\n{}\n{}{}",
                    header, page.title, page.date, page.content, footer
                ))
            }
            Err(_) => {
                error!("article | response: error");
                Ok(format!(
                    "{}<h2>Oops, this shit is bananas</h2>{}",
                    header, footer
                ))
            }
        }
    }

    /// takes a file path and converts it to a type article
    pub fn summarize(path: &str) -> Result<Article, std::io::Error> {
        info!("opening file: {}", path);
        let f = File::open(path)?;
        let mut buf = BufReader::new(f);
        // create a temporary file to store unwanted lines.
        // haven't really found another way to do this.
        let mut _tmp = String::new();
        let mut title = String::new();
        let mut date = String::new();
        let mut raw_content = Vec::new();

        buf.read_line(&mut _tmp)?;
        buf.read_line(&mut title)?;
        buf.read_line(&mut date)?;
        buf.read_line(&mut _tmp)?;
        buf.read_line(&mut _tmp)?;

        title = title.split("title: ").collect::<String>();
        date = date.split("date: ").collect::<String>();

        buf.read_to_end(&mut raw_content)?;

        // pop the newline character off, so we can parse date correctly
        date.pop();
        date = NaiveDate::parse_from_str(&date, "%Y%m%d")
            .expect("Unable to parse date")
            .format("%d %B %Y")
            .to_string();

        let content = markdown_to_html(
            &String::from_utf8(raw_content).unwrap().to_owned(),
            &ComrakOptions::default(),
        );

        Ok(Article {
            title,
            date,
            content,
            uri: path.to_string(),
        })
    }
}

/// implementation of Eq so that we can sort articles
impl Ord for Article {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date.cmp(&other.date)
    }
}

impl PartialOrd for Article {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.date == other.date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_template_load() {
        //arrange
        //act
        let header = Template::Header.load();
        let footer = Template::Footer.load();

        //assert
        assert!(header.is_ok());
        assert!(footer.is_ok());
    }

    #[test]
    pub fn test_blog_load() {
        //arrange
        //act
        let home = Blog::Home.load();

        //assert
        assert!(home.is_ok());
    }

    #[test]
    pub fn test_blog_build() {
        //arrange
        //act
        let home = Blog::Home.build();
        let about = Blog::About.build();

        //assert
        assert!(home.is_ok());
        assert!(about.is_ok());
    }

    #[test]
    pub fn test_article_generate() {
        //arrange
        let count = read_dir("./articles").unwrap().count();

        //act
        let articles = Article::generate();

        //assert
        assert_eq!(articles.unwrap().iter().count(), count);
    }

    #[test]
    pub fn test_article_summarize() {
        //arrange
        let path = "tests/test_file.md".to_string();

        //act
        let article = Article::summarize(&path).unwrap();

        //assert
        assert_eq!(article.title, "Test\n");
        assert_eq!(article.date, "17 July 2021");
        assert_eq!(article.content, "<p>abc</p>\n");
    }
}
