use std::{
    path::{Path, PathBuf},
    slice::Iter,
};

/// A structure representing the https://raw.pixls.us database
pub struct PixlsDatabase {
    companies: Vec<Company>,
}

impl PixlsDatabase {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let read_dir = std::fs::read_dir(path.as_ref())?;

        let mut companies = vec![];
        for entry in read_dir {
            let entry = entry?;
            let ftype = entry.file_type()?;

            if ftype.is_dir() {
                companies.push(Company::from_path(entry.path())?);
            }
        }

        companies.sort_by(|c1, c2| c1.name.cmp(&c2.name));

        Ok(Self { companies })
    }

    pub fn company_iter(&self) -> Iter<Company> {
        self.companies.iter()
    }
}

pub struct Company {
    pub name: String,
    models: Vec<Model>,
}

impl Company {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let name = path.as_ref().file_stem().unwrap().to_string_lossy();
        let read_dir = std::fs::read_dir(path.as_ref())?;

        let mut models = vec![];
        for entry in read_dir {
            let entry = entry?;
            let ftype = entry.file_type()?;

            if ftype.is_dir() {
                models.push(Model::from_path(entry.path())?);
            }
        }

        Ok(Self {
            name: name.to_string(),
            models,
        })
    }

    pub fn model_iter(&self) -> Iter<Model> {
        self.models.iter()
    }
}

pub struct Model {
    pub name: String,
    image_paths: Vec<PathBuf>,
}

impl Model {
    fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let name = path.as_ref().file_stem().unwrap().to_string_lossy();
        let read_dir = std::fs::read_dir(path.as_ref())?;

        let mut image_paths = vec![];
        for entry in read_dir {
            let entry = entry?;
            let ftype = entry.file_type()?;

            if ftype.is_file() {
                image_paths.push(entry.path());
            }
        }

        Ok(Self {
            name: name.to_string(),
            image_paths,
        })
    }

    pub fn image_iter(&self) -> Iter<PathBuf> {
        self.image_paths.iter()
    }
}
