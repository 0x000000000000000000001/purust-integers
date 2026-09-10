pub fn Data_Int_toStringAs(radix: i64, n: i64) -> String {
    let s = if radix == 16 {
        format!("{:x}", n)
    } else if radix == 10 {
        format!("{}", n)
    } else if radix == 8 {
        format!("{:o}", n)
    } else if radix == 2 {
        format!("{:b}", n)
    } else {
        panic!("Unsupported radix: {}", radix);
    };
    s
}

pub fn Data_Int_toNumber(n: i64) -> f64 {
    n as f64
}

pub fn Data_Int_quot(a: i64, b: i64) -> i64 {
    a / b
}

pub fn Data_Int_rem(a: i64, b: i64) -> i64 {
    a % b
}

pub fn Data_Int_pow(a: i64, b: i64) -> i64 {
    a.pow(b as u32)
}

pub fn Data_Int_fromNumberImpl(
    just: purust_core::Func1<UnknownType, std::rc::Rc<Purs_Data_Maybe::Maybe>>,
    nothing: std::rc::Rc<Purs_Data_Maybe::Maybe>,
    num: f64,
) -> std::rc::Rc<Purs_Data_Maybe::Maybe> {
    if num.is_finite() && num.fract() == 0.0 && num >= (-2147483648.0) && num <= (2147483647.0) {
        let int_val = num as i64;
        just(crate::mk_int(int_val))
    } else {
        nothing.clone()
    }
}
