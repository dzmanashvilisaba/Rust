use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,time::Duration,
};

use proj_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        
        /*thread::spawn(|| {
            handle_connection(stream);
        });*/
                 
        pool.execute(|| {
            handle_connection(stream);
            println!("Connection established!");
        });
        
        //handle_connection(stream);
        
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    /*let http_request: Vec<_> = buf_reader.lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();*/

    /*let c = &buf_reader.lines().next();

    println!("{:?}", c);*/
    //println!("{&buf_reader.lines()}");
    //println!("{&buf_reader.lines().next()}");

    let request_line = match buf_reader.lines().next() {
        Some(line) => line.unwrap(),
        _ => String::from("Nothing"), 
    };

    
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.xhtml"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(4));
            ("HTTP/1.1 200 OK", "sleep.xhtml")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.xhtml"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\n\
        Content-Length: {length}\r\n\r\n\
        {contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();


    /*
    if request_line == "GET / HTTP/1.1" {
        //let response = "HTTP/1.1 200 OK\r\n\r\n";
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.xhtml").unwrap();
        let length = contents.len();
        let response = format!(
            "{status_line}\r\n\
            Content-Length: {length}\r\n\r\n\
            {contents}"
        );

        //println!("Request: {:#?}", http_request);
        stream.write_all(response.as_bytes()).unwrap();
    } 
    else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.xhtml").unwrap();
        let length = contents.len();
        let response = format!(
            "{status_line}\r\n\
            Content-Length: {length}\r\n\r\n
            {contents}"
        );

        //println!("Request: {:#?}", http_request);
        stream.write_all(response.as_bytes()).unwrap();
    }*/
}
