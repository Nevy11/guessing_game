#[allow(dead_code)]
pub fn using_if_let() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("the value of config_max is: {max}"),
    //     _ => (),
    // }
    if let Some(max) = config_max {
        println!("The value of config max variable is : {max}");
    }
}
