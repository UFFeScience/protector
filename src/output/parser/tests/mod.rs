use super::*;

#[test]
fn number_works() {
    assert_eq!(number("123"), Ok(("", 123)));
    assert_eq!(number("1503), (..."), Ok(("), (...", 1503)));
}

#[test]
fn arc_works() {
    assert_eq!(arc("(500, 502)"), Ok(("", (500, 502))));
    assert_eq!(arc("(1,2)"), Ok(("", (1, 2))));
    assert!(arc("(500,)").is_err());
}

#[test]
fn route_works() {
    assert_eq!(
        route("(1, 2), (2, 3), (3, 1)\n"),
        Ok(("\n", vec![(1, 2), (2, 3), (3, 1)]))
    );

    assert_eq!(
        route("(1,2),(2, 3)   ,    (3,1)\n"),
        Ok(("\n", vec![(1, 2), (2, 3), (3, 1)]))
    );

    assert_eq!(route(""), Ok(("", vec![])));
}

#[test]
fn trailing_comma_fails() {
    assert!(global_route("G (1, 2),(2, 3),\n").is_err());
}

#[test]
fn global_route_works() {
    assert_eq!(
        global_route("G (1,2),(2, 3)\n"),
        Ok(("", vec![(1, 2), (2, 3)]))
    );
}

#[test]
fn zone_route_works() {
    assert_eq!(
        zone_route("Z1 (10, 13), (13, 50), (50, 10)\n"),
        Ok(("", (1, vec![(10, 13), (13, 50), (50, 10)])))
    );

    assert_eq!(zone_route("Z1\n"), Ok(("", (1, vec![]))));
}

#[test]
fn get_routes_works() {
    let global_input = "\
            G (1, 2)\n\
            G (2, 3)\n\
        ";
    let expected = vec![vec![(1, 2)], vec![(2, 3)]];
    let routes = get_routes(global_route)(global_input).unwrap().1;
    assert_eq!(routes, expected);

    let zones_input = "\
        Z1 (1, 2)\n\
        Z1\n\n\
        Z2 (2, 3)\n\
        Z2 (4, 5)\n\
        ";
    let expected = vec![
        (1, vec![(1, 2)]),
        (1, vec![]),
        (2, vec![(2, 3)]),
        (2, vec![(4, 5)]),
    ];
    let zone_routes = get_routes(zone_route)(zones_input).unwrap().1;

    assert_eq!(zone_routes, expected);
}

#[test]
fn fixed_units_works() {
    let input = "F 1 2 3\n";
    assert_eq!(fixed_units(input), Ok(("", vec![1, 2, 3])));

    let input = "F 1     2 3\n";
    assert_eq!(fixed_units(input), Ok(("", vec![1, 2, 3])));
}

#[test]
fn score_works() {
    assert_eq!(score("S 105.57\n"), Ok(("", 105.57)));
}

#[test]
fn output_works() {
    let input = include_str!("example.txt");
    let output = Output::new(input).unwrap();

    assert!((output.score - 999.35).abs() < f64::EPSILON);

    assert_eq!(output.fixed_units.len(), 10);

    assert_eq!(output.global_routes.len(), 2);
    assert_eq!(output.global_routes[0].len(), 31);
    assert_eq!(output.global_routes[1].len(), 28);

    assert_eq!(output.zone_routes.len(), 9);
    for (_zone, route) in output.zone_routes.iter() {
        assert!(!route.is_empty());
    }
}

#[test]
fn output_should_fail() {
    let score_missing = "\
        S\n\
        F 10\n\
        G (1, 2)";
    let output = Output::new(score_missing);
    assert!(output.is_err());
    assert!(output.unwrap_err().to_string().contains("score"));

    let fixed_units_line_missing = "\
        S 10.5\n\
        G (1, 2)";
    let output = Output::new(fixed_units_line_missing);
    assert!(output.is_err());
    assert!(output.unwrap_err().to_string().contains("fixed unit"));
}
