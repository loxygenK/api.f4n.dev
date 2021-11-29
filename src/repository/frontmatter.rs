use crate::domain::blog::BlogHeader;

pub fn parse_blog(content: &str) -> Option<BlogHeader> {
    let frontmatter_candicates: Vec<&str> = content.split("---").collect();
    if frontmatter_candicates.len() < 3 {
        return None;
    }

    let frontmatter = frontmatter_candicates[1];
    let blog_info = serde_yaml::from_str(frontmatter).ok()?;

    Some(blog_info)
}
