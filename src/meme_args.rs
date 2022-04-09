pub enum MemeArg {
    Text(String),
    ImageUrl(String),
    ImagePath(String),
}

pub struct MemeArgs {
    pub args: Vec<MemeArg>,
}

impl MemeArgs {
    pub fn builder() -> MemeArgsBuilder {
        MemeArgsBuilder::new()
    }
}

pub struct MemeArgsBuilder {
    result: MemeArgs,
}

impl MemeArgsBuilder {
    pub fn new() -> MemeArgsBuilder {
        MemeArgsBuilder {
            result: MemeArgs { args: Vec::new() },
        }
    }

    pub fn text(mut self, text: &str) -> MemeArgsBuilder {
        self.result.args.push(MemeArg::Text(text.to_string()));
        self
    }

    pub fn image_url(mut self, url: &str) -> MemeArgsBuilder {
        self.result.args.push(MemeArg::ImageUrl(url.to_string()));
        self
    }

    pub fn image_path(mut self, path: &str) -> MemeArgsBuilder {
        self.result.args.push(MemeArg::ImagePath(path.to_string()));
        self
    }

    pub fn build(self) -> MemeArgs {
        self.result
    }
}
