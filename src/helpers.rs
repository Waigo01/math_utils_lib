use crate::PREC;

#[doc(hidden)]
pub fn center_in_string(f: String, n: i32) -> String {
    let f_string = f.to_string();
    let f_string_len = f_string.len();

    if f_string_len as i32 > n {
        return f_string;
    }

    let padding = n-f_string_len as i32;
    let l_padding;
    let r_padding;
    if padding % 2 == 0 {
        l_padding = padding/2;
        r_padding = l_padding;
    } else {
        l_padding = (padding as f64/2.).floor() as i32;
        r_padding = (padding as f64/2.).ceil() as i32;
    }
    let mut buffer_string = String::new();
    for _ in 0..l_padding {
        buffer_string += " ";
    }
    buffer_string += &f_string;
    for _ in 0..r_padding {
        buffer_string += " ";
    }

    return buffer_string;
}

#[doc(hidden)]
pub fn round_and_format(x: f64, latex: bool) -> String {
    if (x*10f64.powi(PREC-2)).round()/10f64.powi(PREC-2) == 0. && !latex && x != 0. {
        let mut scientific = format!("{:+e}", x);
        if scientific.chars().nth(0).unwrap() == '+' {
            scientific = scientific[1..].to_string();
        }
        return scientific;
    } else if (x*10f64.powi(PREC-2)).round()/10f64.powi(PREC-2) == 0. && x != 0. {
        let mut scientific = format!("{:+e}", x);
        if scientific.chars().nth(0).unwrap() == '+' {
            scientific = scientific[1..].to_string();
        }
        let left = scientific.split("e").nth(0).unwrap();
        let right = scientific.split("e").nth(1).unwrap();
        return format!("{}\\cdot 10^{{{}}}", left, right);
    } else {
        let rounded = (x*10f64.powi(PREC-2)).round()/10f64.powi(PREC-2);
        let rounded_string;
        if rounded == 0. && rounded.to_string().len() > 1 {
            rounded_string = rounded.to_string()[1..].to_string();
        } else {
            rounded_string = rounded.to_string();
        }
        return rounded_string;
    }
}

#[doc(hidden)]
pub fn cart_prod<T: Clone>(arr: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut results: Vec<Vec<T>> = vec![vec![]];
    for i in 0..arr.len() {
        let mut temp_res = vec![];
        for res in &results {
            for el in &arr[i] {
                let mut new_res = res.to_vec();
                new_res.push(el.clone());
                temp_res.push(new_res);
            }
        }
        results = temp_res;
    }

    return results;
}
