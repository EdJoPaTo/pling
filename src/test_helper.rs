#![allow(dead_code)]

#[cfg(feature = "serde")]
pub fn can_serde_parse<T>(input: &T)
where
    T: std::fmt::Debug + std::cmp::PartialEq + serde::Serialize + serde::de::DeserializeOwned,
{
    dbg!(input);

    let json = serde_json::to_string_pretty(&input).unwrap();
    println!("json {}", json);
    let yaml = serde_yaml::to_string(&input).unwrap();
    println!("yaml\n{}", yaml);

    let json_parsed = serde_json::from_str::<T>(&json).unwrap();
    dbg!(&json_parsed);
    let yaml_parsed = serde_yaml::from_str::<T>(&yaml).unwrap();
    dbg!(&yaml_parsed);

    assert_eq!(input, &json_parsed);
    assert_eq!(input, &yaml_parsed);
}

pub fn can_string_parse<T>(input: &T)
where
    T: std::fmt::Debug + std::cmp::PartialEq + ToString + std::str::FromStr,
{
    dbg!(input);
    let json = input.to_string();
    println!("string {}", json);
    let parsed = json
        .parse::<T>()
        .unwrap_or_else(|_| panic!("failed to parse"));
    dbg!(&parsed);
    assert_eq!(input, &parsed);
}
