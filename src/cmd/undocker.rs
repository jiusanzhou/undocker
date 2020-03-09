#![allow(unused_variables, unused_mut, unused_parens, dead_code)]
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::console;

pub fn undocker(file_name: &str) {
    console::info(&format!("开始处理文件: {}", file_name));

    // 打开文件
    let f = match File::open(file_name) {
        Ok(v) => v,
        Err(why) => {
            panic!("打开文件错误: {}", why);
        }
    };

    // 业务负责人负责的业务
    let mut devs: HashMap<String, Vec<Record>> = HashMap::new();

    // 读取文件, 加载到hashmap
    let file = BufReader::new(&f);
    for (_num, line) in file.lines().skip(1).enumerate() {
        let record = Record::new(line.unwrap());
        // 过滤非 问财 和 语言图像
        // 如果开发为空,用运维,如果运维为空: 使用函数处理
        let depr = record.department.to_string();
        if (
            depr.as_str() == "i问财开发部" ||
            depr.as_str() == "语音图像-语音系统开发组" ||
            depr.as_str() == "语音图像-语音核心组" ) {

            devs.entry(record.dev_mail.to_string()).or_insert(Vec::new()).push(record);
        }
    }

    for (dev_mail, items) in devs.iter() {
        println!("{} 有 {} 个项目", dev_mail, items.len());
        // 生成表格, 生成邮件, 发送邮件
        gen_csv_file(&dev_mail, &items.to_vec())
    }
}

fn calculate_hash(t: &str) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn gen_csv_file(dev_mail: &str, items: &Vec<Record>) {
    // dev_mail hash 生成文件名
    println!("{:?}", calculate_hash(dev_mail))

}

fn gen_email() {

}

#[derive(Clone)]
#[derive(Debug)]
struct Record {
    app: String,
    department: String,
    dev_mail: String,
    opr_mail: String,
    first_release_time: String,
    status: String,
    container_name: String,
    container_time: String,
    comment: String,

    dev_empty: bool,
    opr_empty: bool,
}

impl Record {
    fn new(line: String) -> Self {
        let parts: Vec<&str> = line.split(",").collect();
        let mut record = Record{
            app: parts[0].to_string(),
            department: parts[1].to_string(),
            dev_mail: parts[2].to_string(),
            opr_mail: parts[3].to_string(),
            first_release_time: parts[4].to_string(),
            status: parts[5].to_string(),
            container_name: parts[6].to_string(),
            container_time: parts[7].to_string(),
            comment: parts[8].to_string(),
            dev_empty: false,
            opr_empty: false,
        };

       if ( record.opr_mail == "" ) {
            record.opr_empty = true;
            record.opr_mail = match record.department.as_ref() {
                "语音图像-语音系统开发组" => "zoe@zoe.im".to_string(),
                "语音图像-语音核心组" => "zoe@zoe.im".to_string(),
                _ => "unknown@zoe.im".to_string(),
            };
        }

        if ( record.dev_mail == "" ) {
            record.dev_empty = true;
            record.dev_mail = record.opr_mail.to_string();
        }

        record
    }
}