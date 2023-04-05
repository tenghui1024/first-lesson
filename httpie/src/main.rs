use anyhow::{anyhow, Ok, Result};
use clap::{AppSettings, Parser};
use reqwest::{header, Url};
use std::str::FromStr;

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "xxx")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    // clap 允许为解析祝来的值添加自定义函数
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    // 检查一下url是否合法
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 分割
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 迭代器中取得的第一个结果作为key,
            // 迭代器返回Some(T)/None
            // 将其转换成Ok(T)/Err(E),然后用?处理
            k: (split.next().ok_or_else(err)?).to_string(),
            // 第二个结果作为value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}
