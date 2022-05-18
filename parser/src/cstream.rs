use std::fs;

#[derive(Debug)]

pub struct CStream {
    pub filename: String,
    pub line_num: i32,
    pub char_pos: i32,
    pub cur_char: usize,
    pub file_data: Vec<u8>,
    pub file_str: String
}

impl CStream {
    //read file to a vector and initialize position trackers
    pub fn new(s: &str) -> CStream {
        CStream {
            filename: s.to_string(),
            line_num: -1,
            char_pos: -1,
            cur_char: 0,
            file_data: fs::read(s).expect("Unable to read file"),
            file_str: fs::read_to_string(s).expect("Unable to read file")
        }
    }
    //check if we are at the end of the vector
    pub fn more_available(&mut self) -> bool {
        if usize::from(self.cur_char) == self.file_data.len()-2 {
            return false;
        }
        return true;
    }
    //return the current char 
    pub fn get_cur_char(&mut self) -> char {
        return self.file_data[self.cur_char] as char;
    }
    //return the next char
    pub fn get_next_char(&mut self) -> char {
        //if we're at the start of the file
        if self.line_num == -1 && self.char_pos == -1 {
            self.line_num = 0;
            self.char_pos = 0;
            return self.file_data[self.cur_char] as char;
        }
        self.cur_char += 1;
        //if we're at the end of a line
        if self.file_data[self.cur_char] as char == '\n' {
            self.char_pos = 0;
            self.line_num += 1;
            self.cur_char += 1;
        }
        //if no special case applies
        else {
            self.char_pos += 1;
        }
        return self.file_data[self.cur_char] as char;
    }
    //return the next char, without modifying position trackers
    pub fn peek_next_char(&mut self) -> char {
        if self.line_num == -1 && self.char_pos == -1 {
            return self.file_data[self.cur_char] as char;
        }

        if self.file_data[self.cur_char+1] as char == '\n' {
            return self.file_data[self.cur_char+2] as char;
        }
        else {
            return self.file_data[self.cur_char+1] as char;
        }
    }
    //return the kth ahead char, without modifying position trackers
    pub fn peek_ahead_char(&mut self, k: i32) -> char {
        let mut new_char_num = self.cur_char;
        let mut count = 0;
        //account for being at the start of the file
        if self.line_num == -1 && self.char_pos == -1 {
            count += 1;
        }
        while count <= k {
            if self.file_data[new_char_num] as char == '\n' {
                new_char_num += 1;
            }
            else {
                new_char_num += 1;
                count += 1;
            }
        }
        if self.file_data[new_char_num] as char == '\n' {
            return self.file_data[new_char_num+1] as char;
        }
        return self.file_data[new_char_num] as char;
    }
}