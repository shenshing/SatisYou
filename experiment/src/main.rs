extern crate rodio;

use std::io::BufReader;

use rodio::buffer::SamplesBuffer;
use rodio::decoder::Decoder;

fn main() {

//play any music (mp3)
    let device = rodio::default_output_device().unwrap();

    let  sink = rodio::Sink::new(&device);

    let file = std::fs::File::open("your-music-file").unwrap(); 
    let dec: rodio::Decoder<std::io::BufReader<std::fs::File>> = rodio::Decoder::new(BufReader::new(file)).unwrap();
    
   

    let mut vec_music: Vec<String>  = Vec::new();
    vec_music.push(String::from("your-music-file")); //your music file here is something like = song.mp3, song.ogg
    vec_music.push(String::from("your-music-file"));
    vec_music.push(String::from("your-music-file"));
    /*
    .
    .
    .
    */


    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    
    for music in vec_music.iter() {
        sink.append(rodio::Decoder::new(BufReader::new(music)).unwrap());
        println!("{} is now playing", music);
        sink.sleep_until_end();
    }

//play music with 3 functions
    let file_name = String::from("sound2.mp3");
    let music_file_res = music_file(file_name);
    let music_raw_data = return_music_raw_data(music_file_res);
    play_music(music_raw_data);

    
}
//make 2 function
//1st convert music by name provide and return to the 2nd function to play
//2nd play music via data send from 1st function

fn music_data(music_name: String) -> std::fs::File {
    let file = std::fs::File::open(music_name).unwrap();

    return file;
}

use rodio::decoder::DecoderError;
fn return_raw_data(file: std::fs::File) -> rodio::Decoder<std::io::BufReader<std::fs::File>>{
    let dec = rodio::Decoder::new(BufReader::new(file)).unwrap();

    return dec;
}

fn play_music(data: rodio::Decoder<std::io::BufReader<std::fs::File>>) {
    let device = rodio::default_output_device().unwrap();

    let  sink = rodio::Sink::new(&device);

    sink.append(data);

    sink.sleep_until_end();

}