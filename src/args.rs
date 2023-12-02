
pub struct Args {
    file_name: String,
}

impl Args {
    pub fn build(args: &Vec<String>) -> Option<Args> {
        let file_name = match args.get(1) {
            Some(str) => str.clone(),
            None => {
                return None;
            }
        };

        Some(
            Args { file_name }
        )

    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }
}

