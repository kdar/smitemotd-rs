  // let resp: serde_json::Value = reqwest::Client::new()
  //   .post("https://api.pushbullet.com/v2/pushes")
  //   .header("Access-Token", env!("PB_ACCESS_TOKEN"))
  //   .json(&json!({
  //     "channel_tag": "smitemotd",
  //     "type": "note",
  //     "title": motd[0].title.as_ref().unwrap(),
  //     "body": "*hey*\n_there_\n:) ", //motd[0].description.as_ref().unwrap(),
  //   }))
  //   .send()?
  //   .json()?;