extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use self::serde::{Deserialize, Serialize};
use common::manager::pj_file_manager::PJFileManager;

pub trait PJSerdeDeserialize<'a>: Deserialize<'a> + Serialize {
    type Item: Default + std::fmt::Debug;
    fn new() -> Self::Item {
        Self::Item::default()
    }
}

pub struct PJSerializeUtils {}

impl PJSerializeUtils {
    pub fn from_str<'a, T>(json_str: &'a str) -> Result<T, serde_json::Error>
    where
        T: Deserialize<'a>,
    {
        let parse_result = serde_json::from_str(json_str);
        parse_result
    }

    pub fn from_slice<'a, T>(v: &'a [u8]) -> Result<T, serde_json::Error>
    where
        T: Deserialize<'a>,
    {
        let parse_result = serde_json::from_slice(v);
        parse_result
    }

    pub fn from_hyper_chunk<'a, T>(body: &'a hyper::Chunk) -> Result<T, serde_json::Error>
    where
        T: std::fmt::Debug + PJSerdeDeserialize<'a>,
    {
        let parse_result = PJSerializeUtils::from_slice(body);
        pj_info!("parse data result: {:?}", parse_result);
        parse_result
    }

    pub fn from_file_by_line<'a, T>(json_file_path: &'a str) -> Vec<Result<T, serde_json::Error>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        pj_info!(
            "👉👉from_file_by_line json_file_path: {}",
            json_file_path
        );
        let mut v: Vec<Result<T, serde_json::Error>> = Vec::new();
        let file_content: String = PJFileManager::read_file_content(json_file_path);
        if !file_content.is_empty() {
            v = PJSerializeUtils::model_from_file_content_lines::<T>(&file_content);
        }
        v
    }

    fn model_from_file_content_lines<'a, T>(
        file_content: &'a str,
    ) -> Vec<Result<T, serde_json::Error>>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let mut v: Vec<Result<T, serde_json::Error>> = Vec::new();
        let lines_vec: Vec<Vec<&'a str>> = file_content
            .lines()
            .map(|line| line.split_whitespace().collect())
            .collect();

        for line_vec in lines_vec {
            for line in line_vec {
                let parse_line_result = PJSerializeUtils::from_str::<T>(line);
                v.push(parse_line_result);
            }
        }
        v
    }
}
