pub enum Language {
    Php(CommentOutSyntax),
    // Rust(CommentOutSyntax),
    // Java(CommentOutSyntax),
}

pub struct CommentOutSyntax {
    single_line_patterns: Vec<String>,
    doc: Option<DocCommentSyntax>,
}

#[derive(Debug)]
pub struct DocCommentSyntax {
    pub start: String,
    pub middle: String,
    pub end: String,
}

impl Language {
    pub fn new_php() -> Self {
        Language::Php(CommentOutSyntax {
            single_line_patterns: vec!["//".to_string(), "#".to_string()],
            doc: Some(DocCommentSyntax {
                start: "/**".to_string(),
                middle: "*".to_string(),
                end: "*/".to_string(),
            }),
        })
    }

    pub fn syntax(&self) -> &CommentOutSyntax {
        match self {
            Language::Php(syntax) => syntax,
        }
    }
}

impl CommentOutSyntax {
    pub fn single_line_patterns(&self) -> &Vec<String> {
        &self.single_line_patterns
    }

    pub fn doc_comment(&self) -> Option<&DocCommentSyntax> {
        self.doc.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_php_doc_syntax() {
        let php = Language::new_php();
        let doc = php.syntax().doc_comment().unwrap();
        assert_eq!(doc.start, "/**");
        assert_eq!(doc.middle, "*");
        assert_eq!(doc.end, "*/");
    }

    #[test]
    fn test_php_single_line_syntax() {
        let php = Language::new_php();
        assert_eq!(php.syntax().single_line_patterns(), &["//", "#"]);
    }
}
