fn winner(str1: String, str2: String) -> String {
    let repeated_str1 = (0..str2.len()).fold(String::new(), |acc, _| acc + &str1);
    let repeated_str2 = (0..str1.len()).fold(String::new(), |acc, _| acc + &str2);

    if repeated_str1 == repeated_str2 {
        return str1;
    }

    let mut repeated_str1_iter = repeated_str1.chars();
    let mut repeated_str2_iter = repeated_str2.chars();

    for _ in 0..repeated_str1.len() {
        let mut s1 = String::new();
        let mut s2 = String::new();

        let ss1 = repeated_str1_iter.next();
        let ss2 = repeated_str2_iter.next();

        match ss1.and(ss2) {
            Some(_) => {
                s1 = ss1.unwrap().to_string();
                s2 = ss2.unwrap().to_string();
            }
            _ => continue,
        }

        if s1 == s2 {
            continue;
        }

        if (s1 == "R" && s2 == "S") || (s1 == "S" && s2 == "P") || (s1 == "P" && s2 == "R") {
            return str1;
        } else {
            return str2;
        }
    }

    return str1;
}

fn solve(input: &str) -> String {
    let players: Vec<&str> = input.trim_matches(|c| c == '(' || c == ')').split(")(").collect();
    // println!("{:?}", players);

    let level = (players.len() as f32).log2().ceil() as u32;
    let seeded_count = 2i32.pow(level) - (players.len() as i32);
    // println!("{:?}", seeded_count);

    let (seeded, unseeded) = players.split_at(seeded_count as usize);

    let mut bracket: Vec<String> = Vec::new();
    for player in seeded {
        bracket.push(player.to_string());
        bracket.push(player.to_string());
    }
    for player in unseeded {
        bracket.push(player.to_string());
    }

    // println!("seeded:   {:?}", seeded);
    // println!("unseeded: {:?}", unseeded);
    // println!("bracke:   {:?}", bracket);

    while bracket.len() > 1 {
        let mut next_bracket: Vec<String> = Vec::new();
        let mut i: usize = 0;
        while i < bracket.len() {
            next_bracket.push(winner(bracket[i].clone(), bracket[i + 1].clone()));
            i += 2
        }
        bracket = next_bracket;
    }

    let mut result: String = "(".to_string();
    result = result + &bracket[0];
    result = result + ")";
    result
}

fn test(input: &str, expected: &str) {
    let actual = solve(input);
    if actual == expected {
        print!(".")
    } else {
        println!(r#"
input:    {}
expected: {}
actual:   {}"#,
                 input,
                 expected,
                 actual)
    }
}

fn main() {
    test("(RSP)(R)(RPS)(SP)", "(RPS)");
    test("(RPS)(R)(RSP)(SP)(RSSP)", "(RSSP)");
    test("(RRS)(S)(PSSRP)(PRP)(PSS)", "(PRP)");
    test("(PRS)(PSPP)(PRSP)(S)(RR)(SSPR)", "(PRS)");
    test("(PSRP)(PR)(RPRPR)(PSSPP)(SP)(SRPP)(PR)", "(SP)");
    test("(SPS)(R)(RP)(RRS)(PPRRS)(R)(RS)(RRRRP)", "(PPRRS)");
    test("(PPSRPSPRR)(SP)(PPPRSSR)(PS)(P)(PRSPS)(PP)(RSSR)", "(SP)");
    test("(SRPRS)(SRPSRS)(SPP)(RSPRS)(S)(SRPSPS)(RSPPSSS)(SRRPRRPSSP)",
         "(RSPPSSS)");
    test("(SRSPSPRS)(RRPRRS)(PRRRRS)(RSSPSSRPS)(PPSSPPRR)(PPSPPS)(PSPSPSSSP)(RPPRPS)",
         "(PRRRRS)");
    test("(S)(PRS)(RSRP)(S)(PPRR)(PP)(RSSS)(P)(RSR)", "(PP)");
    test("(RPR)(P)(PSPR)(SRSRP)(SR)(RPPR)(RRS)(S)(SSPR)(PRPR)",
         "(RPPR)");
    test("(PSR)(PPPRR)(S)(SP)(S)(PR)(SPSRP)(PPSRR)(PRPPR)(RRRSP)(SR)",
         "(S)");
    test("(PPRPP)(RSS)(PRS)(R)(RPRP)(SPSSS)(RR)(PPRP)(RSSS)(RSRS)(RP)",
         "(PPRPP)");
    test("(P)(PPPRR)(RRRS)(RR)(RPRSS)(PRSPS)(PP)(R)(PSR)(RPPP)(RP)(SSSR)",
         "(PSR)");
    test("(SR)(P)(RRPRP)(RSPS)(PSS)(SPPSP)(RRPS)(PR)(RRRSR)(PRR)(SSS)(RRRSS)(P)",
         "(SR)");
    test("(PS)(RS)(RR)(RPR)(SR)(SP)(PRP)(PPS)(R)(PRSP)(SSPRR)(SP)(PPR)(RSRR)",
         "(SSPRR)");
    test("(RRRRS)(SRPRR)(PPSS)(SSPPS)(R)(R)(P)(P)(PSSPR)(S)(RRPP)(SPRR)(S)(RR)(S)",
         "(PSSPR)");
    test("(RRPSSRP)(SSSSSP)(RRSPSS)(PRSRRSRP)(SSRRRRR)(SS)(SSSSSSPPRP)(R)(SRRSR)(PPPSRSP)(RPRS)(R\
          SRPPRS)(RPPPPRPR)(PRRSR)(RPRRSR)",
         "(PPPSRSP)");
    test("(SSSRS)(SRPSS)(RSPRP)(RPPPP)(S)(PPRPS)(RRR)(PS)(RPSPS)(SPP)(PSRS)(P)(P)(RR)(S)(PSP)",
         "(RSPRP)");
    test("(SPP)(PR)(SR)(SRPSP)(P)(RR)(SSPP)(RS)(RRRPP)(R)(PRSPS)(RRPP)(RRRSS)(RRRSS)(RSP)(SRPR)(P\
          PS)",
         "(SPP)");
    test("(SSS)(SSPR)(SSRR)(P)(PRRSP)(RRRPP)(PR)(P)(PS)(PPR)(R)(SRPSR)(R)(S)(SSPRS)(SRPR)(PPPR)(S\
          RS)",
         "(SSRR)");
    test("(PR)(R)(PRPS)(PR)(S)(PS)(R)(P)(R)(SS)(RP)(SS)(SP)(R)(SPR)(RPR)(PSP)(PPPS)(SPRPR)",
         "(RP)");
    test("(SPS)(SRPR)(P)(SPPS)(SS)(RS)(SRPPS)(SRSPS)(RSR)(SRPR)(P)(SPSS)(SRS)(SP)(RSRRP)(PP)(SR)(\
          RPRP)(P)(SPPPS)",
         "(RSR)");
    test("(SSRSP)(SPRRPRSPS)(SPSPS)(PRPR)(SPPRP)(RS)(SPSSPRRS)(PSPPRPSSP)(PSRRRRRP)(SPPRS)(SRRP)(\
          SP)(SRSPRPSP)(PPSRRRSR)(PPPSSRSR)(PRPSPS)(SRR)(RP)(SP)(RSRPSPSSRS)",
         "(RS)");
    test("(RRPS)(SRPR)(PS)(SPPS)(SS)(RS)(SRPPS)(SRSPS)(RSR)(SRPR)(P)(SPSS)(SRS)(SP)(RSRRP)(PP)(SR\
          )(RPRP)(P)(SPPPS)",
         "(RRPS)");
    test("(S)(PRSRR)(PP)(PSSSS)(SR)(SRRP)(PRRPR)(PRSS)(SPPS)(SS)(SPPR)(SSRSR)(PSRPP)(RSP)(R)(P)(P\
          PP)(SS)(SP)(SSSS)(RRSR)",
         "(SRRP)");
    test("(PS)(R)(R)(S)(S)(SSP)(RPPP)(RPSP)(RPRR)(R)(SRRSS)(RSR)(PS)(PRP)(SSSS)(S)(SSSR)(SS)(PSP)\
          (RS)(PSRSR)(SR)",
         "(SR)");
    test("(RSPSS)(RRSSR)(S)(RRS)(PSSRR)(S)(RPRRP)(RS)(PS)(RR)(R)(PSRR)(RPPRP)(SSS)(S)(R)(R)(SRSS)\
          (PR)(S)(RRPPS)(S)(SSPRR)",
         "(RRS)");
    test("(PSSS)(RRRPR)(PRPP)(RSSS)(RR)(RP)(PPS)(PSR)(SPS)(SRSS)(R)(RR)(SPRSR)(RSPRP)(RRSP)(SSRRP\
          )(RSSSR)(PPSS)(PRS)(RRSRS)(PS)(SS)(P)(SPR)",
         "(PRPP)");
    test("(RSRPSS)(RPPRPRRSP)(PRPSRSRPPP)(SSRSSRS)(RPS)(SP)(PPPPPSSP)(RRRPSR)(PSR)(SRSRSSR)(RPSSS\
          RP)(RRSPSSSPPR)(RS)(SRRRSPRP)(PR)(RSSRPSSS)(PPRRRRRR)(RRSRP)(RRR)(PSPRSSPRP)(PRPPRSSRP)\
          (SPPSPSS)(PSS)(RPS)(P)(RRSRSP)(PS)(RRPSSSRR)(RR)(PPPSPRPR)(PS)(PRSSRPR)(RRP)(PSRPR)(PS)\
          (R)(RRPP)(SSPPSS)(SRPSSS)(RRSRRPRPP)",
         "(SPPSPSS)");

    println!("")
}
