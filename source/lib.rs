//! # gravatar-rs
//!
//! This crate provides an API for creating [Gravatar image URLs], and by
//! extension, [Ivatar/Libravatar image URLs].
//!
//! [Gravatar image URLs]: https://gravatar.com/site/implement/images/
//! [Ivatar/Libravatar image URLs]: https://wiki.libravatar.org/api/
//!
//! The default [`Generator::base_url`] is `www.gravatar.com`, if you want to
//! set a custom `base_url` use [`Generator::set_base_url`].
//!
//! ```rust
//! use gravatar_rs::Generator;
//!
//! let generator = Generator::default();
//!
//! let gravatar_url = generator.generate("helllo@holllo.cc");
//!
//! assert_eq!(
//!   gravatar_url,
//!   "https://www.gravatar.com/avatar/ebff9105dce4954b1bdb57fdab079ff3"
//! );
//! ```

/// A generator for Gravatar image URLs.
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
  /// Hashes an email with [`md5`] according to the [Gravatar hashing steps].
  ///
  /// ```rust
  /// use gravatar_rs::Generator;
  ///
  /// let hash = Generator::hash_email("helllo@holllo.cc");
  ///
  /// assert_eq!(
  ///   hash,
  ///   "ebff9105dce4954b1bdb57fdab079ff3"
  /// );
  /// ```
  ///
  /// [Gravatar hashing steps]: https://en.gravatar.com/site/implement/hash/
  pub fn hash_email(email: &str) -> String {
    let hash = md5::compute(email.trim().to_lowercase());
    format!("{hash:x}")
  }

  /// Generates a new Gravatar image URL using the Generator's configuration.
  ///
  /// See the top-level module documentation for examples.
  pub fn generate(&self, email: &str) -> String {
    let base_url = &self.base_url;
    let hash = Self::hash_email(email);

    format!("https://{base_url}/avatar/{hash}")
  }

  /// Configures the Generator to use the base URL for generated URLs.
  ///
  /// ```rust
  /// use gravatar_rs::Generator;
  ///
  /// // Use Libravatar instead of Gravatar.
  /// Generator::default().set_base_url("cdn.libravatar.org");
  /// ```
  pub fn set_base_url(self, base_url: &str) -> Self {
    Self {
      base_url: base_url.to_string(),
      ..self
    }
  }
}
