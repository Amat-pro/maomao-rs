use std::fs::{File};
use std::io::{self, BufRead, Write};
use id3::Tag;

// todo 修改文件地址
// 文件示例
// 1663759011758997504,陌生的地方,https://readfile.zhulang.com/audio_wp/miyue/6.农门寡妇养包子/第001集 陌生的地方.mp3
// 1663759011763191808,窝窝头,https://readfile.zhulang.com/audio_wp/miyue/6.农门寡妇养包子/第002集 窝窝头.mp3
const FILE_PATH: &str = "/data/service/jin-feed/audio_mp3_check/chapter_url.txt";
// const FILE_PATH: &str = "/Users/luzb/log/album/chapter_url1.txt";

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

        loop {
            let response_r = reqwest::get(url.clone()).await;
            match response_r {
                Ok(response) => {
                    let content_length = response.content_length().unwrap();
                    let bytes_r = response.bytes().await;
                    match bytes_r {
                        Ok(bytes) => {
                            // 对比大小
                            if content_length as usize != bytes.len() {
                                continue;
                            }

                            // 写入文件
                            let mut chapter_f = File::create("chapter.mp3").unwrap();
                            let _ = chapter_f.write_all(bytes.as_ref());
                            if !can_open("chapter.mp3".to_string()) {
                                println!("cannot open, line: {}", line_str);
                            }
                            // 删除文件
                            let _ = std::fs::remove_file("chapter.mp3");
                            break;
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
    }

    Ok(())
}


use hound::{WavReader};
use std::io::BufReader;
use rodio::Decoder;

pub fn can_open(path: String) -> bool {
    // 1.
    // return match WavReader::open(path) {
    //     Ok(_reader) => {
    //         // 读取 WAV 文件成功，可以认为音频流有效
    //         // let spec = reader.spec();
    //         // println!("音频流有效");
    //         // println!("通道数: {}", spec.channels);
    //         // println!("采样率: {}", spec.sample_rate);
    //         // println!("样本格式: {:?}", spec.sample_format);
    //         // 可以继续读取其他音频流信息
    //
    //         true
    //     }
    //     Err(err) => {
    //         // 无法读取音频流，可能文件损坏或不是有效的音频流
    //         println!("无效的音频流: {:?}", err);
    //
    //         false
    //     }
    // };

    // 2.
    // let tag = Tag::read_from_path(path.clone());
    // return match tag {
    //     Ok(_) => {
    //         true
    //     }
    //     Err(e) => {
    //         println!("open err: {}", e);
    //         false
    //     }
    // };

    // 3.
    return match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match Decoder::new(reader) {
                Ok(decoder) => {
                    // 成功打开和解码 MP3 文件，可以认为文件能正常播放
                    // println!("MP3 文件能正常播放");
                    // 可以继续操作解码器，如获取音频信息等
                    true
                }
                Err(err) => {
                    // 解码失败，可能文件损坏或不是有效的 MP3 文件
                    println!("无法解码 MP3 文件: {:?}", err);
                    false
                }
            }
        }
        Err(err) => {
            // 无法打开文件
            println!("无法打开文件: {:?}", err);
            false
        }
    };
}
