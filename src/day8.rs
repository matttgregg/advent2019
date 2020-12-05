use std::time::SystemTime;


pub fn run() {
    println!("Day8!");
    let start = SystemTime::now();
    let cbytes = include_bytes!("../data/data8.txt");
    let contents = String::from_utf8_lossy(cbytes);

    to_layers(&contents, 25, 6);
    decode(&contents, 25, 6);


    let timed = SystemTime::now().duration_since(start).unwrap().as_micros();
    println!("Timed: {}us", timed);
}

fn decode(raw_image: &str, wx: usize, wy: usize) {
    let per_layer = wx * wy;
    let mut image = vec!['2'; per_layer]; // Initialize transparent image

    for (i, c) in raw_image.trim().chars().enumerate() {
        let ii = i % per_layer;

        if image[ii] == '2' {
            image[ii] = c
        }
    }


    // Display it!
    for (i, c) in image.iter().enumerate() {
        if i % wx == 0 {
            println!();
        }
        match c {
            '0' => print!(" "),
            _ => print!("0"),
        }
    }
    println!();
    println!("Image complete!");
}

fn to_layers(image: &str, wx: usize, wy: usize) {
    let per_layer = wx * wy;
    let (mut c0, mut c1, mut c2) = (0,0,0);
    let mut last_layer = 0;
    let mut layers = Vec::new();
    let mut best0 = 0;
    let mut best_layer = 0;
    for (i, c) in image.trim().chars().enumerate() {
        let layer = i / per_layer;
        //let ii = i % per_layer;
        //let y = ii / wx;
        //let x = ii % wx;
        if layer != last_layer {
            println!("{}:{}[0],{}[1],{}[2] = {}", last_layer, c0, c1, c2, c1 * c2);
            layers.push((last_layer, c0, c1, c2));
            if best0 == 0 || c0 < best0 {
                best_layer = last_layer;
                best0 = c0;
            } 
            c0=0;c1=0;c2=0;
            last_layer = layer;
        }

        match c {
            '0' => c0 += 1,
            '1' => c1 += 1,
            '2' => c2 += 1,
            _ => {},
        }
    }
    println!("{}:{}[0],{}[1],{}[2] = {}", last_layer, c0, c1, c2, c1 * c2);
    layers.push((last_layer, c0, c1, c2));
    if best0 == 0 || c0 < best0 {
        best_layer = last_layer;
        best0 = c0;
    } 

    let (l, b0, b1, b2) = layers[best_layer];
    println!("Best Layer: {}:{} {}:{}[0] {}[1] {}[2] -> {}", best_layer, l, b0, best0, b1, b2, b1 * b2);
}
