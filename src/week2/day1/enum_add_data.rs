#[derive(Debug)]
#[allow(unused_variables)]
pub enum IpAddress {
    V4(String),
    V6(String),
}

#[allow(dead_code)]
impl IpAddress {
    pub fn new_v4(ip: String) -> Self {
        IpAddress::V4(ip)
    }
    pub fn new_v6(ip: String) -> Self {
        IpAddress::V6(ip)
    }
}

#[allow(dead_code)]
pub fn display_ip_address() {
    let home: IpAddress = IpAddress::new_v4(String::from("Localhost:4200"));
    println!("Home address: {:?}", home);
}
