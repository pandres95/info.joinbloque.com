mod cube;
use std::{thread, time};
use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use drawille::Canvas;
use cube::Cube;
use handlebars::Handlebars;
use std::collections::HashMap;
use ansi_term::{Style};
use ansi_term::Colour::{Purple, Blue, };

const FEATURES: [&str; 3] = ["processor 🚀", "button 🧱", "method 💳"];

fn handle_client(mut stream: TcpStream) -> Result<(), ()> {
    let mut feature_index = 0;
    for i in 30..1000000 {
        let cube = Cube::create(20., 20., 20., 30.);
        let cube2 = Cube::create(20., 20., 20., 5.);

        let canvas = Canvas::new(50, 60);
        let rotation = (i as f64%360.).to_radians();

        let cube = cube.rotate_z(rotation);
        let cube = cube.rotate_y(rotation);
        let cube = cube.rotate_x(rotation);

        let cube2 = cube2.rotate_z(0.0);
        let cube2 = cube2.rotate_y(rotation);
        let cube2 = cube2.rotate_x(30.0_f64.to_radians());
        
        let canvas = cube.draw(5, 10, canvas);
        let canvas = cube2.draw(5, 10, canvas);


        stream.write(Purple.bold().blink().paint(canvas.frame()).to_string().as_bytes()).unwrap();

        //clean screen and put the cursor in the first col
        let one_second = time::Duration::from_millis(13);
        let mut handlebars = Handlebars::new();

        let title = Style::new().bold().paint("Welcome to Joinbloque 🚀").to_string();
        let tag_line = Style::new().bold().italic().paint("The smartest payment").to_string();
        let feature = Purple.underline().italic().bold().paint(FEATURES[feature_index % 3]).to_string();
        let join_us = Blue.bold().paint("Join us:").to_string();
        let email = Style::new().bold().paint("hi@joinbloque.com").to_string();

        let source = "
   {{title}}
{{tag_line}} {{feature}}

{{join_us}} {{email}}

      We Love Rust
        🧱 ❤️ 🦀
";

        handlebars
            .register_template_string("welcome", source)
            .unwrap();

        if i%100 == 99 {
            feature_index = feature_index + 1;
        }
        let mut data_template = HashMap::new();

        data_template.insert("title", title);
        data_template.insert("tag_line", tag_line);
        data_template.insert("feature", feature);
        data_template.insert("join_us", join_us);
        data_template.insert("email", email);

        
        let mut data = HashMap::new();

        data.insert("feature", FEATURES[i%3]);
    
        stream.write(handlebars.render("welcome", &data_template).unwrap().as_bytes()).unwrap();
        thread::sleep(one_second);
        stream.write("\x1B[2J\x1B[1;1H".as_bytes()).unwrap();
    }
    Ok(())
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    println!("Server listening on port 3333");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut map: HashMap<String, String> = HashMap::new();
                map.insert(String::from("ip"), stream.peer_addr().unwrap().to_string());
                let json = serde_json::to_string(&map).unwrap();
                println!("{}", json);
                thread::spawn(move|| {
                    
                    match handle_client(stream) {
                        Ok(_) => {

                        },
                        Err(_) => {

                        }
                    }
                });
            }
            Err(_) => {
                // println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}