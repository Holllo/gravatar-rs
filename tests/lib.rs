use gravatar_rs::Generator;

const BAUKE_EMAIL: &str = "me@bauke.xyz";
const BAUKE_HASH: &str = "ecd836ee843ff0ab75d4720bd40c2baf";

const HOLLLO_EMAIL: &str = "helllo@holllo.cc";
const HOLLLO_HASH: &str = "ebff9105dce4954b1bdb57fdab079ff3";

#[test]
fn test_hash_email() {
  assert_eq!(Generator::hash_email(BAUKE_EMAIL), BAUKE_HASH);
  assert_eq!(Generator::hash_email(HOLLLO_EMAIL), HOLLLO_HASH);
}

#[test]
fn test_generator() {
  let base_urls = [&Generator::default().base_url, "cdn.libravatar.org"];
  let samples = [(BAUKE_EMAIL, BAUKE_HASH), (HOLLLO_EMAIL, HOLLLO_HASH)];

  for base_url in base_urls {
    let generator = Generator::default().set_base_url(base_url);

    for (email, hash) in samples {
      let actual = generator.generate(email);
      let expected = format!("https://{base_url}/avatar/{hash}");
      assert_eq!(actual, expected);
    }
  }
}
