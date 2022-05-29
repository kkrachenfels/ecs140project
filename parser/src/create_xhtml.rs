use crate::File;
use std::io::Write;
use crate::token::*;

pub fn create(all_tokens:Vec<Token>){
    let mut file =File::create("color.xhtml").expect("Could not create file");
    file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n").expect("Unable to write to file");
    file.write_all(b"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n").expect("Unable to write to file");
    file.write_all(b"<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">\n").expect("Unable to write to file");
    file.write_all(b"<head>\n <title>Token XHTML</title>\n</head>\n<body style=\"background-color:navy;font-family:Courier New;color:orange\">\n").expect("Unable to write to file");
    file.write_all(b"<p>").expect("Unable to write to file");
    for token in all_tokens.iter()
    {
        if token.text=="<"{file.write_all(b"&lt;").expect("Unable to write to file");continue}
        if token.text==";" || token.text=="{" || token.text=="}" {
            file.write_all(b"<span style=\"color: white\">").expect("Unable to write to file");
            if token.text==";" {
                file.write_all(token.text.as_bytes()).expect("Unable to write to file");
                file.write_all(b"<br />").expect("Unable to write to file");
            }
            if token.text=="{"{
                file.write_all(b"<br />{<br />").expect("Unable to write to file");
            }
            if token.text=="}"{
                file.write_all(b"<br />}<br />").expect("Unable to write to file");
            }
            file.write_all(b"</span>").expect("Unable to write to file");
        }
        else
        {
            if token.token_type==TokenType::IntConstant|| token.token_type==TokenType::FloatConstant {
                file.write_all(b"<span style=\"color: aqua\">").expect("Unable to write to file");
                file.write_all(b"<b>").expect("Unable to write to file");
                file.write_all(token.text.as_bytes()).expect("Unable to write to file");
                file.write_all(b"</b></span>").expect("Unable to write to file");
            }
            else if  token.token_type==TokenType::Keyword || token.token_type==TokenType::Operator{
                file.write_all(b"<span style=\"color: white\">").expect("Unable to write to file");
                file.write_all(b"<b>").expect("Unable to write to file");
                file.write_all(token.text.as_bytes()).expect("Unable to write to file");
                file.write_all(b"</b></span>").expect("Unable to write to file");
            }
            else if  token.token_type==TokenType::Identifier{
                file.write_all(b"<span style=\"color: yellow\">").expect("Unable to write to file");
                file.write_all(token.text.as_bytes()).expect("Unable to write to file");
                file.write_all(b"</span>").expect("Unable to write to file");
            }


        }
        file.write_all(b" ").expect("Unable to write to file");
    }
    file.write_all(b"\n</p>").expect("Unable to write to file");
    file.write_all(b"</body>\n</html>").expect("Unable to write to file");
}