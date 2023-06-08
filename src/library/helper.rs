use ethers::types::U256;
pub fn ethers_wei(amount: U256) -> String {
    ethers::utils::format_ether(amount)[0..4].to_string()
}

pub fn prettify_int(int: U256, decimal: i128) -> String {
    let mut s = String::new();
    let int_div_decimal = int / i128::pow(10, decimal.try_into().unwrap());
    let int_str = int_div_decimal.to_string();
    let a = int_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ' ');
        }
        s.insert(0, val)
    }
    s
}
