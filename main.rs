use std::fmt::format;
use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;
use mysql::prelude::*;
use mysql::*;


fn main(){
    let addr = "0.0.0.0:3030";
    let Conex = TcpListener::bind(addr).unwrap();
    for stream in Conex.incoming(){
        let stream = stream.unwrap();
        println!("Nova conexão: {:?}", stream);
        thread::spawn(|| RecevComunication(stream));
}
}

fn RecevComunication(mut Tcp: TcpStream){
    let text = "Sejam Muito Bem-Vindo ao Servidor do Lanbyshell!";

    let text = format!("\x1B[2J\x1B[1;1H \r\n\r\r{}\n\rSenha:\n\r", text);

    Tcp.write(text.as_bytes());
    let mut buffer_bytes = [0; 3000];
let mut message = String::new().replace(" ", "");
    let mut usuario_conectado = false;
    let mut tentativas = 3;
    let lanby: String = String::from("Lanbycode");
    let url = Pool::new("mysql://root:@localhost:3306/logins").unwrap();
    let mut conn = url.get_conn().unwrap();
    let mut Buffer_recev_Mysql = String::new();
    
    loop {

        let mut view_mensagem = String::new();
        match Tcp.read(&mut buffer_bytes) {
        Ok(bytes_read) => {

            if bytes_read == 0 {
                // Usuario desconectou 
                println!("Terminei A Task");
                break;
            }
            else {
                 // Concatenar os bytes lidos em uma string
                 let received_text = String::from_utf8_lossy(&buffer_bytes[..bytes_read]);
                 message.push_str(&received_text);

                 // Verificar se a mensagem está completa
                 while let Some(newline_idx) = message.find('\n') {
                     let line = message[..newline_idx].trim();
                     if !line.is_empty() {
                        let pegar_Valor = Pegar_valo(line);
                        view_mensagem = pegar_Valor.to_string();
                         //println!("Mensagem do cliente: {}", pegar_Valor);
                         
                         // Aqui você pode implementar a lógica para encaminhar a mensagem para outros clientes.
                     }
                     // Remover a mensagem processada da string
                     message = message[newline_idx + 1..].to_string();
                    
                 }
                 if view_mensagem.len() < 1 {
                    continue;
                 }
                 else if usuario_conectado == true {
                    let query = format!("SELECT * FROM info WHERE host REGEXP '{}' LIMIT 100", view_mensagem);
                    let result_set:Vec<(String, String, String)> = conn.query_map(query, |(host, user, pass)| {
                        (host, user, pass)
                    }).unwrap();
                    if result_set.len() < 1 {
                        Tcp.write(b"\x1B[2J\x1B[1;1H \rNao encontrei nada :(\n\n\rConsulta:\n\n\r");
                        Tcp.flush();
                        continue;

                    }   
                    Tcp.write(b"\x1B[2J\x1B[1;1H \r");
                    for (host , user , pass) in result_set{
                        let valor = format!("host: {} > user:{}> pass:{}\r\n\n\r", host,user,pass);
                        Buffer_recev_Mysql.push_str(valor.as_str());
                    }

                    Tcp.write(Buffer_recev_Mysql.as_bytes());
                    Tcp.write(b"\n\n\rConsulta:\r\n\r");
                 }
                 else{
                    if view_mensagem == lanby {
                        // logado
                        println!("{}",view_mensagem);
                        usuario_conectado = true;
                        Tcp.write(b"\x1B[2J\x1B[1;1H\nConectado Ao servidor\r\n\r\nConsulta:\n\r").unwrap();
                    } 
                    else if tentativas < 1 {
                        Tcp.write(b"Sai daqui seu viado");
                        break;
                    }
                    else {
                        let msg_tentativas = format!("\x1B[2J\x1B[1;1H\n\nVoce tem {} Tentativas \n\rSenha: \n\r", tentativas);
                        Tcp.write(msg_tentativas.as_bytes()).unwrap();
                        tentativas -=1;

                    }
                 }

            }

        },
        Err(erro) => {  
            // Erro ao ler o bytes 
            break;
        
        },
    }
}

}

fn Pegar_valo(a: &str) -> &str{
    a
}
