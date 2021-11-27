use nom::{
    bytes::complete::{tag, take, take_till, take_while},
    character::complete::multispace1,
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use num::Num;

use crate::types::Cell;

fn int_from_str<T: num::Integer>(input: &str) -> Result<T, <T as Num>::FromStrRadixErr> {
    T::from_str_radix(input, 10)
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_int<T: num::Integer>(input: &str) -> IResult<&str, T> {
    map_res(take_while(is_digit), int_from_str)(input)
}

fn parse_size(input: &str) -> IResult<&str, u64> {
    let (input, size) = parse_int::<u64>(input)?;
    let (input, _) = tag("T")(input)?;

    Ok((input, size))
}

fn parse_grid_name(input: &str) -> IResult<&str, (u8, u8)> {
    let (input, _) = tag("/dev/grid/node-x")(input)?;
    let (input, x) = parse_int::<u8>(input)?;
    let (input, _) = tag("-y")(input)?;
    let (input, y) = parse_int::<u8>(input)?;

    Ok((input, (x, y)))
}

fn parse_grid_line(input: &str) -> IResult<&str, (u8, u8, Cell)> {
    let (input, (x, y)) = parse_grid_name(input)?;
    let (input, _) = multispace1(input)?;
    let (input, size) = parse_size(input)?;
    let (input, _) = multispace1(input)?;
    let (input, used) = parse_size(input)?;
    let (input, _) = multispace1(input)?;
    let (input, avail) = parse_size(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = take_till(|c| c == '\n')(input)?;

    Ok((input, (x, y, Cell { size, used, avail })))
}

pub fn parse_input(input: &str) -> IResult<&str, Vec<(u8, u8, Cell)>> {
    let (input, _) = take_till(|c| c == '\n')(input)?;
    let (input, _) = take(1u32)(input)?;
    let (input, _) = take_till(|c| c == '\n')(input)?;
    let (input, _) = take(1u32)(input)?;
    separated_list1(tag("\n"), parse_grid_line)(input)
}

#[cfg(test)]
mod test {
    use crate::{
        parser::{parse_grid_line, parse_grid_name, parse_input, parse_size},
        types::Cell,
    };

    #[test]
    fn test_parse_size() {
        let result = parse_size("123T");
        match result {
            Ok((_, size)) => assert_eq!(size, 123),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_grid_name() {
        let result = parse_grid_name("/dev/grid/node-x0-y0");
        match result {
            Ok((_, coords)) => assert_eq!(coords, (0, 0)),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_grid_line() {
        let result = parse_grid_line("/dev/grid/node-x0-y0     89T   65T    24T   73%");
        match result {
            Ok((_, cell)) => assert_eq!(
                cell,
                (
                    0,
                    0,
                    Cell {
                        size: 89,
                        used: 65,
                        avail: 24,
                    }
                )
            ),
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_input() {
        let result = parse_input(
            r#"root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     89T   65T    24T   73%
/dev/grid/node-x0-y1     92T   65T    27T   70%"#,
        );
        match result {
            Ok((_, cells)) => assert_eq!(
                cells,
                vec![
                    (
                        0,
                        0,
                        Cell {
                            size: 89,
                            used: 65,
                            avail: 24,
                        }
                    ),
                    (
                        0,
                        1,
                        Cell {
                            size: 92,
                            used: 65,
                            avail: 27,
                        }
                    )
                ],
            ),
            _ => panic!(),
        }
    }
}
