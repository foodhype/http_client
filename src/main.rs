use std::io;
use std::io::net::addrinfo;
use std::io::TcpStream;


fn http_get_request(host: &str, path: &str) {
    match addrinfo::get_host_addresses(host) {
        Ok(ip_addresses) => {
            let ip_address = format!("{}", ip_addresses[0]);    

            println!("Establishing TCP connection...");
            let mut socket = TcpStream::connect(
                    ip_address.as_slice(), 80).unwrap();

            println!("Sending GET request...");
            let get_request = "GET /".to_string() + path + " HTTP/1.1\n" +
                    "Host: " + host + "\n\n";
            match socket.write(get_request.as_bytes()) {
                Ok(_) => {
                    println!("Request sent successfully.");
                },
                Err(error) => {
                    println!("Failed to send request: {}", error);
                }
            }

            println!("Reading from socket...");
            match socket.read_to_end() {
                Ok(value) => {
                    println!("{}", String::from_utf8(value));
                },
                Err(error) => {
                    println!("HTTP error response: {}", error);
                }
            }
        },
        Err(error) => {
            println!("DNS lookup failed: {}", error);
        }
    }

}


fn main() {
    println!("Enter host:");
    let host = io::stdin().read_line();
    println!("Enter path:");
    let path = io::stdin().read_line();
    http_get_request(
        host.unwrap().as_slice().trim_right(),
        path.unwrap().as_slice().trim_right());
}

