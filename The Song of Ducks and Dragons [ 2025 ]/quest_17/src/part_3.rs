use std::{cmp::Reverse, collections::BinaryHeap};

use anyhow::{Context, ensure};

type Point = (usize, usize);
const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn solve(input: &str) -> anyhow::Result<u32> {
    let (grid, volcano, initial_position) = parse_input(input)?;

    for radius in 1.. {
        let mut dist = vec![vec![vec![u32::MAX; grid[0].len()]; grid.len()]; 2];
        dist[0][initial_position.1][initial_position.0] = 0;

        let mut pq = BinaryHeap::from([(Reverse(0), 0usize, initial_position)]);
        while let Some((Reverse(seconds), level, position)) = pq.pop() {
            if seconds / 30 > radius {
                continue;
            }

            if position == initial_position && level == 1 {
                return Ok(seconds * radius);
            }

            for (dx, dy) in DIRECTIONS {
                if let (Some(nx), Some(ny)) = (
                    position.0.checked_add_signed(dx),
                    position.1.checked_add_signed(dy),
                ) && ny < grid.len()
                    && nx < grid[0].len()
                {
                    let next_position = (nx, ny);
                    if distance(next_position, volcano) <= radius as f64 {
                        continue;
                    }

                    let new_cost = seconds + grid[ny][nx] as u32;
                    let next_level = if ny > volcano.1
                        && nx != position.0
                        && (nx == volcano.0 || position.0 == volcano.0)
                    {
                        let delta = match nx.cmp(&position.0) {
                            std::cmp::Ordering::Less => -1,
                            std::cmp::Ordering::Greater => 1,
                            std::cmp::Ordering::Equal => unreachable!(),
                        };

                        if nx == volcano.0 {
                            match level.checked_add_signed(delta) {
                                Some(level) => level,
                                None => continue,
                            }
                        } else {
                            level.saturating_add_signed(delta)
                        }
                        .min(1)
                    } else {
                        level
                    };

                    if new_cost < dist[next_level][ny][nx] {
                        dist[next_level][ny][nx] = new_cost;
                        pq.push((Reverse(new_cost), next_level, next_position));
                    }
                }
            }
        }
    }

    unreachable!();
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Vec<u8>>, Point, Point)> {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut volcano = None;
    let mut starting_pos = None;

    for (i, line) in input.trim().lines().enumerate() {
        let mut row = vec![];

        for c in line.chars() {
            if c == 'S' {
                ensure!(starting_pos.is_none());
                starting_pos = Some((row.len(), i));
                row.push(0);
            } else if c == '@' {
                ensure!(volcano.is_none());
                volcano = Some((row.len(), i));
                row.push(0);
            } else {
                row.push(c.to_digit(10).context(format!("invalid interger: `{c}`"))? as u8);
            }
        }

        if let Some(first) = grid.first() {
            ensure!(row.len() == first.len());
        }

        grid.push(row);
    }

    let volcano = volcano.context("volcano position not in input")?;
    let starting_pos = starting_pos.context("starting position not in input")?;

    Ok((grid, volcano, starting_pos))
}

fn distance(a: Point, b: Point) -> f64 {
    let xdiff = a.0.abs_diff(b.0);
    let ydiff = a.1.abs_diff(b.1);

    ((xdiff.pow(2) + ydiff.pow(2)) as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let expected = 592;
        let actual = solve(
            "\
2645233S5466644
634566343252465
353336645243246
233343552544555
225243326235365
536334634462246
666344656233244
6426432@2366453
364346442652235
253652463426433
426666225623563
555462553462364
346225464436334
643362324542432
463332353552464",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn test_2() {
        let expected = 330;
        let actual = solve(
            "\
545233443422255434324
5222533434S2322342222
523444354223232542432
553522225435232255242
232343243532432452524
245245322252324442542
252533232225244224355
523533554454232553332
522332223232242523223
524523432425432244432
3532242243@4323422334
542524223994422443222
252343244322522222332
253355425454255523242
344324325233443552555
423523225325255345522
244333345244325322335
242244352245522323422
443332352222535334325
323532222353523253542
553545434425235223552",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn test_3() {
        let expected = 3180;
        let actual = solve(
            "\
5441525241225111112253553251553
133522122534119S911411222155114
3445445533355599933443455544333
3345333555434334535435433335533
5353333345335554434535533555354
3533533435355443543433453355553
3553353435335554334453355435433
5435355533533355533535335345335
4353545353545354555534334453353
4454543553533544443353355553453
5334554534533355333355543533454
4433333345445354553533554555533
5554454343455334355445533453453
4435554534445553335434455334353
3533435453433535345355533545555
534433533533535@353533355553345
4453545555435334544453344455554
4353333535535354535353353535355
4345444453554554535355345343354
3534544535533355333333445433555
3535333335335334333534553543535
5433355333553344355555344553435
5355535355535334555435534555344
3355433335553553535334544544333
3554333535553335343555345553535
3554433545353554334554345343343
5533353435533535333355343333555
5355555353355553535354333535355
4344534353535455333455353335333
5444333535533453535335454535553
3534343355355355553543545553345",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
}
