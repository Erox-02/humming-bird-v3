use anyhow::Result;
use hbp100::{Engine, Sesman};

fn main() -> Result<()> {
    let mut engine = Engine::new();
    let sesman = Sesman::new();

    let sid = sesman.Crtses(Some("hospital_discharge"));
    println!("session id : {}", sid);

    let r1 = sesman
        .prc_wses(
            engine.pipeline_mut(),
            &sid,
            "Patient John Doe, MRN: 123456",
        )
        .expect("session 1 failed");

    println!("masked 1   : {}", r1.masked_text);
    println!("metadata 1 : {:?}", r1.metadata);

    let r2 = sesman
        .prc_wses(
            engine.pipeline_mut(),
            &sid,
            "Patient Jane Smith, MRN: 789012",
        )
        .expect("session 2 failed");

    println!("masked 2   : {}", r2.masked_text);
    println!("metadata 2 : {:?}", r2.metadata);
    let restored = sesman
        .res_wses(
            engine.pipeline_mut(),
            &sid,
            "[NAME_1] and [NAME_2] have MRNs [ID_1] and [ID_2]",
        )
        .expect("restore failed");

    println!("restored   : {}", restored);

    println!("sessions   : {:?}", sesman.ls_ses());
    println!("count      : {}", sesman.ses_cnt());
    assert!(sesman.rm_ses(&sid));
    println!("after rm   : {}", sesman.ses_cnt());

    Ok(())
}