use std::{
    fs::File,
    io::{BufWriter, Write},
};

use bitvec::{prelude::*, slice::BitSliceIndex};
use peak_alloc::PeakAlloc;

const N: usize = 1024;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

fn main() {
    let mut board = Board::default();
    let mut place = (N / 2 - 1, N / 2 - 1);
    let mut dir = 0;

    static DIRS: &[(isize, isize); 4] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

    while place.0 != N && place.1 != N {
        let mut x = board.get_mut(place);
        if *x {
            dir += 3;
        } else {
            dir += 1;
        }
        dir %= 4;
        *x = !*x;
        if let Some(i) = place.0.checked_add_signed(DIRS[dir].0) {
            place.0 = i;
        } else {
            break;
        }
        if let Some(j) = place.1.checked_add_signed(DIRS[dir].1) {
            place.1 = j;
        } else {
            break;
        }
    }

    println!("{}", board.count_black());

    let file = File::create("./out.png").unwrap();
    let w = BufWriter::new(file);
    let n = N.try_into().unwrap();
    let mut enc = png::Encoder::new(w, n, n);
    enc.set_color(png::ColorType::Grayscale);
    enc.set_depth(png::BitDepth::One);
    let mut writer = enc.write_header().unwrap().into_stream_writer().unwrap();
    writer.write_all(&board.into_inner()).unwrap();

    println!("{}", PEAK_ALLOC.peak_usage_as_kb());
}

#[derive(Default)]
struct Board(BitArr!(for N * N, in u8));

impl Board {
    fn get_mut(&mut self, (i, j): (usize, usize)) -> <usize as BitSliceIndex<'_, u8, Lsb0>>::Mut {
        self.0.get_mut(i * N + j).unwrap()
    }

    fn count_black(&self) -> usize {
        self.0.count_ones()
    }

    fn into_inner(self) -> [u8; N * N / 8] {
        self.0.into_inner()
    }
}
