use ncurses::{attr_t};
use ncurses::{COLOR_PAIR, NCURSES_BITS};
use ncurses::{
    init_pair,
    init_color,
    COLOR_BLACK,
    COLOR_RED,
    COLOR_GREEN,
    COLOR_YELLOW,
    COLOR_BLUE,
    COLOR_MAGENTA,
    COLOR_CYAN,
    COLOR_WHITE
};

// Default colors
pub static COLOR_BKG: i16 = COLOR_BLACK;

// Default pairs (color over BKG)
pub static CPAIR_BLA: i16 = 1;
pub static CPAIR_RED: i16 = 2;
pub static CPAIR_GRE: i16 = 3;
pub static CPAIR_YEL: i16 = 4;
pub static CPAIR_BLU: i16 = 5;
pub static CPAIR_MAG: i16 = 6;
pub static CPAIR_CYA: i16 = 7;
pub static CPAIR_WHI: i16 = 8;

// Default pairs (color over BKG)
pub static CPAIR_IBLA: i16 = 9;
pub static CPAIR_IRED: i16 = 10;
pub static CPAIR_IGRE: i16 = 11;
pub static CPAIR_IYEL: i16 = 12;
pub static CPAIR_IBLU: i16 = 13;
pub static CPAIR_IMAG: i16 = 14;
pub static CPAIR_ICYA: i16 = 15;
pub static CPAIR_IWHI: i16 = 16;

pub static CPAIR_DEFAULT: i16 = 17;

// Custom colors 1-8 are already taken by default
pub static COLOR_C1: i16 = 9;
pub static COLOR_C2: i16 = 10;

// Custom pairs
pub static CPAIR_CUST: i16 = 10;

/// We keep the proportions of rgb values and apply the alpha value, thus, at equal alpha, (1,1,1),
/// (250,250,250) and (899,899,899) are all the same colors (whitin rounding errors)
pub fn custom_color(color: i16, alpha: f64, r: usize, g: usize, b: usize) -> i32 {
    // ||1000, 1000, 1000||
    let max = 1732.0508075688772f64;
    let alpha = alpha * max;
    let (nr, ng, nb) = {
        let norm = ((r * r + g * g + b * b) as f64).sqrt();
        (alpha * r as f64 / norm, alpha * g as f64/ norm, alpha * b as f64 / norm)
    };
    init_color(color, nr as i16, ng as i16, nb as i16)
}

pub fn create_custom_colors() {
    custom_color(COLOR_C1, 1., 800, 400, 200);
    custom_color(COLOR_C2, 0.1, 1000, 1000, 1000);
}

pub fn create_custom_pairs() {
    init_pair(CPAIR_CUST,     COLOR_C1,   COLOR_C2);
}

pub fn init_color_set() {

    init_pair(CPAIR_DEFAULT, COLOR_RED,     COLOR_BKG);

    init_pair(CPAIR_BLA,     COLOR_BLACK,   COLOR_BKG);
    init_pair(CPAIR_RED,     COLOR_RED,     COLOR_BKG);
    init_pair(CPAIR_GRE,     COLOR_GREEN,   COLOR_BKG);
    init_pair(CPAIR_YEL,     COLOR_YELLOW,  COLOR_BKG);
    init_pair(CPAIR_BLU,     COLOR_BLUE,    COLOR_BKG);
    init_pair(CPAIR_MAG,     COLOR_MAGENTA, COLOR_BKG);
    init_pair(CPAIR_CYA,     COLOR_CYAN,    COLOR_BKG);
    init_pair(CPAIR_WHI,     COLOR_WHITE,   COLOR_BKG);

    init_pair(CPAIR_IBLA,    COLOR_BKG,     COLOR_BLACK  );
    init_pair(CPAIR_IRED,    COLOR_BKG,     COLOR_RED    );
    init_pair(CPAIR_IGRE,    COLOR_BKG,     COLOR_GREEN  );
    init_pair(CPAIR_IYEL,    COLOR_BKG,     COLOR_YELLOW );
    init_pair(CPAIR_IBLU,    COLOR_BKG,     COLOR_BLUE   );
    init_pair(CPAIR_IMAG,    COLOR_BKG,     COLOR_MAGENTA);
    init_pair(CPAIR_ICYA,    COLOR_BKG,     COLOR_CYAN   );
    init_pair(CPAIR_IWHI,    COLOR_BKG,     COLOR_WHITE  );


    create_custom_colors();
    create_custom_pairs();

}

