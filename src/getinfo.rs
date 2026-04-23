use std::io::Write;
use std::fs::File;


// macro de erro
macro_rules! error {
    ($msg:expr, $($arg:expr), *) => {
        println!("[!] {}", format!($msg,$($arg),*))
    };
}


pub fn info_basica(file: &mut File){
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

pub fn log(file: &mut File, s:String){
    match file.write(s.as_bytes()){
        Err(err) => error!("Não foi possivel escrever no arquivo erro= {}", err),
        _ => {},
    }

    match file.flush(){
        Err(err) => {error!("Erro a dar flush para escrever no arquivo erro= {}",err)},
        _=> {}
    }
}
