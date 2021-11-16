use ini::Ini;
use std::path::Path;

fn ini_exist() -> bool {
    Path::new("config.ini").exists()
}

fn create_ini() {
    let mut conf = Ini::new();
    conf.with_section(Some("Mail Server"))
        .set("ip", "10.8.192.228")
        .set("port", "25");
    conf.with_section(Some("Recipient"))
        .set("recipient1", "user1@test.com")
        .set("recipient2", "user2@test.com");
    conf.with_section(Some("Subject"))
        .set("subject", "Brand Check");
    conf.with_section(Some("Body"))
        .set("body", "發現異常");
    conf.write_to_file("config.ini").unwrap();
}

fn list_receiver() {
    let i = Ini::load_from_file("config.ini").unwrap();
    let s = i.section(Some("Recipient")).unwrap();
    for (k, v) in s.iter() {
        println!("{}: {:?}", k, v);
    }
}

fn main() {
    if ini_exist() {
        println!("Config file is ready");
        list_receiver();
    } else {
        println!("To create config file");
        create_ini();
    }
}
