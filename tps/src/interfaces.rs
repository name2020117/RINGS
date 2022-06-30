use interfaces::{Address, Interface, Kind};
use std::net::IpAddr;

pub fn get_interface_ip(interface_name: &str) -> IpAddr {
    let interface: Interface = match Interface::get_by_name(interface_name) {
        Ok(Some(i)) => i,
        Ok(None) => {
            println!("Could not find an interface named: {}", interface_name);
            panic!("Could not find interface")
        }
        Err(e) => {
            panic!("An error occured fetching interfaces: {:?}", e);
        }
    };

    // There are multiple addresses for a given network interface
    // There can be only one Ipv4 address
    let addresses: Vec<&Address> = interface
        .addresses
        .iter()
        .filter(|a| a.kind == Kind::Ipv4)
        .collect();
    assert_eq!(addresses.len(), 1, "There is more than one Ipv4 interface somehow");
    addresses[0].addr.unwrap().ip()
}
