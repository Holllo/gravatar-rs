#[derive(Debug)]
pub struct Generator {
  pub base_url: String,
}

impl Default for Generator {
  fn default() -> Self {
    Self {
      base_url: "www.gravatar.com".to_string(),
    }
  }
}

impl Generator {
  pub fn hash_email(email: &str) -> String {
    let hash = md5::compute(email.trim().to_lowercase());
    format!("{hash:x}")
  }

  pub fn generate(&self, email: &str) -> String {
    let base_url = &self.base_url;
    let hash = Self::hash_email(email);

    format!("https://{base_url}/avatar/{hash}")
  }

  pub fn set_base_url(self, base_url: &str) -> Self {
    Self {
      base_url: base_url.to_string(),
      ..self
    }
  }
}
