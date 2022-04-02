#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

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
//!
//! For all possible options see [`Generator`].

/// A generator for Gravatar image URLs.
#[derive(Debug)]
#[non_exhaustive]
pub struct Generator {
  /// The base URL for images, defaults to `www.gravatar.com`.
  pub base_url: String,

  /// Which default image to use when there is no matching Gravatar, defaults
  /// to `None`.
  ///
  /// See the [Gravatar documentation] for all the possible ways to use it.
  ///
  /// [Gravatar documentation]: https://gravatar.com/site/implement/images/#default-image
  pub default_image: Option<String>,

  /// Whether you always want the default image to be returned, defaults to
  /// `false`.
  pub force_default: bool,

  /// A custom size for images, defaults to `None`.
  pub image_size: Option<i32>,

  /// Whether to include `.jpg` in the image URL, defaults to false.
  pub include_file_extension: bool,

  /// Which rating should be allowed, defaults to `None`.
  ///
  /// See the [Gravatar documentation] for all the possible ratings.
  ///
  /// [Gravatar documentation]: https://gravatar.com/site/implement/images/#rating
  pub rating: Option<String>,
}

impl Default for Generator {
  fn default() -> Self {
    Self {
      base_url: "www.gravatar.com".to_string(),
      default_image: None,
      force_default: false,
      image_size: None,
      include_file_extension: false,
      rating: None,
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
    let query_parameters = self.query_parameters();

    let file_extension = if self.include_file_extension {
      ".jpg"
    } else {
      ""
    };

    format!(
      "https://{base_url}/avatar/{hash}{file_extension}{query_parameters}"
    )
  }

  /// Returns all configurable options as a query parameter string.
  pub fn query_parameters(&self) -> String {
    fn encode<D: std::fmt::Display>(data: D) -> String {
      urlencoding::encode(&data.to_string()).into_owned()
    }

    let mut query_parameters = vec![];

    if let Some(default_image) = &self.default_image {
      query_parameters.push(format!("d={}", encode(default_image)));
    }

    if self.force_default {
      query_parameters.push("f=y".to_string());
    }

    if let Some(image_size) = self.image_size {
      query_parameters.push(format!("s={}", encode(image_size)));
    }

    if let Some(rating) = &self.rating {
      query_parameters.push(format!("r={}", encode(rating)));
    }

    if query_parameters.is_empty() {
      String::new()
    } else {
      format!("?{}", query_parameters.join("&"))
    }
  }

  /// Configures the Generator to use a custom base URL for generated URLs.
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

  /// Configures the Generator to include `d=<default image>` in the URL.
  ///
  /// See the [Gravatar documentation] for all the possible ways to use it.
  ///
  /// [Gravatar documentation]: https://gravatar.com/site/implement/images/#default-image
  ///
  /// ```rust
  /// use gravatar_rs::Generator;
  ///
  /// // Use the "identicon" default image, a geometric pattern based on the
  /// // email hash.
  /// Generator::default().set_default_image("identicon");
  /// ```
  pub fn set_default_image(self, default_image: &str) -> Self {
    Self {
      default_image: Some(default_image.to_string()),
      ..self
    }
  }

  /// When set to true, the Generator will always add `f=y` to the URL. Making
  /// Gravatar always return the default image.
  ///
  /// ```rust
  /// use gravatar_rs::Generator;
  ///
  /// Generator::default().set_force_default(true);
  /// ```
  pub fn set_force_default(self, force_default: bool) -> Self {
    Self {
      force_default,
      ..self
    }
  }

  /// Configures the Generator to include a `s=<image size>` in the URL.
  ///
  /// ```rust
  /// use gravatar_rs::Generator;
  ///
  /// // Get 128px images instead of the default 80px.
  /// Generator::default().set_image_size(128);
  /// ```
  pub fn set_image_size(self, image_size: i32) -> Self {
    Self {
      image_size: Some(image_size),
      ..self
    }
  }

  /// Configures the Generator to add `.jpg` to the end of the hash.
  ///
  /// ```rust
  /// use gravatar_rs::Generator;
  ///
  /// Generator::default().set_include_file_extension(true);
  /// ```
  pub fn set_include_file_extension(
    self,
    include_file_extension: bool,
  ) -> Self {
    Self {
      include_file_extension,
      ..self
    }
  }

  /// Configures the Generator to include `r=<rating>` in the URL.
  ///
  /// See the [Gravatar documentation] for all the possible ratings.
  ///
  /// [Gravatar documentation]: https://gravatar.com/site/implement/images/#rating
  ///
  /// ```rust
  /// use gravatar_rs::Generator;
  ///
  /// // Allow G and PG rated images.
  /// Generator::default().set_rating("pg");
  /// ```
  pub fn set_rating(self, rating: &str) -> Self {
    Self {
      rating: Some(rating.to_string()),
      ..self
    }
  }
}
