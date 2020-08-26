use std::process::Command;

fn main() {
    let cmd = format!("ffmpeg");

    let output = Command::new(cmd)
        .current_dir("./assets")
        .arg("-i")
        .arg("sample.ts")
        .arg("-vcodec")
        .arg("mpeg2video")
        .arg("output.ts")
        .output()
        .expect("failed to run ffmpeg");

    println!("output: {}", String::from_utf8_lossy(&output.stdout));

    // let cmd = format!("ls");

    // let output = Command::new(cmd)
    //     .current_dir("./assets")
    //     .output()
    //     .expect("failed to run ls");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}