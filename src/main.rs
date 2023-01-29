use std::{process, io::Read};
use std::fs::File;

use sargparse::{ArgumentParser, ArgumentType, InnerData};

mod ssh;
mod ftp;

pub fn main() {

    let login: String;
    let loginWordlist: Vec::<&str>;

    let password: String;
    let passwordWordlist: Vec<&str>;

    let ip: String;
    let service: Vec<&str>;

    let port: &str;

    let mut parser = ArgumentParser::new(Some("Login and Password Cracker"));

    parser.add_argument("-l", "--loginUser", "pass the found user", false, Some(InnerData::STR(String::from("NULL"))), ArgumentType::STR);
    parser.add_argument("-L", "--loginList", "pass the login wordlist for crack", false, Some(InnerData::STR(String::from("NULL"))), ArgumentType::STR);
    parser.add_argument("-p", "--passwordUser", "pass the found password", false, Some(InnerData::STR(String::from("NULL"))), ArgumentType::STR);
    parser.add_argument("-P", "--passwordList", "pass the passwords wordlist for crack", false, Some(InnerData::STR(String::from("NULL"))), ArgumentType::STR);
    parser.add_argument("-i", "--ip", "pass the ip address for the crack", true, Some(InnerData::STR(String::from("NULL"))), ArgumentType::STR);
    parser.add_argument("-s", "--service", "pass the service like this: SSH:22, FTP:21, MYSQL:3306", true, Some(InnerData::STR(String::from("NULL"))), ArgumentType::STR);
    
    let args = parser.parse_args().unwrap();

    if args.get("loginList").unwrap().get_str() != "NULL"{
        let mut Wordlist = File::open(args.get("loginList")
            .unwrap()
            .get_str())
                .expect("Can't locate the login wordlist file...");    
        
        let mut content = String::new();

        Wordlist.read_to_string(&mut content);

        loginWordlist = content.split("\n").collect();
        

        if args.get("passwordList").unwrap().get_str() != "NULL"{
            let mut Wordlist = File::open(args.get("passwordList")
                .unwrap()
                .get_str())
                    .expect("Can't locate password wordlist file...");
                
            let mut content = String::new();
            
            Wordlist.read_to_string(&mut content);

            passwordWordlist = content.split("\n").collect();

            ip = args.get("ip")
                .unwrap()
                .get_str();
            
            let textService: String = args.get("service")
                .unwrap()
                .get_str();

            service = textService
                .split(":")
                .collect();
            
            port = service[1];

            if service[0] == "SSH" {
                ssh::BruteSSHUserListAndPassList(loginWordlist, passwordWordlist, ip, port);

            }else if service[0] == "FTP" {
                ftp::BruteFtpUserListAndPassList(loginWordlist, passwordWordlist, ip, port);

            }else {
                println!("Application Protocol don't supported....\n");
                println!("Supported Protocols (use uppercase):");
                println!("[+] SSH");
                println!("[+] FTP");
                process::exit(1);

            }


        }else if args.get("passwordUser").unwrap().get_str() != "NULL"{
            password = args.get("passwordUser")
                .unwrap()
                .get_str();        
            
            ip = args.get("ip")
                .unwrap()
                .get_str();
            
            let textService: String = args.get("service")
                .unwrap()
                .get_str();
            
            service = textService
                .split(":")
                .collect();

            port = service[1];
    
            if service[0] == "SSH" {
                ssh::BruteSSHUserListAndPass(loginWordlist, password, ip, port)
    
            }else if service[0] == "FTP" {
                ftp::BruteFtpUserListAndPass(loginWordlist, password, ip, port);

            }else {
                println!("Application Protocol don't supported....\n");
                println!("Supported Protocols (use uppercase):");
                println!("[+] SSH");
                println!("[+] FTP");
                process::exit(1);
    
            }

        }else {
            println!("Pls pass the password or password wordlist for crack...");
            process::exit(1);
            
        }
            

    }else if args.get("loginUser").unwrap().get_str() != "NULL"{
        login = args.get("loginUser")
            .unwrap()
            .get_str();

        if args.get("passwordList").unwrap().get_str() != "NULL"{
            let mut Wordlist = File::open(args.get("passwordList")
                .unwrap()
                .get_str())
                    .expect("Can't locate password wordlist file...");
            
            let mut content = String::new();

            Wordlist.read_to_string(&mut content);

            passwordWordlist = content.split("\n").collect();

            ip = args.get("ip")
                .unwrap()
                .get_str();

            let textService: String = args.get("service")
                .unwrap()
                .get_str();

            service = textService
                .split(":")
                .collect();

            port = service[1];
            
            if service[0] == "SSH" {
                ssh::BruteSSHUserAndPassList(login, passwordWordlist, ip, port)

            }else if service[0] == "FTP" {
                ftp::BruteFtpUserAndPassList(login, passwordWordlist, ip, port);
                //println!("Crack login and password using wordlist in protocol FTP");
                

            }else {
                println!("Application Protocol don't supported....\n");
                println!("Supported Protocols (use uppercase):");
                println!("[+] SSH");
                println!("[+] FTP");
                process::exit(1);

            }

        }else if args.get("passwordUser").unwrap().get_str() != "NULL"{
            password = args.get("passwordUser")
                .unwrap()
                .get_str();        
            
            ip = args.get("ip")
                .unwrap()
                .get_str();

            let textService: String = args.get("service")
                .unwrap()
                .get_str();

            service = textService
                .split(":")
                .collect();
        
            port = service[1];

            if service[0] == "SSH" {
                ssh::BruteSSHUserAndPass(login, password, ip, port);
    
            }else if service[0] == "FTP" {
                ftp::BruteFtpUserAndPass(login, password, ip, port); 
                // Crack FTP with login and password

            }else {
                println!("Application Protocol don't supported....\n");
                println!("Supported Protocols (use uppercase):");
                println!("[+] SSH");
                println!("[+] FTP");
                process::exit(1);
    
            }
    
        }else {
            println!("Pls pass the password or password wordlist for crack...");
            process::exit(1);
        
        }

    }else {
        println!("Pls pass the login or login wordlist for crack...");
        process::exit(1);

    }
    
}