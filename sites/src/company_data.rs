use std::{collections::HashMap, fmt::Display, fs};

use serde::Deserialize;

#[derive(Deserialize)]
struct Entry {
    form_address: String,
    domains: Vec<String>,
}

#[derive(Clone)]
pub struct CompanyData {
    domain_to_form_address: HashMap<String, String>,
    domain_to_company: HashMap<String, String>,
}

#[derive(Debug)]
pub enum CompanyDataError {
    ConfigPathNotFound,
    ConfigPathReadFailure,
    ConfigFileDeserialisationError(serde_json::Error),
    DuplicateDomain(String),
}

impl Display for CompanyDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompanyDataError::ConfigPathNotFound => write!(f, "The COMPANY_DATA_CONFIG environment variable was not provided."),
            CompanyDataError::ConfigPathReadFailure => write!(f, "The company data config file could not be read."),
            CompanyDataError::ConfigFileDeserialisationError(error) => write!(f, "The company data config could not be deserialised: {error}."),
            CompanyDataError::DuplicateDomain(domain) => write!(f, "The following domain is used for multiple companies: {domain}."),
        }
    }
}

impl CompanyData {
    pub fn init() -> Result<Self, CompanyDataError> {
        let config_path = std::env::var("COMPANY_DATA_CONFIG")
            .map_err(|_| CompanyDataError::ConfigPathNotFound)?;

        let data =
            fs::read_to_string(config_path).map_err(|_| CompanyDataError::ConfigPathReadFailure)?;

        let data: HashMap<String, Entry> = serde_json::from_str(&data)
            .map_err(|err| CompanyDataError::ConfigFileDeserialisationError(err))?;

        let mut domain_to_company = HashMap::new();
        let mut domain_to_form_address = HashMap::new();

        for (company, entry) in data {
            for domain in entry.domains {
                if domain_to_company.contains_key(&domain)
                    || domain_to_form_address.contains_key(&domain)
                {
                    return Err(CompanyDataError::DuplicateDomain(domain));
                }

                domain_to_company.insert(domain.clone(), company.clone());
                domain_to_form_address.insert(domain, entry.form_address.clone());
            }
        }

        Ok(CompanyData {
            domain_to_form_address,
            domain_to_company,
        })
    }

    pub fn domain_to_company(&self, domain: &str) -> Option<&String> {
        self.domain_to_company.get(domain)
    }

    pub fn domain_to_form_address(&self, domain: &str) -> Option<&String> {
        self.domain_to_form_address.get(domain)
    }
}
