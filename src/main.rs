use getopts::Options;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use std::process::Command;

const PATTERN_COUNT: usize = 22;

// save the images
fn save_img(
    pxs: &mut [u32],
    width: usize,
    height: usize,
    from: usize,
    to: usize,
) -> io::Result<()> {
    for i in from..to {
        let mut fp = File::create(format!("{}.ppm", i))?;
        println!("Saving {}...", format!("{}.ppm", i));
        write!(fp, "P6\n{} {} 255\n", width, height)?;
        for y in 0..height {
            for x in 0..width {
                // get current pixel
                let px = pxs[(y * width + x) + ((i - from) * width * height)];
                // separate the colors of the single pixel
                let c = [
                    ((px >> 8 * 2) & 0xff) as u8,
                    ((px >> 8 * 1) & 0xff) as u8,
                    ((px >> 8 * 0) & 0xff) as u8,
                ];
                // write the color buffer
                fp.write(&c)?;
            }
        }
    }
    Ok(())
}

// render pixels for all the images
fn fill_pixels(pxs: &mut [u32], width: u128, height: u128, from: u128, to: u128) {
    let c = [width / 2, height / 2]; // center for the circle pattern
    for i in from..to {
        print!("Rendering {}...", i);
        for y in 0..height {
            for x in 0..height {
                // these are some cpu rendering experiments that I tried to invent
                let d = [c[0] - x, c[1] - y]; // distance from center for the circle pattern
                let ceq = d[0].pow(2) + d[1].pow(2); // circle equation (w/o radius part) for the circle pattern
                pxs[((y * width + x) + ((i - from) * width * height)) as usize] = (if i == 0 {
                    (x & y) * 0xff0000
                } else if i == 1 {
                    (x & y) * 0xf
                } else if i == 2 {
                    (x & y) * y * x
                } else if i == 3 {
                    (x & y) * (y / (x + 1))
                } else if i == 4 {
                    ((x / (y + 1)) + (y / (x + 1))) * ((x % (y + 1)) + (y % (x + 1)))
                } else if i == 5 {
                    ((x % (y + 1)) + (y % (x + 1))) / (((x / (y + 1)) + (y / (x + 1))) + 1)
                } else if i == 6 {
                    ((x % (y + 1)) + (y % (x + 1))) / (((x / (y + 1)) + (y / (x + 1))) + 1)
                } else if i == 7 {
                    (y * width) + (x * height)
                } else if i == 8 {
                    ((y * width) & (x * height)) * 0xf
                } else if i == 9 {
                    ((y | width) | (x | height)) * 0xf
                } else if i == 10 {
                    ((y | width) | (x | height)) * 0xf
                } else if i == 11 {
                    ((y | width) | (x | height)) * 0xf
                } else if i == 12 {
                    y.pow(2)
                } else if i == 13 {
                    x.pow(2) * y.pow(2)
                } else if i == 14 {
                    x.pow(2) * y.pow(2) % 256
                } else if i == 15 {
                    (x.pow(2)) % ((y.pow(2)) + 1)
                } else if i == 16 {
                    y.pow(2) % (x.pow(2) + 1)
                } else if i == 17 {
                    (y.pow(2) % (x.pow(2) + 1)) + (x.pow(2) % (y.pow(2) + 1))
                } else if i == 18 {
                    (y.pow(2) % (x.pow(2) + 1)) / ((x.pow(2) % (y.pow(2) + 1)) + 1)
                } else if i == 19 {
                    (y * width + x * i as u128) / (x + 1)
                } else if i == 20 || i == 21 {
                    // circle pattern
                    (ceq <= (width / 2).pow(2)) as u128
                        // empty circle
                        * !(ceq <= (((width / 2) - (width / 16)).pow(2)) && i == 21) as u128
                        * 0xFF0000
                } else {
                    0
                })
                    as u32;
                if i >= PATTERN_COUNT as u128 {
                    eprintln!("ERROR: image with id '{}' does not exist!", i);
                    std::process::exit(1);
                }
            }
        }
        println!(" Done!");
    }
}

fn print_usage(program: &str, opts: Options) {
    print!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}

fn main() -> io::Result<()> {
    // get cmd line args
    let argv: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    // defining options
    opts.optflag("c", "clean", "removes generated images");
    opts.optflagopt(
        "C",
        "convert",
        "converts images to the specified format, png by default. Depends on imagemagick",
        "FORMAT",
    );
    opts.optopt("f", "from-to", "render only some patterns", "FROM:TO");
    opts.optopt("h", "height", "change rendered image height", "HEIGHT");
    opts.optopt("i", "id", "render pattern with specific id", "ID");
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optflag("r", "render", "renders the images and saves them");
    opts.optflagopt("s", "show", "run feh to show rendered images", "ID");
    opts.optopt("w", "width", "change rendered image width", "WIDTH");
    opts.optflag("", "help", "print this help menu");
    // parsing options
    let matches = match opts.parse(&argv[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    // if no options are provided, send an error message
    if argv[1..].is_empty() {
        print_usage(&argv[0], opts);
        eprintln!("ERROR: no options were provided!");
        std::process::exit(1);
    };

    // help option
    if matches.opt_present("help") {
        print_usage(&argv[0], opts);
        std::process::exit(0);
    }

    // clean option
    if matches.opt_present("c") {
        Command::new("sh").arg("-c").arg("rm *.ppm").output()?;
    }

    // set image numbers that should be generated
    let mut from: usize = 0;
    let mut to: usize = PATTERN_COUNT - 1;

    // render option
    if matches.opt_present("r") {
        // from-to option
        match matches.opt_str("f") {
            Some(x) => {
                // read the option arguments separated by ':'
                let f_arg = x.as_str().split(":").collect::<Vec<&str>>();
                from = f_arg[0].parse::<usize>().unwrap();
                to = f_arg[1].parse::<usize>().unwrap();
            }
            None => {}
        }
        // id option
        match matches.opt_str("i") {
            Some(x) => {
                // read the option as usize
                from = x.parse::<usize>().unwrap();
                to = from + 1;
            }
            None => {}
        }
        // get height from options
        let mut height: usize = 512;
        match matches.opt_str("h") {
            Some(x) => height = x.parse::<usize>().unwrap(),
            None => {}
        }
        // get width from options
        let mut width: usize = 512;
        match matches.opt_str("w") {
            Some(x) => width = x.parse::<usize>().unwrap(),
            None => {}
        }
        // allocating the pixels vector (all images are stored in the same array)
        let mut pixels = vec![0u32; width * height * (to - from + 1)];
        // generate the patterns
        fill_pixels(
            &mut pixels,
            width as u128,
            height as u128,
            from as u128,
            to as u128,
        );
        // write the patterns in image files
        save_img(&mut pixels, width, height, from, to)?;
    }

    // convert the image to the specified
    let mut imgf = String::from("ppm");
    if matches.opt_present("C") {
        match matches.opt_str("C") {
            Some(x) => imgf = x.parse().unwrap(),
            None => imgf = String::from("png"),
        }
        for i in from..to {
            // run the command, get its errors and convert them to a string
            let cmd_err = match String::from_utf8(
                Command::new("sh")
                    .arg("-c")
                    .arg(format!("convert {0}.ppm {0}.{1}", i, imgf))
                    .output()
                    .unwrap()
                    .stderr,
            ) {
                // get unicode errors
                Ok(v) => v,
                Err(e) => panic!("Invalid utf-8 sequence: {}", e),
            };
            // print eventual convert errors
            if !cmd_err.is_empty() {
                eprintln!("{}", cmd_err);
            }
        }
    }
    // open the image in an image viewer (right now only feh is supported)
    if matches.opt_present("s") {
        let mut imgname = format!("*.{}", imgf);
        let imgid; // -1 for all
        match matches.opt_str("s") {
            Some(x) => imgid = x.parse::<i32>().unwrap(),
            None => imgid = -1,
        }
        if imgid > 0 {
            imgname = format!("{}.{}", imgid, imgf);
        }
        Command::new("sh")
            .arg("-c")
            .arg("feh -B black")
            .arg(imgname)
            .output()?;
    }

    Ok(())
}
