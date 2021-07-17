use actix_web::HttpResponse;
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
                p.to_string()
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
    pub fn build(page: &str) -> Result<HttpResponse, HttpResponse> {
        let header = Template::Header.load().expect("Unable to load template");
        let footer = Template::Footer.load().expect("Unable to load template");
        let article = Article::summarize(&format!("{}", page));
        match article {
            Ok(page) => {
                info!("article | response: found");
                Ok(HttpResponse::Ok().body(format!(
                    "{}<h1>{}</h1>\n{}\n{}{}",
                    header, page.title, page.date, page.content, footer
                )))
            }
            Err(_) => {
                error!("article | response: error");
                Err(HttpResponse::NotFound().body(format!(
                    "{}<h2>Oops, this shit is bananas</h2>{}",
                    header, footer
                )))
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
        let header = Template::Header.load().unwrap();
        let footer = Template::Footer.load().unwrap();

        //assert
        assert_eq!(header, "<html>\n    <head>\n        <title>jereee.me</title>\n        <link rel=\"stylesheet\" href=\"/assets/styles.css\">\n        <meta charset=\"utf-8\">\n        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n    </head>\n    <body>\n\t    <div class=topnav><p class=header>jereee.me | <a href=\"/\">home</a> | <a href=\"/about\">about</a> | <a href=\"/contact\">contact</a> | <a href=\"https://github.com/anomalyze\">github</a> | <a href=\"https://sr.ht/~anomaly/\">sourcehut</a></p>\n        </div>\n        <div class=container>\n            <div class=side></div>\n            <div class=child>\n");
        assert_eq!(footer, "            </div>\n            <div class=side></div>\n        </div>\n    </body>\n</html>\n");
    }

    #[test]
    pub fn test_blog_load() {
        //arrange
        //act
        let home = Blog::Home.load().unwrap();

        //assert
        assert_eq!(home,"<p>ramblings from working in infosec, red teaming, fun with automation and homelab adventures.</p>\n");
    }

    #[test]
    pub fn test_blog_build() {
        //arrange
        //act
        let home = Blog::Home.build().unwrap();
        let about = Blog::About.build().unwrap();

        //assert
        assert_eq!(home,"<html>\n    <head>\n        <title>jereee.me</title>\n        <link rel=\"stylesheet\" href=\"/assets/styles.css\">\n        <meta charset=\"utf-8\">\n        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n    </head>\n    <body>\n\t    <div class=topnav><p class=header>jereee.me | <a href=\"/\">home</a> | <a href=\"/about\">about</a> | <a href=\"/contact\">contact</a> | <a href=\"https://github.com/anomalyze\">github</a> | <a href=\"https://sr.ht/~anomaly/\">sourcehut</a></p>\n        </div>\n        <div class=container>\n            <div class=side></div>\n            <div class=child>\n<p>ramblings from working in infosec, red teaming, fun with automation and homelab adventures.</p>\n<a href=\"./articles/20210715-Only-the-beginning.md\"><h2>Only the beginning...\n</h2></a><p>15 July 2021</p><p><p>Well...I've finally done it. I've actually got around to building my first blog. Now the question lies &quot;What to write about?&quot;. Well since we're only starting this journey, how about we st...</p><a href=\"./articles/20210713-Blog-Two.md\"><h2>Blog Two\n</h2></a><p>13 July 2021</p><p><p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas risus massa, commodo id blandit ut, rhoncus ac nisl. Nullam vitae venenatis tellus. Etiam malesuada a magna accumsan laoreet. Proin...</p><a href=\"./articles/20210712-Blog-Juan.md\"><h2>Blog juan\n</h2></a><p>12 July 2021</p><p><p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas risus massa, commodo id blandit ut, rhoncus ac nisl. Nullam vitae venenatis tellus. Etiam malesuada a magna accumsan laoreet. Proin...</p>            </div>\n            <div class=side></div>\n        </div>\n    </body>\n</html>\n");
        assert_eq!(about,"<html>\n    <head>\n        <title>jereee.me</title>\n        <link rel=\"stylesheet\" href=\"/assets/styles.css\">\n        <meta charset=\"utf-8\">\n        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n    </head>\n    <body>\n\t    <div class=topnav><p class=header>jereee.me | <a href=\"/\">home</a> | <a href=\"/about\">about</a> | <a href=\"/contact\">contact</a> | <a href=\"https://github.com/anomalyze\">github</a> | <a href=\"https://sr.ht/~anomaly/\">sourcehut</a></p>\n        </div>\n        <div class=container>\n            <div class=side></div>\n            <div class=child>\n<h2>about</h2>\n<p>The names Jeremy, I've been working in InfoSec since 2015, working mostly in red teaming these days. I spend my free time automating all the things, and forever learning. If you want to learn a little bit more about me you can check out my first blog post <a href=\"/articles/20210715-Only-the-beginning.md\">here</a></p>\n<h2>work experience</h2>\n<p>Lead Security Consultant - Jul 2020 - current</p>\n<p>Security Consultant - Jun 2016 - Jun 2020</p>\n<p>Junior Security Consultant - Aug 2015 - Jun 2016</p>\n<h2>notable projects</h2>\n<ul>\n<li>Automated red team infrastructure - Terraform, AWS, Docker, Wireguard\n<ul>\n<li>Worked on this project in collaboration with another colleague. It automates all our tooling used for red teams. This includes all phishing infrastructure, including mail servers, dns, redirectors, command and control, and vpns. Unfortunately, I'm not at liberty to share this one with you all.</li>\n</ul>\n</li>\n<li><a href=\"https://github.com/anomalyze/bookmarkd\">bookmarkd</a> - Rust, Docker\n<ul>\n<li>I built this little daemon because I was getting annoyed that I couldn't save bookmarks on one machine and not access them on another. It basically just sits on the network, and I can access it via a CLI tool which I also build. This was mainly just to find a project to work on to learn rust.</li>\n</ul>\n</li>\n<li><a href=\"https://github.com/anomalyze/jereeeme\">jereee.me</a> - Rust\n<ul>\n<li>Thie site is written in 90% rust, while the remaining is just a bit of HTML, CSS and lots of Markdown. This was another project I built, mostly just to learn Rust, but I thoroughly enjoyed trying to figure out how to create a template engine and minimise writing as much HTML, CSS as possible. There's probably a few bugs here and there, but it's only the first iteration.</li>\n</ul>\n</li>\n</ul>\n            </div>\n            <div class=side></div>\n        </div>\n    </body>\n</html>\n");
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
    pub fn test_article_build() {
        //arrange
        let page = "tests/test_file.md".to_string();

        //act
        let article = Article::build(&page).unwrap();

        //assert
        assert_eq!(article.status(), 200);
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
