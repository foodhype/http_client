use std::io;
use std::io::net::addrinfo;
use std::io::TcpStream;


fn test_http_request(host: &str) {
    println!("host: {}", host);

    let dns_lookup_result = addrinfo::get_host_addresses(host);
    match dns_lookup_result {
        Ok(ip_addresses) => {
            let ip_address = format!("{}", ip_addresses[0]);
           
            let mut socket = TcpStream::connect(ip_address.as_slice(), 80).unwrap();
            socket.write(b"GET /index.html HTTP/1.1\nHost: www.example.com\n\n");
            let response = socket.read_to_end();
            match response {
                Ok(value) => {
                    println!("{}", String::from_utf8(value));
                },
                Err(error) => {
                    println!("Error response: {}", error);
                }
            }
        },
        Err(error) => {
            println!("DNS lookup failed: {}", error);
        }
    }

}


fn main() {
    for line in io::stdin().lines() {
        test_http_request(line.unwrap().as_slice());
    }
}

