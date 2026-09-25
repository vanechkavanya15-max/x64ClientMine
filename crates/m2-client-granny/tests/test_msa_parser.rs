use m2_client_granny::msa_parser::{MsaParser, MsaError};

#[test]
fn test_integration_msa_parser() {
    let instance = MsaParser::new();
    assert!(instance.is_ready());
}

#[test]
fn test_parse_valid_msa() {
    let msa_content = "
        ScriptType               MotionData

        MotionFileName           \"d:/ymir work/pc/warrior/general_ani_attack01.GR2\"
        MotionDuration           1.666667
        Accumulation             0.00   -100.00 0.00

        Group MotionEventData
        {
            MotionEventDataCount     2
            Group Event00
            {
                MotionEventType      1
                StartingTime         0.316667
            }
            Group Event01
            {
                MotionEventType      1
                StartingTime         0.613333
            }
        }
    ";
    
    let parser = MsaParser::new();
    let result = parser.parse(msa_content);
    
    assert!(result.is_ok());
    let data = result.unwrap();
    
    assert_eq!(data.motion_duration, 1.666667);
    assert_eq!(data.accumulation, (0.0, -100.0, 0.0));
    assert_eq!(data.motion_event_data.len(), 2);
    assert_eq!(data.motion_event_data[0].starting_time, 0.316667);
    assert_eq!(data.motion_event_data[1].starting_time, 0.613333);
}

#[test]
fn test_parse_missing_duration() {
    let msa_content = "
        Accumulation             0.00   -100.00 0.00
    ";
    
    let parser = MsaParser::new();
    let result = parser.parse(msa_content);
    assert_eq!(result, Err(MsaError::MissingField));
}

#[test]
fn test_parse_missing_accumulation() {
    let msa_content = "
        MotionDuration           1.666667
    ";
    
    let parser = MsaParser::new();
    let result = parser.parse(msa_content);
    assert_eq!(result, Err(MsaError::MissingField));
}

#[test]
fn test_parse_invalid_float() {
    let msa_content = "
        MotionDuration           invalid
        Accumulation             0.00   -100.00 0.00
    ";
    
    let parser = MsaParser::new();
    let result = parser.parse(msa_content);
    assert_eq!(result, Err(MsaError::ParseFloatError));
}

#[test]
fn test_parse_invalid_accumulation() {
    let msa_content = "
        MotionDuration           1.666667
        Accumulation             0.00   invalid 0.00
    ";
    
    let parser = MsaParser::new();
    let result = parser.parse(msa_content);
    assert_eq!(result, Err(MsaError::ParseFloatError));
}

#[test]
fn test_parse_invalid_starting_time() {
    let msa_content = "
        MotionDuration           1.666667
        Accumulation             0.00   0.00    0.00
        StartingTime             invalid
    ";
    
    let parser = MsaParser::new();
    let result = parser.parse(msa_content);
    assert_eq!(result, Err(MsaError::ParseFloatError));
}

#[test]
fn test_parse_incomplete_accumulation() {
    let msa_content = "
        MotionDuration           1.666667
        Accumulation             0.00   0.00
    ";
    
    let parser = MsaParser::new();
    let result = parser.parse(msa_content);
    assert_eq!(result, Err(MsaError::InvalidFormat));
}
