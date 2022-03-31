use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

pub type Districts = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct Census {
    #[serde(rename(deserialize = "dataSet"), default)]
    pub populations: Vec::<Population>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Population {
    pub period: String,

    #[serde(rename(serialize= "AgeDesc", deserialize = "AgeDesc"), default)]
    pub age: String,

    #[serde(rename(serialize= "DCDesc", deserialize = "DCDesc"), default)]
    pub district: String,

    #[serde(rename(serialize= "figure", deserialize = "figure"), default)]
    pub count: u32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
