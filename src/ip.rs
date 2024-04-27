use std::net::IpAddr;

pub fn ip_to_u32(ip: IpAddr) -> u32 {
    let octets: [u8; 4] = match ip {
        IpAddr::V4(ipv4) => ipv4.octets(),
        IpAddr::V6(ipv6) => {
            let ipv6_octets = ipv6.octets();
            [
                ipv6_octets[12],
                ipv6_octets[13],
                ipv6_octets[14],
                ipv6_octets[15],
            ]
        }
    };

    ((octets[0] as u32) << 24)
        + ((octets[1] as u32) << 16)
        + ((octets[2] as u32) << 8)
        + (octets[3] as u32)
}
