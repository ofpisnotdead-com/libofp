use std::net::UdpSocket;
use std::time::Duration;
use std::collections::HashMap;

struct Server {
    address: String
}

#[tokio::main]
async fn main() {
    let server_list = get_server_list().await;
    let mut servers: Vec<Server> = Vec::new();

    for line in server_list.unwrap().lines() {
        let server = Server { address: line.to_string() };
        servers.push(server);
    }

    println!("loaded {:?} servers", servers.len());
    get_server_status();
}

fn get_server_status() -> std::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0")?;
    let remote_addr = "79.211.46.237:2103";
    sock.set_read_timeout(Some(Duration::from_secs(1)));
    sock.connect(remote_addr);

    let mut buf = [0; 1024];
    sock.send("\\status\\".as_bytes());
    sock.recv(&mut buf);
    let binding = String::from_utf8_lossy(&buf);
    let body = binding.trim_matches(char::from(0));
    //println!("response: {:?}", body);

    // sock.send("//status//").await?;
    // let mut buf = [0; 1024];
    // let len = sock.recv(&mut buf).await?;
    // println!("{:?} bytes received from {:?}", len, remote_addr);

    let mut hash: HashMap<String, String> = HashMap::new();
    let mut key = false;

    let mut last_key = String::from("");
    let mut last_value = String::from("");
    for c in body.chars() {
        if c == "\\".chars().next().unwrap() {
            key = !key;

            if !key {
                hash.insert(last_key.clone(), last_value.clone());
                last_key = "".to_string();
                last_value = "".to_string();
            }
        } else {
            if key {
                last_key.push(c);
            } else {
                last_value.push(c);
            }
        }
    }

    println!("{:?}", hash);

    Ok(())
}

async fn get_server_list() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get("https://master.ofpisnotdead.com/servers.txt").send()
        .await?
        .text()
        .await?;

    Ok(body)
}
