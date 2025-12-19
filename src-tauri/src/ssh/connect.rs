use std::net::TcpStream;

use ssh2::Session;
// 45.77.18.10
// cpt20251219.

pub fn connect() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting via SSH...");
    let tcp = TcpStream::connect("45.77.18.10:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password("root", "cpt20251219.").unwrap();
    // assert!(sess.authenticated());

    if !sess.authenticated() {
        return Err("Authentication failed".into());
    }
    println!("SSH connection established and authenticated.");


    // 打开 SFTP 会话
    let mut sftp = sess.sftp()?;

    // 指定要列出的远程目录
    let remote_path = "/opt/travel-service";
    // let dir = sftp.opendir(remote_path)?;

    println!("Contents of {}:", remote_path);
    let entries = sftp.readdir(remote_path)?;
    // println!("dir:{}", dir);
    for (path, stat) in entries {
        let filename = path.file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let file_type = if stat.is_dir() {
            "dir"
        } else if stat.is_file() {
            "file"
        } else {
            "other"
        };
        println!("- {} ({})", filename, file_type);
    }

    Ok(())
}