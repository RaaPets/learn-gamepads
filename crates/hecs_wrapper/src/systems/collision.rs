use super::*;

const I: f64 = 0.85;
//  //  //  //  //  //  //  //
pub(crate) fn update(items: hecs::QueryMut<(&mut Movement, &CellPosition, &WaveFunction)>) {
    let mut list: Vec<_> = items.into_iter().collect();
    let n = list.len();
    if n < 2 {
        return;
    }
    for alpha in 0..(n - 1) {
        for betta in alpha..n {
            let pos = list[alpha].1 .1;
            let counter_pos = list[betta].1 .1;
            let CellPosition { x, y } = *pos - *counter_pos;
            let delta = Position::new(x as f64, y as f64);
            let scalar_x = delta.x.abs();
            let scalar_y = delta.y.abs();
            let scalar = if scalar_x > scalar_y {
                scalar_x
            } else {
                scalar_y
            };
            if scalar >= 1. {
                continue;
            } else {
                if scalar > 0. {
                    list[alpha].1 .0 .0 += delta;
                    list[betta].1 .0 .0 -= delta;
                } else {
                    //let r_delta = rnd_delta(interferr as u64);
                    let r_delta_1: arithm2d::pos2d::Pos2D<f64> =
                        WaveFunction::interferr(list[alpha].1 .2, list[betta].1 .2).into();
                    let r_delta = r_delta_1 * I;
                    list[alpha].1 .0 .0 += r_delta;
                    list[betta].1 .0 .0 -= r_delta;
                }
            }
        }
    }
}
