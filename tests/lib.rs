use gravatar_rs::Generator;

const BAUKE_EMAIL: &str = "me@bauke.xyz";
const HOLLLO_EMAIL: &str = "helllo@holllo.cc";

#[test]
fn test_hash_email() {
  let samples = [("bauke", BAUKE_EMAIL), ("holllo", HOLLLO_EMAIL)];
  let mut snapshot = vec![];

  for (name, email) in samples {
    snapshot.push((format!("hash-{name}"), Generator::hash_email(email)));

    snapshot.push((
      format!("hash-{name}-whitespace"),
      Generator::hash_email(&format!("  {email}  ")),
    ));

    snapshot.push((
      format!("hash-{name}-casing"),
      Generator::hash_email(&email.to_uppercase()),
    ));
  }

  insta::assert_debug_snapshot!("hash-email", snapshot);
}

#[test]
fn test_generator() {
  let emails = [BAUKE_EMAIL, HOLLLO_EMAIL];
  let samples = [
    ("gravatar", Generator::default().base_url),
    ("libravatar", "cdn.libravatar.org".to_string()),
  ];
  let mut snapshot = vec![];

  for (name, base_url) in samples {
    let generator = Generator::default().set_base_url(&base_url);
    let urls = emails.map(|email| generator.generate(email));
    snapshot.push((format!("generate-{name}"), urls));
  }

  insta::assert_debug_snapshot!("generator", snapshot);
}

#[test]
fn test_all_options() {
  let generator = Generator::default()
    .set_base_url("cdn.libravatar.org")
    .set_default_image("identicon")
    .set_force_default(true)
    .set_image_size(128)
    .set_include_file_extension(true)
    .set_rating("pg");

  let urls = [BAUKE_EMAIL, HOLLLO_EMAIL].map(|email| generator.generate(email));
  insta::assert_debug_snapshot!("generate-options", urls);
}
