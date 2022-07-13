use std::process::Command;

fn main() 
{
    Command::new("zsh")
            .args(["-c","firefox www.youtube.com/watch\\?v=dQw4w9WgXcQ"])
            .output()
            .expect("failed to execute process");
}
