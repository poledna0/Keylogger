use std::fs::OpenOptions;
//use std::thread;
//use std::time::Duration;
//use chrono::{format, prelude::*};
use std::io::Write;
use std::fs::File;

// macro de erro
macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("[!] {}", format!($msg,$($arg),*))
    };
}
fn info_basica(file: &mut File){
    // aq estou pegando as informações do sistema, como verssao, o tipo de sistema e entre outros.
    let info_so = {
        let info = os_info::get();
        format!("OS: {}\nVerssão: {}\n", info.os_type(), info.version())
    };
    log(file, info_so);
    let hostname_wrap = hostname::get();
    if hostname_wrap.is_ok(){
        log(file, format!("Hostname: {:?}\n",hostname_wrap.unwrap()))
    }
    
    else {log(file, format!("Hostname: Erro"));}

}

fn log(file: &mut File, s:String){
    match file.write(s.as_bytes()){
        Err(err) => error!("Não foi possivel escrever no arquivo erro= {}", err),
        _ => {},
    }

    match file.flush(){
        Err(err) => {error!("Erro a dar flush para escrever no arquivo erro= {}",err)},
        _=> {}
    }
}

fn main() {
    let temp_path = std::env::temp_dir();
    let filename = temp_path.join("keycap.log");

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .expect("erro ao criar arquivo");

    info_basica(&mut output);
    
}