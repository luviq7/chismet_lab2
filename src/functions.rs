pub fn prnt(n: i64, a: f64, b: f64, absol: f64, eps: f64, x: f64){
    if n == 0 {
        println!("{:<5}{:<14}{:<14}{:<16}{:<14}{:<16}", "N", "a", "b", "|expected eps|", "eps", "x");
    }
    println!("{:<5}{:<14.5}{:<14.5}{:<16.5}{:<14.5}{:<16.5}", n, a, b, absol, eps, x);
}

pub fn dix(n: i64, mut a: f64, mut b: f64, eps: f64) -> f64{
    let x: f64 = (a + b) / 2f64;
    if func(a) * func(b) > 0.0 {
        panic!("Корінь не знайдений у межах відрізка [{}, {}]", a, b);
    }
    
    if (a-b).abs() < eps || func(x) == 0.0{
        return x
    }
    else{
        if func(a) * func(x) > 0.0{
            a = x;
        }
        if func(b) * func(x) > 0.0{
            b = x;
        }
        prnt(n, a, b, (a-b).abs(), eps, x);
        return dix(n+1, a, b, eps)
    }
}

pub fn hord(n: i64, a: f64, b: f64, eps: f64) -> f64{
    if func(a) * func(b) > 0.0 {
        panic!("Корінь не знайдений у межах відрізка [{}, {}]", a, b);
    }
    let x: f64 = a - (func(a)*(b-a))/(func(b)-func(a));
    if (a-x).abs()<eps || func(x) == 0.0{
        return x
    }
    else{
        prnt(n, a, b, (a-x).abs(), eps, x);
        return hord(n+1, x, b, eps)
    }
}

pub fn newton(n: i64, a: f64, b: f64, eps: f64) -> f64 {
    let fx = func(b);
    let dfx = func_derivative(b);
    if func(a) * func(b) > 0.0 {
        panic!("Корінь не знайдений у межах відрізка [{}, {}]", a, b);
    }
    if fx.abs() < eps {
        return b;
    } else {
        let x = b - fx / dfx;
        if x < a || x > b {
            panic!("Корінь не знайдений у межах відрізка [{}, {}]", a, b);
        }
        prnt(n, a, b, (a - b).abs(), eps, b);
        return newton(n + 1, a, x, eps);
    }
}

pub fn iteration(n: i64, x: f64, a: f64, b: f64, eps: f64) -> f64 {
    let g = |x: f64| (6.0 * x.powi(2) + 7.0) / x.powi(2);
    if func(a) * func(b) > 0.0 {
        panic!("Корінь не знайдений у межах відрізка [{}, {}]", a, b);
    }
    let x_next = g(x);

    if x < a || x > b {
        panic!("Корінь не знайдений у межах відрізка [{}, {}]", a, b);
    }
    if (x_next - x).abs() < eps {
        return x_next;
    } else {
        prnt(n, x, x_next, (x - x_next).abs(), eps, x_next);
        return iteration(n + 1, x_next, a, b, eps);
    }
}

pub fn func(x: f64) -> f64{
    x.powi(3) - 6f64 * x.powi(2) - 7f64
}

pub fn func_derivative(x: f64) -> f64{
    3f64 * x.powi(2) - 12f64 * x
}