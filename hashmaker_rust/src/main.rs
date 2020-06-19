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
                                
                            .get_matches();

    
    
    if !(Path::new(variaveis.value_of("arquivo").unwrap()).exists()) {

        println!("O arquivo não existe!!!");
        exit(1);
    }

    let mut Arquivo = File::with_options().read(true).open(variaveis.value_of("arquivo").unwrap()).unwrap();
    let mut tmp2 = vec![0; std::fs::metadata(variaveis.value_of("arquivo").unwrap()).unwrap().len() as usize];
    Arquivo.read(&mut tmp2).unwrap();

    let resultado: String = match variaveis.value_of("algoritmo").unwrap() {

        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.input(&tmp2);
            hasher.result_str()
        }

        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.input(&tmp2);
            hasher.result_str()
        }
        
        "sha224" => {
            let mut hasher = Sha224::new();
            hasher.input(&tmp2);
            hasher.result_str()
        }

        "sha384" => {
            let mut hasher = Sha384::new();
            hasher.input(&tmp2);
            hasher.result_str()
        }

        "md5" => {
            let mut hasher = Md5::new();
            hasher.input(&tmp2);
            hasher.result_str()
        }

        _ => {
            println!("Algoritmo não suportado");
            exit(1);
        }
    };

    if variaveis.is_present("saida") {
        let mut saida = File::create(variaveis.value_of("saida").unwrap()).expect("erro ao criar arquivo de saida");
        saida.write_all(resultado.as_bytes()).unwrap();
    }
    else {
        println!("resultado: {}", resultado)
    }
}