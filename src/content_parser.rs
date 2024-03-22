use log::debug;
// ========Public functions========

pub fn parse_content(content: &str) -> Vec<String>{
    // Validate content is matching expected format
    // Should contains public enum
    let class_name = get_class_name(content);
    let enum_fields = get_enum_fields(content);
    let enum_values = get_enum_values(content);
    to_csv_format(&enum_fields, &enum_values)
}

// ========Private functions========

fn get_class_name(content: &str) -> String {
    let class_name = content
        .split("public enum")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()[0];
    class_name.to_string()
}

fn get_enum_fields(content: &str) -> Vec<String> {
    // find out the private final variables
    let fields = content.split("private final").collect::<Vec<&str>>();
    let mut enum_fields = Vec::new();
    for field in fields {
        // ignore the first two element since it is the type declaration and final keyword
        let field_name = field.split(" ").collect::<Vec<&str>>()[2];
        // remove \n and ;
        let field_name = field_name.replace("\n", "").replace(";", "");
        enum_fields.push(field_name.to_string());
    }
    // replace first element since it is not a field with enum
    enum_fields[0] = "enum_value".to_string();
    // debug!("enum_fields: {:?}", enum_fields);
    enum_fields
}

fn get_enum_values(content: &str) -> Vec<String> {
    // enum value should match the regex pattern /(\S+)\(\s*"(\d+)"\s*,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,\s*ErrorLevel\.(\w+)\)/gm
    let re = regex::Regex
        ::new(r#"(\S+)\(\s*"(\d+)"\s*,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,\s*ErrorLevel\.(\w+)\)"#)
        .unwrap();

    // find all matches
    let mut enum_values = Vec::new();
    for cap in re.captures_iter(content) {
        let enum_value = format!("{},{},{},{},{}", &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]);
        // debug!("enum_value: {}", enum_value);
        enum_values.push(enum_value);
    }
    // print count of enum values
    // debug!("enum_values count: {}", enum_values.len());
    enum_values
}

fn to_csv_format(enum_fields: &Vec<String>, enum_values: &Vec<String>) -> Vec<String> {
    let mut csv_content = Vec::new();
    let mut header = String::new();
    for field in enum_fields {
        // first field should not have comma
        if field == &enum_fields[0] {
            header.push_str(field);
            continue;
        }
        else {
            header.push_str(format!(",{}", field).as_str());
        }
    }
    csv_content.push(header);
    for value in enum_values {
        csv_content.push(format!("{}", value));
    }
    for c in &csv_content {
        // debug!("line num: {}", c.len());
        // debug!("csv_content: {}", c);
    }
    csv_content
}