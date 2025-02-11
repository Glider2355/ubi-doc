#[derive(Debug, PartialEq)]
pub struct Ubiquitous {
    pub ubiquitous: String,
    pub context: Option<String>,
    pub class_name: Option<String>,
    pub description: Option<String>,
    pub file_path: Option<String>,
    pub line_number: Option<usize>,
}

impl Ubiquitous {
    pub fn new() -> Self {
        Ubiquitous {
            class_name: None,
            ubiquitous: String::new(),
            context: None,
            description: None,
            file_path: None,
            line_number: None,
        }
    }

    pub fn set_class_name(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    pub fn set_ubiquitous(mut self, ubiquitous: String) -> Self {
        self.ubiquitous = ubiquitous;
        self
    }

    pub fn set_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }

    pub fn set_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn set_file_path(mut self, file_path: String) -> Self {
        self.file_path = Some(file_path);
        self
    }

    pub fn set_line_number(mut self, line_number: usize) -> Self {
        self.line_number = Some(line_number);
        self
    }

    pub fn is_all_none(&self) -> bool {
        self.class_name.is_none()
            && self.ubiquitous.is_empty()
            && self.context.is_none()
            && self.description.is_none()
    }
}
