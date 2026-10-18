
use rand::random;
use std::{
    env,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const MagicBytes: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00, 0xfe, 0xfe, 0xfe, 0xfe,
    0xfd, 0xfd, 0xfd, 0xfd, 0x12, 0x34, 0x56, 0x78,
];

fn ping() -> Vec<u8> {
    let mut b = vec![0x01];
    b.extend_from_slice(
        &(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64)
            .to_be_bytes(),
    );
    b.extend_from_slice(&MagicBytes);
    b.extend_from_slice(&random::<u64>().to_be_bytes());
    b
}

fn pong(b: &[u8]) -> Option<(u64, String)> {
    if b.len() < 35 || b[0] != 0x1c || b[17..33] != MagicBytes {
        return None;
    }
    let guid = u64::from_be_bytes(b[9..17].try_into().ok()?);
    let n = u16::from_be_bytes(b[33..35].try_into().ok()?) as usize;
    Some((guid, String::from_utf8_lossy(b.get(35..35 + n)?).into_owned()))
}

fn open1() -> Vec<u8> {
    let mut b = vec![0x05];
    b.extend_from_slice(&MagicBytes);
    b.push(0x0b);
    b.resize(1464, 0);
    b
}

fn reply1(b: &[u8]) -> Option<(u64, u32, u16)> {
    if b.len() < 28 || b[0] != 0x06 || b[1..17] != MagicBytes {
        return None;
    }

    let guid = u64::from_be_bytes(b[17..25].try_into().ok()?);
    let secure = b[25] != 0;
    let i = if secure { 30 } else { 26 };
    if b.len() < i + 2 {
        return None;
    }

    let cookie = if secure {
        u32::from_be_bytes(b[26..30].try_into().ok()?)
    } else {
        0
    };
    let mtu = u16::from_be_bytes(b[i..i + 2].try_into().ok()?);
    Some((guid, cookie, mtu))
}

fn open2(addr: SocketAddr, cookie: u32, mtu: u16, guid: u64) -> Vec<u8> {
    let mut b = vec![0x07];
    b.extend_from_slice(&MagicBytes);
    b.extend_from_slice(&cookie.to_be_bytes());
    b.push(0);

    match addr {
        SocketAddr::V4(a) => {
            b.push(4);
            b.extend(a.ip().octets().iter().map(|x| !x));
            b.extend_from_slice(&a.port().to_be_bytes());
        }
        SocketAddr::V6(_) => panic!("Use an IPv4 server address"),
    }

    b.extend_from_slice(&mtu.to_be_bytes());
    b.extend_from_slice(&guid.to_be_bytes());
    b
}

fn reply2(b: &[u8]) -> Option<(u64, u16, bool)> {
    if b.len() < 35 || b[0] != 0x08 || b[1..17] != MagicBytes {
        return None;
    }

    Some((
        u64::from_be_bytes(b[17..25].try_into().ok()?),
        u16::from_be_bytes(b[32..34].try_into().ok()?),
        b[34] != 0,
    ))
}

fn receive(s: &UdpSocket, buf: &mut [u8]) -> std::io::Result<usize> {
    let (n, _) = s.recv_from(buf)?;
    Ok(n)
}

fn main() -> std::io::Result<()> {
    let target = env::args()
        .nth(1)
        .expect("Usage: cargo run -- <IPv4:port>")
        .to_socket_addrs()?
        .find(SocketAddr::is_ipv4)
        .expect("No IPv4 address found");

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    let start = Instant::now();
    let mut buf = [0u8; 2048];

    socket.send_to(&ping(), target)?;
    let n = receive(&socket, &mut buf)?;
    let (server_guid, motd) = pong(&buf[..n])
        .ok_or_else(|| std::io::Error::other("Invalid unconnected pong"))?;

    println!("Server: {motd}");
    println!("GUID: {server_guid}");

    socket.send_to(&open1(), target)?;
    let n = receive(&socket, &mut buf)?;
    let (_, cookie, mtu) = reply1(&buf[..n])
        .ok_or_else(|| std::io::Error::other("Invalid open connection reply 1"))?;

    let client_guid = random::<u64>();
    socket.send_to(&open2(target, cookie, mtu, client_guid), target)?;
    let n = receive(&socket, &mut buf)?;
    let (guid, server_mtu, security) = reply2(&buf[..n])
        .ok_or_else(|| std::io::Error::other("Invalid open connection reply 2"))?;

    println!("Connection handshake complete!");
    println!("Server GUID: {guid}");
    println!("MTU: {server_mtu}");
    println!("Security enabled: {security}");
    println!("Elapsed: {:.2?}", start.elapsed());

    Ok(())
}
