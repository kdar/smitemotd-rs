// reqwest::Client::new()
//   .post("https://api.pushed.co/1/push")
//   .form(&[
//     ("app_key", env!("PUSHED_APP_KEY")),
//     ("app_secret", env!("PUSHED_APP_SECRET")),
//     ("target_type", "app"),
//     ("content", &format!("{}: {}", motd[0].title.as_ref().unwrap(), motd[0].description.as_ref().unwrap())),
//   ])
//   .send()?;