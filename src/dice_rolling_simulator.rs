
use crate::rand::Rng;

macro_rules! ONE {
        () => {
            1  
        };
}

macro_rules! TWO {
        () => {
            2  
        };
}

macro_rules! THREE {
        () => {
            3  
        };
}

macro_rules! FOUR {
        () => {
            4  
        };
}

macro_rules! FIVE {
        () => {
            5  
        };
}

macro_rules! SIX {
        () => {
            6  
        };
}

pub static DICE_ONE: &'static str = "
                ---------
                |       |
                |   *   |
                |       |
                ---------";

pub static DICE_TWO: &'static str = "
                ---------
                | *     |
                |       |
                |     * |
                ---------";

pub static DICE_THREE: &'static str = "
                ---------
                | *     |
                |   *   |
                |     * |
                ---------";

pub static DICE_FOUR: &'static str = "
                ---------
                | *   * |
                |       |
                | *   * |
                ---------";

pub static DICE_FIVE: &'static str = "
                ---------
                | *   * |
                |   *   |
                | *   * |
                ---------";

pub static DICE_SIX: &'static str = "
                ---------
                | * * * |
                |       |
                | * * * |
                ---------";

pub fn choose() {
    let random_variable = rand::thread_rng().gen_range(1, 7);

    match random_variable {
        ONE!() => {
            println!("{}", DICE_ONE);

        }

        TWO!() => {
            println!("{}", DICE_TWO);
        }

        THREE!() => {
            println!("{}", DICE_THREE);
        }

        FOUR!() => {
            println!("{}", DICE_FOUR);
        }

        FIVE!() => {
            println!("{}", DICE_FIVE);
        }
        SIX!() => {
            println!("{}", DICE_ONE);
        }
        _ => {
            ()
        }
    }
}

pub fn roll_dice() {
    choose()
}
