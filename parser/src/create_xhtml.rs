use crate::File;
use std::io::Write;
use crate::token::*;


pub fn indent_line(fp: &mut File, num_indents:i32) {
    for indent in 0..num_indents {
        fp.write_all(b"&nbsp;&nbsp;&nbsp;").expect("Unable to write to file");
    }
}

pub fn create(all_tokens:Vec<Token>){
    let mut file =File::create("color.xhtml").expect("Could not create file");
    file.write_all(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n").expect("Unable to write to file");
    file.write_all(b"<!DOCTYPE html PUBLIC \"-//W3C//DTD XHTML 1.0 Transitional//EN\" \"http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd\">\n").expect("Unable to write to file");
    file.write_all(b"<html xmlns=\"http://www.w3.org/1999/xhtml\" xml:lang=\"en\">\n").expect("Unable to write to file");
    file.write_all(b"<head>\n <title>X Formatted file</title>\n</head>\n<body bgcolor=\"navy\" text=\"orange\" link=\"orange\" vlink=\"orange\">\n<font face=\"Courier New\">\n").expect("Unable to write to file");
    file.write_all(b"<p>").expect("Unable to write to file");
    let mut indent_level = 0;
    let mut token_num = 0;
    for token in all_tokens.iter()
    {
        if token.text=="<"{
            file.write_all(b"<span style=\"color: white\"><b>").expect("Unable to write to file");
            file.write_all(b"&lt; ").expect("Unable to write to file");
            file.write_all(b"</b></span>").expect("Unable to write to file");
            token_num += 1;
            continue
        }
        if token.text=="<="{
            file.write_all(b"<span style=\"color: white\"><b>").expect("Unable to write to file");
            file.write_all(b"&lt;= ").expect("Unable to write to file");
            file.write_all(b"</b></span>").expect("Unable to write to file");
            token_num += 1;
            continue
        }
        if token.text==";" || token.text=="{" || token.text=="}" {
            file.write_all(b"<span style=\"color: white\"><b>").expect("Unable to write to file");
            if token.text==";" {
                file.write_all(token.text.as_bytes()).expect("Unable to write to file");
                file.write_all(b"<br />").expect("Unable to write to file");
                if all_tokens[token_num+1].text == "}" {
                    indent_line(&mut file, indent_level-1);
                }
                else {
                    indent_line(&mut file, indent_level);
                }
            }
            if token.text=="{"{
                indent_level += 1;
                file.write_all(b"{<br />").expect("Unable to write to file");
                indent_line(&mut file, indent_level);
            }
            if token.text=="}"{
                indent_level -= 1;
                file.write_all(b"}<br />").expect("Unable to write to file");
                if token_num < all_tokens.len()-1 && all_tokens[token_num+1].text == "}" {
                    indent_line(&mut file, indent_level-1);
                }
                else {
                    indent_line(&mut file, indent_level);
                }
            }
            file.write_all(b"</b></span>").expect("Unable to write to file");
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
        token_num += 1;
    }
    file.write_all(b"\n</p>").expect("Unable to write to file");
    file.write_all(b"</font></body>\n</html>").expect("Unable to write to file");
}