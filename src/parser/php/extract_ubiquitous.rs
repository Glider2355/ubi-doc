use crate::parser::ubiquitous;

pub fn extract_ubiquitous(comments: Vec<String>) -> Vec<ubiquitous::Ubiquitous> {
    comments
        .into_iter()
        .map(get_ubiquitous)
        .filter(|u| !u.is_all_none())
        .collect()
}

fn get_ubiquitous(comment: String) -> ubiquitous::Ubiquitous {
    let comment = comment.trim();
    let mut result = ubiquitous::Ubiquitous::new();

    for line in comment.lines() {
        let line = line.trim();

        // @ubiquitous
        if let Some(pos) = line.find("@ubiquitous") {
            let tag_len = "@ubiquitous".len();
            if pos + tag_len <= line.len() {
                let val = line[pos + tag_len..].trim().to_string();
                result = result.set_ubiquitous(val);
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

    result
}

#[cfg(test)]
mod tests {
    use super::ubiquitous::Ubiquitous;
    use super::*;

    #[test]
    fn test_extract_ubiquitous_empty() {
        let comments = vec![
            String::from("/* nothing here */"),
            String::from("// hello world"),
        ];

        let result = extract_ubiquitous(comments);
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_ubiquitous_single() {
        let comments = vec![
            String::from("// normal comment"),
            String::from("// @ubiquitous first"),
        ];

        let result = extract_ubiquitous(comments);
        assert_eq!(result.len(), 1);
        assert_eq!(
            result[0],
            Ubiquitous::new().set_ubiquitous("first".to_string())
        );
    }

    #[test]
    fn test_extract_ubiquitous_multi_fields() {
        let comments = vec![String::from(
            r#"/**
 * @ubiquitous ubiquitous_lang
 * @context context_example
 * @description description_text
 */"#,
        )];
        let result = extract_ubiquitous(comments);
        assert_eq!(result.len(), 1);

        let expected = Ubiquitous::new()
            .set_ubiquitous("ubiquitous_lang".to_string())
            .set_context("context_example".to_string())
            .set_description("description_text".to_string());
        assert_eq!(result[0], expected);
    }
}
