// This allows you to ignore the root key.
// Does something similar to: https://github.com/serde-rs/serde/issues/1345
#[macro_export]
macro_rules! suboptions {
  ($str:expr, $inner:ty) => {{
    use serde_derive::Deserialize;
    use toml;
    #[derive(Deserialize, Debug)]      
    struct Outer {
      #[serde(rename = "_")] 
      inner: $inner,
    }

    let tmp: Outer = toml::from_str(&format!(
      "_ = {{ {} }}",
      $str,
    )).map_err(|e| format!("{:?}. Tried to parse: _ = {{ {} }}", e, $str))?;

    tmp.inner
  }}
}
