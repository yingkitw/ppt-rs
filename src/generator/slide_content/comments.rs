//! Comments and review annotations for slides
//!
//! Supports adding review comments to slides with author, date, and position.
//! Generates proper OOXML `<p:cmLst>` and `<p:cmAuthorLst>` XML.

use std::collections::HashMap;

/// A comment author
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommentAuthor {
    pub id: u32,
    pub name: String,
    pub initials: String,
    pub color_index: u32,
}

impl CommentAuthor {
    pub fn new(id: u32, name: &str, initials: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            initials: initials.to_string(),
            color_index: id,
        }
    }

    pub fn color_index(mut self, idx: u32) -> Self {
        self.color_index = idx;
        self
    }

    /// Generate XML for `<p:cmAuthor>` element
    pub fn to_xml(&self) -> String {
        format!(
            r#"<p:cmAuthor id="{}" name="{}" initials="{}" lastIdx="1" clrIdx="{}"/>"#,
            self.id,
            xml_escape(&self.name),
            xml_escape(&self.initials),
            self.color_index,
        )
    }
}

/// A single comment on a slide
#[derive(Clone, Debug)]
pub struct Comment {
    pub author_id: u32,
    pub text: String,
    pub date: String,
    pub x: u32,
    pub y: u32,
    pub index: u32,
}

impl Comment {
    /// Create a new comment at a position (x, y in EMU)
    pub fn new(author_id: u32, text: &str) -> Self {
        Self {
            author_id,
            text: text.to_string(),
            date: "2025-01-01T00:00:00.000".to_string(),
            x: 0,
            y: 0,
            index: 1,
        }
    }

    /// Set comment position (in EMU)
    pub fn position(mut self, x: u32, y: u32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Set comment date (ISO 8601 format)
    pub fn date(mut self, date: &str) -> Self {
        self.date = date.to_string();
        self
    }

    /// Set comment index
    pub fn index(mut self, idx: u32) -> Self {
        self.index = idx;
        self
    }

    /// Generate XML for `<p:cm>` element
    pub fn to_xml(&self) -> String {
        format!(
            r#"<p:cm authorId="{}" dt="{}" idx="{}"><p:pos x="{}" y="{}"/><p:text>{}</p:text></p:cm>"#,
            self.author_id,
            xml_escape(&self.date),
            self.index,
            self.x,
            self.y,
            xml_escape(&self.text),
        )
    }
}

/// Manages comment authors across the presentation
#[derive(Clone, Debug, Default)]
pub struct CommentAuthorList {
    authors: Vec<CommentAuthor>,
    name_to_id: HashMap<String, u32>,
    next_id: u32,
}

impl CommentAuthorList {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add or get an author by name. Returns the author ID.
    pub fn get_or_add(&mut self, name: &str, initials: &str) -> u32 {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }
        let id = self.next_id;
        self.next_id += 1;
        let author = CommentAuthor::new(id, name, initials);
        self.authors.push(author);
        self.name_to_id.insert(name.to_string(), id);
        id
    }

    /// Get author by ID
    pub fn get_by_id(&self, id: u32) -> Option<&CommentAuthor> {
        self.authors.iter().find(|a| a.id == id)
    }

    /// Get all authors
    pub fn authors(&self) -> &[CommentAuthor] {
        &self.authors
    }

    /// Number of authors
    pub fn len(&self) -> usize {
        self.authors.len()
    }

    /// Whether the list is empty
    pub fn is_empty(&self) -> bool {
        self.authors.is_empty()
    }

    /// Generate `commentAuthors.xml` content
    pub fn to_xml(&self) -> String {
        let mut xml = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
        );
        xml.push_str(
            r#"<p:cmAuthorLst xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
        );
        for author in &self.authors {
            xml.push_str(&author.to_xml());
        }
        xml.push_str("</p:cmAuthorLst>");
        xml
    }
}

/// Comments for a single slide
#[derive(Clone, Debug, Default)]
pub struct SlideComments {
    comments: Vec<Comment>,
}

impl SlideComments {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a comment
    pub fn add(&mut self, comment: Comment) {
        self.comments.push(comment);
    }

    /// Add a comment with auto-incrementing index
    pub fn add_comment(&mut self, author_id: u32, text: &str, x: u32, y: u32) {
        let idx = self.comments.len() as u32 + 1;
        self.comments.push(
            Comment::new(author_id, text)
                .position(x, y)
                .index(idx),
        );
    }

    /// Get all comments
    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }

    /// Number of comments
    pub fn len(&self) -> usize {
        self.comments.len()
    }

    /// Whether there are no comments
    pub fn is_empty(&self) -> bool {
        self.comments.is_empty()
    }

    /// Generate `commentN.xml` content for a slide
    pub fn to_xml(&self) -> String {
        let mut xml = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#,
        );
        xml.push_str(
            r#"<p:cmLst xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">"#,
        );
        for comment in &self.comments {
            xml.push_str(&comment.to_xml());
        }
        xml.push_str("</p:cmLst>");
        xml
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_author_new() {
        let author = CommentAuthor::new(0, "John Doe", "JD");
        assert_eq!(author.id, 0);
        assert_eq!(author.name, "John Doe");
        assert_eq!(author.initials, "JD");
        assert_eq!(author.color_index, 0);
    }

    #[test]
    fn test_comment_author_color_index() {
        let author = CommentAuthor::new(0, "Jane", "J").color_index(5);
        assert_eq!(author.color_index, 5);
    }

    #[test]
    fn test_comment_author_xml() {
        let author = CommentAuthor::new(0, "John Doe", "JD");
        let xml = author.to_xml();
        assert!(xml.contains(r#"id="0""#));
        assert!(xml.contains(r#"name="John Doe""#));
        assert!(xml.contains(r#"initials="JD""#));
    }

    #[test]
    fn test_comment_new() {
        let comment = Comment::new(0, "Review this slide");
        assert_eq!(comment.author_id, 0);
        assert_eq!(comment.text, "Review this slide");
        assert_eq!(comment.x, 0);
        assert_eq!(comment.y, 0);
    }

    #[test]
    fn test_comment_position() {
        let comment = Comment::new(0, "Note").position(100, 200);
        assert_eq!(comment.x, 100);
        assert_eq!(comment.y, 200);
    }

    #[test]
    fn test_comment_date() {
        let comment = Comment::new(0, "Note").date("2025-06-15T10:30:00.000");
        assert_eq!(comment.date, "2025-06-15T10:30:00.000");
    }

    #[test]
    fn test_comment_xml() {
        let comment = Comment::new(0, "Fix this").position(100, 200).index(1);
        let xml = comment.to_xml();
        assert!(xml.contains(r#"authorId="0""#));
        assert!(xml.contains(r#"idx="1""#));
        assert!(xml.contains(r#"x="100""#));
        assert!(xml.contains(r#"y="200""#));
        assert!(xml.contains("Fix this"));
    }

    #[test]
    fn test_comment_xml_escaping() {
        let comment = Comment::new(0, "Use <b> & \"quotes\"");
        let xml = comment.to_xml();
        assert!(xml.contains("&lt;b&gt;"));
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&quot;quotes&quot;"));
    }

    #[test]
    fn test_comment_author_list_new() {
        let list = CommentAuthorList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_comment_author_list_add() {
        let mut list = CommentAuthorList::new();
        let id1 = list.get_or_add("Alice", "A");
        let id2 = list.get_or_add("Bob", "B");
        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_comment_author_list_dedup() {
        let mut list = CommentAuthorList::new();
        let id1 = list.get_or_add("Alice", "A");
        let id2 = list.get_or_add("Alice", "A");
        assert_eq!(id1, id2);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_comment_author_list_get_by_id() {
        let mut list = CommentAuthorList::new();
        list.get_or_add("Alice", "A");
        let author = list.get_by_id(0);
        assert!(author.is_some());
        assert_eq!(author.unwrap().name, "Alice");
        assert!(list.get_by_id(99).is_none());
    }

    #[test]
    fn test_comment_author_list_xml() {
        let mut list = CommentAuthorList::new();
        list.get_or_add("Alice", "A");
        let xml = list.to_xml();
        assert!(xml.contains("<p:cmAuthorLst"));
        assert!(xml.contains("Alice"));
        assert!(xml.contains("</p:cmAuthorLst>"));
    }

    #[test]
    fn test_slide_comments_new() {
        let comments = SlideComments::new();
        assert!(comments.is_empty());
        assert_eq!(comments.len(), 0);
    }

    #[test]
    fn test_slide_comments_add() {
        let mut comments = SlideComments::new();
        comments.add(Comment::new(0, "First comment").position(10, 20));
        comments.add(Comment::new(1, "Second comment").position(30, 40));
        assert_eq!(comments.len(), 2);
    }

    #[test]
    fn test_slide_comments_add_comment() {
        let mut comments = SlideComments::new();
        comments.add_comment(0, "Auto-indexed", 100, 200);
        comments.add_comment(0, "Second", 300, 400);
        assert_eq!(comments.comments()[0].index, 1);
        assert_eq!(comments.comments()[1].index, 2);
    }

    #[test]
    fn test_slide_comments_xml() {
        let mut comments = SlideComments::new();
        comments.add_comment(0, "Review needed", 100, 200);
        let xml = comments.to_xml();
        assert!(xml.contains("<p:cmLst"));
        assert!(xml.contains("Review needed"));
        assert!(xml.contains("</p:cmLst>"));
    }

    #[test]
    fn test_slide_comments_xml_empty() {
        let comments = SlideComments::new();
        let xml = comments.to_xml();
        assert!(xml.contains("<p:cmLst"));
        assert!(xml.contains("</p:cmLst>"));
    }

    #[test]
    fn test_comment_author_list_xml_multiple() {
        let mut list = CommentAuthorList::new();
        list.get_or_add("Alice", "A");
        list.get_or_add("Bob", "B");
        let xml = list.to_xml();
        assert!(xml.contains("Alice"));
        assert!(xml.contains("Bob"));
        assert!(xml.matches("<p:cmAuthor ").count() == 2);
    }
}
