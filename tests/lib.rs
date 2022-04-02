use gravatar_rs::Generator;

const BAUKE_EMAIL: &str = "me@bauke.xyz";
const HOLLLO_EMAIL: &str = "helllo@holllo.cc";

#[test]
fn test_hash_email() {
  let samples = [("bauke", BAUKE_EMAIL), ("holllo", HOLLLO_EMAIL)];

  for (name, email) in samples {
    insta::assert_snapshot!(
      format!("hash-{name}"),
      Generator::hash_email(email)
    );

    insta::assert_snapshot!(
      format!("hash-{name}-whitespace"),
      Generator::hash_email(&format!("  {email}  "))
    );

    insta::assert_snapshot!(
      format!("hash-{name}-casing"),
      Generator::hash_email(&email.to_uppercase())
    );
  }
}

#[test]
fn test_generator() {
  let emails = [BAUKE_EMAIL, HOLLLO_EMAIL];
  let samples = [
    ("gravatar", Generator::default().base_url),
    ("libravatar", "cdn.libravatar.org".to_string()),
  ];

  for (name, base_url) in samples {
    let generator = Generator::default().set_base_url(&base_url);
    let urls = emails.map(|email| generator.generate(email));
    insta::assert_debug_snapshot!(format!("generate-{name}"), urls);
  }
}
