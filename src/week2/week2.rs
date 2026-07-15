use crate::week2::day1;
use crate::week2::day1::enum_add_data;
use crate::week2::day1::f_enum;
use crate::week2::day1::match_enums::Color;
use crate::week2::day1::option_enums;

#[allow(dead_code)]
pub fn week2_main() {
    f_enum::take_n_print_gender(f_enum::Gender::Male);
    enum_add_data::display_ip_address();
    option_enums::use_option_enum();
    day1::match_enums::return_color(Color::Black);
    day1::if_let::using_if_let();
}
