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
    for server in servers {
        get_server_status(server.address);
    }
}

fn get_server_status(address: String) -> std::io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:0")?;

    // increment port by 1
    let mut iter = address.split(':');
    let ip = iter.next().unwrap();
    let port = iter.next().unwrap();
    let query_port = port.parse::<u16>().unwrap() + 1;
    let query_address = format!("{}:{}", ip, query_port);

    sock.set_read_timeout(Some(Duration::from_secs(1)));
    sock.connect(query_address);

    let mut buf = [0; 1024];
    sock.send("\\status\\".as_bytes());
    sock.recv(&mut buf);
    let binding = String::from_utf8_lossy(&buf);
    let mut body = binding.trim_matches(char::from(0));
    println!("response: {:?}", body);

    body = &body[1..body.len()];
    let mut response_parts = body.split("\\");

    let mut hash = HashMap::new();

    // trim first
    loop {
        let key = response_parts.next();
        let value = response_parts.next();

        if key == None {
            break;
        }

        hash.insert(key.unwrap().clone(), value.unwrap().clone());
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
