#![feature(with_options)]

use clap::{App, Arg};
use std::process::exit;
use std::path::Path;
use std::fs::File;
use crypto::sha2::{Sha256, Sha512, Sha384, Sha224};
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::io::{Write, Read};


fn main(){

    let variaveis = App::new("HashMaker")
                            .about("Calcula o hash de determinado arquivo")
                            .version("1.0")
                            
                            .arg(Arg::with_name("algoritmo")
                                .short("a")
                                .long("algoritmo")
                                .value_name("ALGORITMO")
                                .help("Algoritmo a ser utilizado para calcular o hash do arquivo.")
                                .default_value("sha256")
                                .takes_value(true)
                                )
                            
                            .arg(Arg::with_name("arquivo")
                                .short("A")
                                .long("arquivo")
                                .value_name("ARQUIVO")
                                .help("Arquivo a ser calculado.")
                                .takes_value(true)
                                .required(true)
                                .index(1))

                            .arg(Arg::with_name("saida")
                                .short("o")
                                .long("saida")
                                .value_name("SAIDA")
                                .help("Utiliza o arquivo passado como saida padrão")
                                .takes_value(true))
                            .arg(Arg::with_name("memoria")
                                .short("m")
                                .long("memoria")
                                .value_name("MEMORIA")
                                .help("memoria máxima a ser utilizada pelo programa")
                                .default_value("268435456")
                                .takes_value(true))
                                
                            .get_matches();

    
    
    if !(Path::new(variaveis.value_of("arquivo").unwrap()).exists()) {

        println!("O arquivo não existe!!!");
        exit(1);
    }

    let mut hasher: Box<dyn Digest> = match variaveis.value_of("algoritmo").unwrap() {

        "sha256" => {
            Box::new(Sha256::new())
        }

        "sha512" => {
            Box::new(Sha512::new())
        }
        
        "sha224" => {
            Box::new(Sha224::new())
        }

        "sha384" => {
            Box::new(Sha384::new())
        }

        "md5" => {
            Box::new(Md5::new())

        }
        _ => {
            println!("Algoritmo não suportado");
            exit(1);
        }
    };


    let max_mem: usize = variaveis.value_of("memoria").unwrap().parse().unwrap();
    let mut arquivo = File::with_options().read(true).open(variaveis.value_of("arquivo").unwrap()).unwrap();
    let mut file_len: usize = arquivo.metadata().unwrap().len() as usize;
    
    file_len = if file_len < max_mem {
                    file_len
                }
                else {
                    max_mem
                };

    let mut mem = vec![0u8; file_len];
    
    loop {
        let n = arquivo.read(&mut mem).unwrap();
        if n == 0 {
            break
        }
        else if n < file_len {
            mem.truncate(n);
        }

        hasher.input(&mem);
        mem.clear();
        mem.resize(file_len, 0)
    }

    drop(mem);

    if variaveis.is_present("saida") {
        let mut saida = File::create(variaveis.value_of("saida").unwrap()).expect("erro ao criar arquivo de saida");
        saida.write_all(hasher.result_str().as_bytes()).unwrap();
    }
    else {
        println!("resultado: {}", hasher.result_str());
    }
}