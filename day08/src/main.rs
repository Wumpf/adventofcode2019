static WIDTH: usize = 25;
static HEIGHT: usize = 6;
static LAYER_STRIDE: usize = WIDTH * HEIGHT;

fn part1(image_layers: &Vec<&[u8]>) -> usize {
    let mut fewest_zerocount_layer = image_layers[0];
    let mut fewest_zerocount_layer_zerocount = LAYER_STRIDE;
    for layer in image_layers {
        let zerocount = layer.iter().filter(|p| **p == 0).count();
        if zerocount < fewest_zerocount_layer_zerocount {
            fewest_zerocount_layer = layer;
            fewest_zerocount_layer_zerocount = zerocount;
        }
    }
    fewest_zerocount_layer.iter().filter(|p| **p == 1).count()
        * fewest_zerocount_layer.iter().filter(|p| **p == 2).count()
}

fn main() {
    let image_raw: Vec<u8> = include_str!("input.txt")
        .as_bytes()
        .iter()
        .map(|c| (*c as char).to_digit(10).unwrap() as u8)
        .collect();

    let mut image_layers = Vec::new();
    let num_layers = image_raw.len() / WIDTH / HEIGHT;
    for l in 0..num_layers {
        image_layers.push(&image_raw[(l * LAYER_STRIDE)..((l + 1) * LAYER_STRIDE)]);
    }

    // PART 1
    println!("num layers {}", part1(&image_layers));

    // PART 2
    let mut final_image = vec![2u8; LAYER_STRIDE];
    for layer in image_layers.iter().rev() {
        for i in 0..LAYER_STRIDE {
            final_image[i] = match layer[i] {
                2 => final_image[i],
                1 => 1,
                0 => 0,
                _ => panic!("unknown color val"),
            }
        }
    }
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", match final_image[x + y * WIDTH] {
                0 => '█',
                1 => ' ',
                2 => '?',
                _ => panic!("unknown color val"),
            });
        }
        println!("");
    }
}
