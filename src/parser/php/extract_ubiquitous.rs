use crate::parser::ubiquitous::Ubiquitous;

pub struct ExtractUbiquitousParam {
    pub class_name: String,
    pub doc_comment: String,
    pub file_path: String,
    pub line_number: usize,
}

pub fn extract_ubiquitous(class_docs: Vec<ExtractUbiquitousParam>) -> Vec<Ubiquitous> {
    class_docs
        .into_iter()
        .map(get_ubiquitous)
        .filter(|u| !u.is_all_none())
        .collect()
}

fn get_ubiquitous(class_doc: ExtractUbiquitousParam) -> Ubiquitous {
    let comment = class_doc.doc_comment.trim();
    let mut result = Ubiquitous::new();

    result = result.set_class_name(class_doc.class_name);

    for (line_index, line) in comment.lines().enumerate() {
        let line = line.trim();
        let current_line = line_index + class_doc.line_number;

        // @ubiquitous (line_numberも更新)
        if let Some(pos) = line.find("@ubiquitous") {
            let tag_len = "@ubiquitous".len();
            if pos + tag_len <= line.len() {
                let val = line[pos + tag_len..].trim().to_string();
                result = result.set_ubiquitous(val).set_line_number(current_line);
            }
        }
        // @context
        else if let Some(pos) = line.find("@context") {
            let tag_len = "@context".len();
            if pos + tag_len <= line.len() {
                let val = line[pos + tag_len..].trim().to_string();
                result = result.set_context(val);
            }
        }
        // @description
        else if let Some(pos) = line.find("@description") {
            let tag_len = "@description".len();
            if pos + tag_len <= line.len() {
                let val = line[pos + tag_len..].trim().to_string();
                result = result.set_description(val);
            }
        }
    }
    result = result.set_file_path(class_doc.file_path.clone());

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_ubiquitous_empty() {
        let doc_comments = vec![];

        let result = extract_ubiquitous(doc_comments);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_ubiquitous_single() {
        let class_docs = vec![ExtractUbiquitousParam {
            class_name: "class_name".to_string(),
            doc_comment: r#"/**
    * @ubiquitous ubiquitous_lang
    */"#
            .to_string(),
            file_path: "tmp/saple.php".to_string(),
            line_number: 2,
        }];
        let result = extract_ubiquitous(class_docs);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            Ubiquitous::new()
                .set_class_name("class_name".to_string())
                .set_ubiquitous("ubiquitous_lang".to_string())
                .set_file_path("tmp/saple.php".to_string())
                .set_line_number(3)
        );
    }

    #[test]
    fn test_extract_ubiquitous_multi_fields() {
        let class_docs = vec![ExtractUbiquitousParam {
            class_name: "class_name".to_string(),
            doc_comment: r#"/**
    *
    * @ubiquitous ubiquitous_lang
    * @context context_example
    * @description description_text
    */"#
            .to_string(),
            file_path: "tmp/saple.php".to_string(),
            line_number: 3,
        }];
        let result = extract_ubiquitous(class_docs);
        assert_eq!(result.len(), 1);

        let expected = Ubiquitous::new()
            .set_class_name("class_name".to_string())
            .set_ubiquitous("ubiquitous_lang".to_string())
            .set_context("context_example".to_string())
            .set_description("description_text".to_string())
            .set_file_path("tmp/saple.php".to_string())
            .set_line_number(5);
        assert_eq!(result[0], expected);
    }
}
