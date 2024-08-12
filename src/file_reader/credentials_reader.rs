use std:: { 
    fs, error,
    collections::HashMap,
};

pub struct CredentialsReader {
    file: String,
}

impl CredentialsReader {
    pub fn new(file_path: String) -> Self {
        CredentialsReader{ file: file_path }
    }

    pub fn get_credentials(&self) -> HashMap<String, String> {
        let credentials_raw: Vec<char> = match self.read_credentials_file() {
            Ok(v) => v,
            Err(e) => panic!("Problems reading credentials {}", e),
        };

        let credentials_map: HashMap<String, String> = match self.parse_xml_file(&credentials_raw) {
            Ok(v) => v,
            Err(e) => panic!("Error parsing credentials {}", e),
        };

        credentials_map
    }

    fn read_credentials_file(&self) -> Result<Vec<char>, Box<dyn error::Error + 'static>> {
        let data: Vec<u8> = match fs::read(&self.file){
            Ok(v) => v,
            Err(e) => panic!("Cannot find file: {} {}", self.file, e),
        };

        Ok(data.into_iter().map(|byte| byte as char).collect())
    }

    fn parse_xml_file(&self, raw_data: &Vec<char>) -> Result<HashMap<String, String>, Box<dyn error::Error + 'static>> {
        let n: usize = raw_data.len();

        let mut tmp: String = String::new();
        let mut credentials_map: HashMap<String, String> = HashMap::new();

        let mut entry_desc_stack: Vec<String> = Vec::new();
        let mut entry_stack: Vec<String> = Vec::new();

        /*
            Status 0: Parse contents
            Status 1: Parse content in <...>
            Status 2: Parse conten in </...>
        */
        let mut parse_status: i32 = 0;
        let mut current_line: u32 = 0;

        let mut i: usize = 0;

        while i < n {
            if raw_data[i] == '<' {
                if i == n-1 { panic!("There is an open < at the last position"); }

                if entry_desc_stack.len() > 0 {
                    entry_stack.push(tmp.to_owned());
                }

                tmp = String::new();

                if raw_data[i+1] != '/' { //save because of check before
                    parse_status = 1;
                    i += 1;
                } else {
                    if entry_desc_stack.len() == 0 { panic!("Found closing line without the corresponding object at line: {}", current_line); }

                    parse_status = 2;
                    i += 2;
                }

                continue;
            }

            if raw_data[i] == '>' {
                match parse_status {
                    1 => {
                        entry_desc_stack.push(tmp.to_owned());
                    },
                    2 => {
                        match entry_desc_stack.pop() {
                            Some(v) => {
                                if tmp != v { panic!("Closing line doesn't corespond to open line: {} {} at line: {}", v, tmp, current_line); }
                                
                                let mut full_path:String = String::new();

                                for path in entry_desc_stack.iter() {
                                    full_path.push_str(path);
                                    full_path.push('.');
                                }

                                full_path.push_str(&v);

                                match entry_stack.pop() {
                                    Some(p) => credentials_map.insert(full_path, p),
                                    None => panic!("Couldn't find an entry for open line {}", v),
                                }
                            
                            },
                            None => panic!("Found closing line without the corresponding object at line: {}", current_line),
                        };
                    },
                    _ => panic!("Found an > without an open < at line: {}", current_line),
                };

                tmp = String::new();
                parse_status = 0;
                i += 1;
                
                continue;
            }

            if raw_data[i] == '\n' { current_line += 1; }

            if raw_data[i] != '\n' && raw_data[i] != '\r' && raw_data[i] != ' ' {
                tmp.push(raw_data[i]);
            }

            i += 1;
        }

        Ok(credentials_map)
    }
}