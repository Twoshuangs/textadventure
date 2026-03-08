use std::io;

#[derive(Clone)]
pub enum T {
    W,
    F,
    E,
    P,
}

pub type Map = Vec<Vec<T>>;
const DIMENSION: [i32; 2] = [4, 4];

pub fn printmap(vecmap: &Map) -> String {
    let mut output: String = String::new();

    for row in vecmap {
        for T in row {
            output.push(match T {
                T::W => '#',
                T::F => '.',
                T::E => '.',
                T::P => 'X',
            });
        }
        output.push('\n');
    }

    output.to_string()
}

pub fn moves(dirn: char, stringmap: &mut String, vecmap: &mut Map, coord: &mut [i32; 2]) -> String {
    let mut x: i32;
    x = (coord[0] - 1) * (DIMENSION[0] + 1);
    x += coord[1] - 1;
    let original_x = x;

    match dirn {
        'w' => {
            if coord[1] > 0 {
                x -= (DIMENSION[0] + 1);
                coord[1] -= 1;
            }
        }
        'a' => {
            if coord[0] > 0 {
                x -= 1;
                coord[0] -= 1;
            }
        }
        's' => {
            if coord[1] < DIMENSION[1] {
                x += (DIMENSION[0] + 1);
                coord[1] += 1;
            }
        }
        'd' => {
            if coord[0] < (DIMENSION[0] + 1) {
                x += 1;
                coord[0] += 1;
            }
        }
        _ => (),
    }

    if stringmap.chars().nth(x as usize).unwrap() != '.' {
        stringmap.to_string()
    } else {
        vecmap[coord[1] as usize][coord[0] as usize] = T::P;
        let mut chars: Vec<char> = stringmap.chars().collect();
        chars[x as usize] = 'X';
        chars[original_x as usize] = '.';
        let newstringmap = chars.into_iter().collect::<String>();
        newstringmap
    }
}
