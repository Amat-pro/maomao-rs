use std::fs::{File};
use std::io::{self, BufRead, Write};
use tokio::task::JoinHandle;

// todo 修改文件地址
// 文件示例
// 1663759011758997504,陌生的地方,https://readfile.zhulang.com/audio_wp/miyue/6.农门寡妇养包子/第001集 陌生的地方.mp3
// 1663759011763191808,窝窝头,https://readfile.zhulang.com/audio_wp/miyue/6.农门寡妇养包子/第002集 窝窝头.mp3
const FILE_PATH: &str = "/data/service/jin-feed/audio_mp3_check/chapter_url.txt";
// const FILE_PATH: &str = "/Users/luzb/00_audio_check/new_0601/chapter_url_01.txt";

pub fn check_mp3() -> io::Result<()> {
    let file = File::open(FILE_PATH)?;

    // 创建一个 BufReader 来读取文件内容
    let reader = io::BufReader::new(file);

    // 逐行读取文件内容
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        // println!("{}", line?);
        let line_str = line.unwrap();
        if line_str.is_empty() {
            continue;
        }

        lines.push(line_str);
    }

    check_bench(lines);

    Ok(())
}

fn check_bench(line_strs: Vec<String>) {
    let mut tasks = Vec::<JoinHandle<()>>::new();

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    let runtime = builder.max_blocking_threads(10).enable_all().build().unwrap();
    for str in line_strs {
        let task = runtime.spawn_blocking(|| { check(str) });
        tasks.push(task);
    }

    // 等待这些后台任务的完成
    for handle in tasks {
        // `spawn` 方法返回一个 `JoinHandle`，它是一个 `Future`，因此可以通过  `block_on` 来等待它完成
        runtime.block_on(handle).unwrap();
    }
}

fn check(line_str: String) {
    // thread::sleep(Duration::from_secs(10));
    let strs: Vec<&str> = line_str.split("^").collect();
    let url = strs[1];

    let file_name = format!("{}.mp3", strs[0]);

    println!("check start: {}", url);

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(120)) // Set the timeout duration to 5 seconds
        .build()
        .unwrap();

    loop {
        let response_r = client.get(url.clone()).send();
        match response_r {
            Ok(response) => {
                let content_length = response.content_length().unwrap();
                let bytes_r = response.bytes();
                match bytes_r {
                    Ok(bytes) => {
                        // 对比大小
                        if content_length as usize != bytes.len() {
                            continue;
                        }

                        // 写入文件
                        let mut chapter_f = File::create(file_name.clone()).unwrap();
                        let _ = chapter_f.write_all(bytes.as_ref());

                        let duration_r = mp3_duration::from_path(file_name.clone());
                        match duration_r {
                            Ok(duration) => {
                                // println!("duration: {}", duration.as_secs());
                                if duration.as_secs() < 5 * 60 {
                                    println!("check_fail: duration lt 5 min, line: {}, duration: {}", line_str, duration.as_secs());
                                }
                            }
                            Err(e) => {
                                println!("check_fail: get duration fail, line: {}, err: {}", line_str, e);
                            }
                        }

                        // if !can_open("chapter.mp3".to_string()) {
                        //     println!("cannot open, line: {}", line_str);
                        // }
                        // 删除文件
                        let _ = std::fs::remove_file(file_name);
                        break;
                    }
                    Err(e) => {
                        println!("url bytes fail: {}, err: {}", line_str, e);
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("url get fail: {}, err: {}", line_str, e);
                continue;
            }
        }
    }
}


// pub fn can_open(path: String) -> bool {
//     // 1.
//     // return match WavReader::open(path) {
//     //     Ok(_reader) => {
//     //         // 读取 WAV 文件成功，可以认为音频流有效
//     //         // let spec = reader.spec();
//     //         // println!("音频流有效");
//     //         // println!("通道数: {}", spec.channels);
//     //         // println!("采样率: {}", spec.sample_rate);
//     //         // println!("样本格式: {:?}", spec.sample_format);
//     //         // 可以继续读取其他音频流信息
//     //
//     //         true
//     //     }
//     //     Err(err) => {
//     //         // 无法读取音频流，可能文件损坏或不是有效的音频流
//     //         println!("无效的音频流: {:?}", err);
//     //
//     //         false
//     //     }
//     // };
//
//     // 2.
//     // let tag = Tag::read_from_path(path.clone());
//     // return match tag {
//     //     Ok(_) => {
//     //         true
//     //     }
//     //     Err(e) => {
//     //         println!("open err: {}", e);
//     //         false
//     //     }
//     // };
//
//     // 3. 可以采纳 需要alsa库的支持
//     return match File::open(path) {
//         Ok(file) => {
//             let reader = BufReader::new(file);
//             match Decoder::new(reader) {
//                 Ok(decoder) => {
//                     // 成功打开和解码 MP3 文件，可以认为文件能正常播放
//                     // println!("MP3 文件能正常播放");
//                     // 可以继续操作解码器，如获取音频信息等
//                     true
//                 }
//                 Err(err) => {
//                     // 解码失败，可能文件损坏或不是有效的 MP3 文件
//                     println!("无法解码 MP3 文件: {:?}", err);
//                     false
//                 }
//             }
//         }
//         Err(err) => {
//             // 无法打开文件
//             println!("无法打开文件: {:?}", err);
//             false
//         }
//     };
// }

