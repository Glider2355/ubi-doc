pub struct UbiquitousRow {
    pub class_name: String,
    pub ubiquitous: String,
    pub context: String,
    pub description: String,
    pub github_url: String,
}

impl UbiquitousRow {
    pub fn new() -> Self {
        UbiquitousRow {
            class_name: "".to_string(),
            ubiquitous: "".to_string(),
            context: "".to_string(),
            description: "".to_string(),
            github_url: "".to_string(),
        }
    }

    pub fn set_class_name(mut self, class_name: String) -> Self {
        self.class_name = class_name;
        self
    }

    pub fn set_ubiquitous(mut self, ubiquitous: String) -> Self {
        self.ubiquitous = ubiquitous;
        self
    }

    pub fn set_context(mut self, context: String) -> Self {
        self.context = context;
        self
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn set_github_url(
        mut self,
        repo: String,
        branch: String,
        file_path: String,
        line_number: usize,
    ) -> Self {
        self.github_url = format!(
            "https://github.com/{}/blob/{}/{}#L{}",
            repo, branch, file_path, line_number
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::UbiquitousRow;

    #[test]
    fn test_set_github_url() {
        let repo = "owner/repo".to_string();
        let branch = "main".to_string();
        let file_path = "src/lib.rs".to_string();
        let line_number = 42;

        let row = UbiquitousRow::new().set_github_url(repo, branch, file_path, line_number);

        assert_eq!(
            row.github_url,
            "https://github.com/owner/repo/blob/main/src/lib.rs#L42"
        );
    }
}
