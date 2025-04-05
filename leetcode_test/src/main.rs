#![allow(unused)]

use std::collections::HashMap;
use reqwest::Error;
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Instant;
use std::fs::{File, OpenOptions};
use std::io::Write;
use tokio::io::AsyncReadExt;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{Duration, sleep};
use futures_util::future::join_all;

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash_map = HashMap::new();
        let len = s.len();
        if len==0{
            return 0i32;
        }
        let mut max = 0;

        let mut idx_l = 0;
        let mut idx_r = 0;
        // 滑动窗口 [l..r]
        while idx_l < len && idx_r < len && idx_l <= idx_r {
            // 向右滑动 1 位
            idx_r += 1;
            let c = s.chars().nth(idx_l).unwrap();
            if hash_map.contains_key(&c) {
                hash_map.remove(&s.chars().nth(idx_l).unwrap());
                idx_l = idx_l + 1;
            } else {
                hash_map.insert(s.chars().nth(idx_r).unwrap(), 1);
                max = max.max(hash_map.len());
            }
        }

        max as i32
    }


#[tokio::main]
async fn main() {
    length_of_longest_substring(String::from(" "));
}

