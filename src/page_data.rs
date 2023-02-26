use crate::error::Error;

#[derive(Debug)]
pub struct PageData {
    frontmatter: toml::Value,
    content: String,
}

impl PageData {
    pub fn get_value(&self, key: &str) -> Result<&toml::Value, Error> {
        self.frontmatter
            .get(key)
            .ok_or_else(|| Error::Favia(format!("key {key} not found")))
    }
}

impl TryFrom<String> for PageData {
    type Error = Error;

    fn try_from(markdown: String) -> Result<Self, Error> {
        let frontmatter_start = markdown.find("+++");
        let frontmatter_end = markdown.rfind("+++");

        let (frontmatter, content) = match (frontmatter_start, frontmatter_end) {
            (Some(start), Some(end)) => (
                markdown[start + 3..end].to_string(),
                markdown[end + 3..].to_string(),
            ),
            _ => (String::from(""), markdown),
        };

        Ok(Self {
            frontmatter: frontmatter.parse()?,
            content: markdown::to_html(&content),
        })
    }
}

impl From<PageData> for tera::Context {
    fn from(markdown: PageData) -> Self {
        let mut context = tera::Context::new();
        context.insert("content", &markdown.content);
        let value = &tera::to_value(markdown.frontmatter).expect("parse from toml to tera::Value");
        context.insert("fm", value);
        context
    }
}
