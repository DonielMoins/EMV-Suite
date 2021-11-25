use bytes::Bytes;

pub struct CmdHeader {
    CLA: [u8; 1],
    INS: [u8; 1],
    P1: [u8; 1],
    P2: [u8; 1],
}
pub struct CmdBody {
    Lc: [u8; 1],
    Data: [u8; 31],
    Le: [u8; 1],
}

impl CmdHeader {
    pub fn new(CLA: [u8; 1], INS: [u8; 1], P1: [u8; 1], P2: [u8; 1]) -> Self {
        Self { CLA, INS, P1, P2 }
    }
}
impl CmdBody {
    pub fn new(Lc: [u8; 1], Data: [u8; 31], Le: [u8; 1]) -> Self {
        Self { Lc, Data, Le }
    }
}
pub struct Command {
    head: CmdHeader,
    body: CmdBody,
}

impl Command {
    pub fn new(head: CmdHeader, body: CmdBody) -> Self {
        Self { head, body }
    }
}

pub struct ResBody {
    Data: [u8; 31], // I think lol
}

impl ResBody {
    pub fn new(Data: [u8; 31]) -> Self {
        Self { Data }
    }
}
pub struct ResTrailer {
    SW1: [u8; 1],
    SW2: [u8; 1],
}

impl ResTrailer {
    pub fn new(SW1: [u8; 1], SW2: [u8; 1]) -> Self {
        Self { SW1, SW2 }
    }
}
pub struct Response {
    body: ResBody,
    trailer: ResTrailer,
}
