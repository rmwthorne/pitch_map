use std::io;

fn main() {
    // The pitch describes the note as a number. 
    // The number 0 corresponds to the note C4 
    // and Midi Note 60.
    loop {
        println!("Enter a Pitch (or q to quit):");
        let mut pitch = String::new();

        io::stdin()
            .read_line(&mut pitch)
            .expect("Failed to read input.");
        let pitch: i32 = match pitch.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        let human_pitch: String = convert_pitch(pitch);

        println!("{pitch} -> {human_pitch}")
    }
}

fn convert_pitch(pitch: i32) -> String {
    match pitch {
        0 => "C4".to_string(),
        1 => "C#4".to_string(),
        2 => "D4".to_string(),
        3 => "D#4".to_string(),
        4 => "E4".to_string(),
        5 => "F4".to_string(),
        6 => "F#4".to_string(),
        7 => "G4".to_string(),
        8 => "G#4".to_string(),
        9 => "A4".to_string(),
        10 => "A#4".to_string(),
        11 => "B4".to_string(),
        12 => "C5".to_string(),
        _ => "No pitch for that value yet".to_string(),
    }
}