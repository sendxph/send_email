use ini::Ini;
use std::path::Path;

fn f_exist() -> bool {
    Path::new("config.ini").exists()
}

fn create_ini() {
    let mut conf = Ini::new();
    conf.with_section(Some("Mail Server"))
        .set("ip", "10.225.21.2")
        .set("port", "25");
    conf.with_section(Some("Receiver"))
        .set("number", "2")
        .set("receiver1", "user1@hello.com")
        .set("receiver2", "user2@hello.com");
    conf.with_section(Some("Subject"))
        .set("subject", "Brand Check");
    conf.with_section(Some("Body"))
        .set("body", "發現異常");
    conf.write_to_file("config.ini").unwrap();
}


fn main() {
    if f_exist() {
        println!("Config file is ready");
    } else {
        println!("To create config file");
        create_ini();
    }
}
