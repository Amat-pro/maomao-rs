use std::fs::{File};
use std::io::{self, BufRead, Write};
use id3::Tag;

// todo 修改文件地址
// 文件示例
// 1663759011758997504,陌生的地方,https://readfile.zhulang.com/audio_wp/miyue/6.农门寡妇养包子/第001集 陌生的地方.mp3
// 1663759011763191808,窝窝头,https://readfile.zhulang.com/audio_wp/miyue/6.农门寡妇养包子/第002集 窝窝头.mp3
const FILE_PATH: &str = "/data/service/jin-feed/audio_mp3_check/chapter_url.txt";

pub async fn check_mp3() -> io::Result<()> {
    let file = File::open(FILE_PATH)?;

    // 创建一个 BufReader 来读取文件内容
    let reader = io::BufReader::new(file);

    // 逐行读取文件内容
    for line in reader.lines() {
        // println!("{}", line?);
        let line_str = line.unwrap();
        if line_str.is_empty() {
            continue;
        }

        let strs: Vec<&str> = line_str.split(",").collect();
        let url = strs[2];

        println!("check start: {}", url);

        let response_r = reqwest::get(url.clone()).await;
        match response_r {
            Ok(response) => {
                let bytes_r = response.bytes().await;
                match bytes_r {
                    Ok(bytes) => {
                        // 写入文件
                        let mut chapter_f = File::create("chapter.mp3").unwrap();
                        let _ = chapter_f.write_all(bytes.as_ref());
                        if !can_open("chapter.mp3".to_string()) {
                            println!("cannot open, line: {}", line_str);
                        }
                        // 删除文件
                        let _ = std::fs::remove_file("chapter.mp3");
                    }
                    Err(e) => {
                        println!("url bytes fail: {}, err: {}", line_str, e);
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("url get fail: {}, err: {}", line_str.clone(), e);
                continue;
            }
        }
    }

    Ok(())
}


pub fn can_open(path: String) -> bool {
    let tag = Tag::read_from_path(path);

    return match tag {
        Ok(_) => {
            true
        }
        Err(_e) => {
            false
        }
    };
}
