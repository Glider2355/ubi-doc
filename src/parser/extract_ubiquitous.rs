use super::ubiquitous;

/// コメントのリストから `Ubiquitous` を抽出した Vec を返す。
/// 「すべてのフィールドが None」のレコードは除外する例。
pub fn extract_ubiquitous(comments: Vec<String>) -> Vec<ubiquitous::Ubiquitous> {
    comments
        .into_iter()
        // 各コメントを Ubiquitous 構造体にパース
        .map(get_ubiquitous)
        // まったく何もタグがない (すべて None) の場合は除外
        .filter(|u| !u.is_all_none())
        .collect()
}

/// 1つのコメント文字列から
/// - @ubiquitous
/// - @context
/// - @description
/// を抽出して `Ubiquitous` 構造体に詰め込む。
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
    use super::*;
    use super::ubiquitous::Ubiquitous;

    #[test]
    fn test_extract_ubiquitous_empty() {
        // どのコメントもタグがない
        let comments = vec![
            String::from("/* nothing here */"),
            String::from("// hello world"),
        ];

        let result = extract_ubiquitous(comments);
        // すべて None の構造体しか生まれない → 除外されて 0 件
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_ubiquitous_single() {
        // 1つだけ @ubiquitous がある
        let comments = vec![
            String::from("// normal comment"),
            String::from("// @ubiquitous first"),
        ];

        let result = extract_ubiquitous(comments);
        // 1個だけ抽出されるはず
        assert_eq!(result.len(), 1);
        // その中身をチェック
        assert_eq!(
            result[0],
            Ubiquitous::new().set_ubiquitous("first".to_string())
        );
    }

    #[test]
    fn test_extract_ubiquitous_multi_fields() {
        // 複数のタグを含むコメント
        let comments = vec![String::from(
r#"/**
 * @ubiquitous ubiquitous_lang
 * @context context_example
 * @description description_text
 */"#,
        )];
        let result = extract_ubiquitous(comments);
        // 1個だけ抽出される
        assert_eq!(result.len(), 1);

        // 中身をチェック
        let expected = Ubiquitous::new()
            .set_ubiquitous("ubiquitous_lang".to_string())
            .set_context("context_example".to_string())
            .set_description("description_text".to_string());
        assert_eq!(result[0], expected);
    }
}
