mod functions;
use plotters::prelude::*;
use std::io;

fn f(x: f64) -> f64 {
    x.powi(3) - 6.0 * x.powi(2) - 7.0
}

fn phi(x: f64) -> f64 {
    (6.0 * x.powi(2) + 7.0) / x.powi(2)
}

fn plot_graphs() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("graphs.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Графіки функцій", ("sans-serif", 50))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10.0..10.0, -10.0..100.0)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (-100..=100).map(|x| (x as f64 / 10.0, f(x as f64 / 10.0))),
        &RED,
    ))?.label("y = f(x)").legend(|(x, y)| {
        Rectangle::new(
            [(x - 5, y - 5), (x + 5, y + 5)],
            ShapeStyle {
                color: RED.to_rgba(),
                filled: true,
                stroke_width: 0,
            },
        )
    });

    chart.draw_series(LineSeries::new(
        (-100..=100).map(|x| (x as f64 / 10.0, x as f64 / 10.0)),
        &BLUE,
    ))?.label("y = x").legend(|(x, y)| {
        Rectangle::new(
            [(x - 5, y - 5), (x + 5, y + 5)],
            ShapeStyle {
                color: BLUE.to_rgba(),
                filled: true,
                stroke_width: 0,
            },
        )
    });

    chart.draw_series(LineSeries::new(
        (-100..=100).map(|x| (x as f64 / 10.0, phi(x as f64 / 10.0))),
        &GREEN,
    ))?.label("y = φ(x)").legend(|(x, y)| {
        Rectangle::new(
            [(x - 5, y - 5), (x + 5, y + 5)],
            ShapeStyle {
                color: GREEN.to_rgba(),
                filled: true,
                stroke_width: 0,
            },
        )
    });

    chart.configure_series_labels().border_style(&BLACK).draw()?;

    root.present()?;
    println!("Графіки збережено у graphs.png");

    Ok(())
}

fn main() {
    println!("Автоматизований режим (межі 6 - 7, точність 0.01) - введіть 1");
    println!("Ручний режим (межі і точність через enter) - введіть 2");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let reg: i64 = input.trim().parse().unwrap();
    input.clear();

    if reg == 1 {
        println!("Метод дихотомії");
        functions::dix(0, 6.0, 7.0, 0.01);

        println!("Метод хорд");
        functions::hord(0, 6.0, 7.0, 0.01);

        println!("Метод Ньютона");
        functions::newton(0, 6.0, 7.0, 0.01);

        println!("Метод простої ітерації");
        functions::iteration(0, 6.0, 6.0, 7.0, 0.01);
    } else if reg == 2 {
        println!("Введіть a");
        io::stdin().read_line(&mut input).unwrap();
        let a: f64 = input.trim().parse().unwrap();

        input.clear();
        println!("Введіть b");
        io::stdin().read_line(&mut input).unwrap();
        let b: f64 = input.trim().parse().unwrap();

        input.clear();
        println!("Введіть точність eps");
        io::stdin().read_line(&mut input).unwrap();
        let eps: f64 = input.trim().parse().unwrap();

        println!("Метод дихотомії");
        functions::dix(0, a, b, eps);

        println!("Метод хорд");
        functions::hord(0, a, b, eps);

        println!("Метод Ньютона");
        functions::newton(0, a, b, eps);

        println!("Метод простої ітерації");
        functions::iteration(0, a, a, b, eps);
    }

    plot_graphs().expect("Не вдалося побудувати графіки");
}