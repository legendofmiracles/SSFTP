#[path = "server/parseFtp.rs"] mod parseFtp;
// #[path = "server/parseSftp.rs"] mod parseSftp;

#[cfg(test)]
mod tests {
    #[test]
    fn malicious(){
        parseFtp::checkForMal("MEMZ.exe"); // definently a virus lmao
        parseFtp::checkForMal("LaTeXtutorial.pdf");
    }

    #[test]
    fn audio(){
        parseFtp::checkForMal("script.mp3");
        parseFtp::checkForMal("nah.exe");
    }
    
    #[test]
    fn video(){
        parseFtp::checkForMal("lmao.mp4");
        parseFtp::checkForMal("when.mp5"); // not a valid type
    }

    #[test]
    fn text(){
        // todo: later
    }
}