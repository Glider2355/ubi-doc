#[derive(Debug, PartialEq)]
pub struct Ubiquitous {
    pub class_name: Option<String>,
    pub ubiquitous: Option<String>,
    pub context: Option<String>,
    pub description: Option<String>,
}

impl Ubiquitous {
    pub fn new() -> Self {
        Ubiquitous {
            class_name: None,
            ubiquitous: None,
            context: None,
            description: None,
        }
    }

    pub fn set_class_name(mut self, class_name: String) -> Self {
        self.class_name = Some(class_name);
        self
    }

    pub fn set_ubiquitous(mut self, ubiquitous: String) -> Self {
        self.ubiquitous = Some(ubiquitous);
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

    pub fn is_all_none(&self) -> bool {
        self.class_name.is_none()
            && self.ubiquitous.is_none()
            && self.context.is_none()
            && self.description.is_none()
    }
}
