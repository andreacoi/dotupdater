// check internet connection
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
fn check_internet_connection() -> bool {
    TcpStream::connect("8.8.8.8:53").is_ok()
}

// function to implement an infinite cycle if the computer is connected. while the computer is not
// connected sleep for 3 second then re-check the connection.
pub fn wait_for_connection() {
    while !check_internet_connection() {
        sleep(Duration::from_secs(3));
    }
}
