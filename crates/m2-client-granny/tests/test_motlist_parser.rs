use m2_client_granny::motlist_parser::MotlistParser;

#[test]
fn test_integration_motlist_parser() {
    let instance = MotlistParser::new();
    assert!(instance.is_ready());
}

#[test]
fn test_parse_valid_lines() {
    let mut parser = MotlistParser::new();
    let content = "WAIT    \"d:/ymir work/pc/warrior/wait.msa\" 100\nWALK    \"d:/ymir work/pc/warrior/walk.msa\" 50\nRUN    \"d:/ymir work/pc/warrior/run.msa\"";
    parser.parse(content);

    let wait_motions = parser.get_motions("WAIT").unwrap();
    assert_eq!(wait_motions.len(), 1);
    assert_eq!(wait_motions[0].path, "d:/ymir work/pc/warrior/wait.msa");
    assert_eq!(wait_motions[0].probability, 100);

    let walk_motions = parser.get_motions("WALK").unwrap();
    assert_eq!(walk_motions.len(), 1);
    assert_eq!(walk_motions[0].path, "d:/ymir work/pc/warrior/walk.msa");
    assert_eq!(walk_motions[0].probability, 50);

    let run_motions = parser.get_motions("RUN").unwrap();
    assert_eq!(run_motions.len(), 1);
    assert_eq!(run_motions[0].path, "d:/ymir work/pc/warrior/run.msa");
    assert_eq!(run_motions[0].probability, 100); // domyslnie 100
}

#[test]
fn test_parse_comments_and_empty_lines() {
    let mut parser = MotlistParser::new();
    let content = "
        // to jest komentarz
        WAIT    \"wait.msa\" 100
        
        # inny komentarz
        RUN     \"run.msa\" 100
    ";
    parser.parse(content);

    assert_eq!(parser.get_motions("WAIT").unwrap().len(), 1);
    assert_eq!(parser.get_motions("RUN").unwrap().len(), 1);
}

#[test]
fn test_parse_multiple_entries() {
    let mut parser = MotlistParser::new();
    let content = "
        WAIT    \"wait1.msa\" 50
        WAIT    \"wait2.msa\" 50
    ";
    parser.parse(content);

    let wait_motions = parser.get_motions("WAIT").unwrap();
    assert_eq!(wait_motions.len(), 2);
    assert_eq!(wait_motions[0].path, "wait1.msa");
    assert_eq!(wait_motions[1].path, "wait2.msa");
}

#[test]
fn test_parse_missing_probability() {
    let mut parser = MotlistParser::new();
    let content = "ATTACK_01    \"attack.msa\"";
    parser.parse(content);

    let motions = parser.get_motions("ATTACK_01").unwrap();
    assert_eq!(motions.len(), 1);
    assert_eq!(motions[0].probability, 100); // 100 jako domyslna wartosc
}

#[test]
fn test_parse_spaces_and_tabs() {
    let mut parser = MotlistParser::new();
    let content = "WAIT\t\t\"wait.msa\"    \t  100";
    parser.parse(content);

    let motions = parser.get_motions("WAIT").unwrap();
    assert_eq!(motions.len(), 1);
    assert_eq!(motions[0].path, "wait.msa");
    assert_eq!(motions[0].probability, 100);
}
