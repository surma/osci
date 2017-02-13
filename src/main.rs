#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(myapp =>
            (version: "0.1.0")
            (author: "Surma <surma@surma.link>")
            (about: "Emulates an osci CPU")
            (@arg IMAGE: -i --image +required +takes_value "Image to load into memory")
            (@arg BIOS: -b --bios +required +takes_value "BIOS to load")
        )
        .get_matches();

    println!("Image: {}, BIOS: {}",
             matches.value_of("IMAGE")
                 .expect("Must provide a memory image with -i"),
             matches.value_of("BIOS")
                 .expect("Must provide a BIOS with -b"));
}
