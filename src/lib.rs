use std::str::FromStr;

enum ValueType {
    Str,
    Value,
    Array,
    Object,
    UnKonw,
}

fn get_sub<'a>(json: &'a str, key: &str) -> (&'a str, ValueType) {
    let jb = json.as_bytes();
    let kb = key.as_bytes();

    let mut deep = 0; // 对象搜索深度
    let mut arr_deep = 0;
    let mut pre_byte = b'{'; //上一个有效字符
    let mut pre_sign = 0; //上一个引号位置

    let mut inside = false;

    for (i, &item) in jb.iter().enumerate() {
        if item == b' ' || item == b'\r' || item == b'\n' || item == b'\t' {
            continue;
        }

        if item == b'"' && pre_byte != b'\\' {
            inside = !inside;
        }
        // println!("{},{}", arr_deep, deep);
        if item == b',' && deep == 1 && arr_deep == 0 && !inside {
            let (k, obj) = json_split(&json[pre_sign + 1..i]);
            if get_json_value(k) == key {
                return get_json_obj(obj);
            }
            pre_sign = i;
        }
        if item == b'[' && !inside {
            arr_deep += 1;
        }
        if item == b']' && !inside {
            arr_deep -= 1;
        }
        if item == b'{' && !inside {
            deep += 1;
        }
        if item == b'}' && !inside {
            deep -= 1;
        }
        pre_byte = item;
    }

    let (k, obj) = json_split(&json[pre_sign + 1..jb.len() - 1]);

    if get_json_value(k) == key {
        return get_json_obj(obj);
    }

    ("", ValueType::UnKonw)
}

fn get_json_array<T>(s: &str) -> Vec<T>
where
    T: FromStr,
{
    let sb = s.as_bytes();
    let mut inside = false;
    let mut cs: Vec<usize> = Vec::new();
    let mut deep = 0; // 对象搜索深度
    let mut arr_deep = 0;
    let mut pre_byte = b'['; //上一个有效字符
    for (i, &item) in sb.iter().enumerate() {
        if item == b' ' || item == b'\r' || item == b'\n' || item == b'\t' {
            continue;
        }

        if item == b'"' && pre_byte != b'\\' {
            inside = !inside;
        }

        if item == b',' && deep == 0 && arr_deep == 1 && !inside {
            cs.push(i);
        }

        if item == b'[' && !inside {
            arr_deep += 1;
        }
        if item == b']' && !inside {
            arr_deep -= 1;
        }
        if item == b'{' && !inside {
            deep += 1;
        }
        if item == b'}' && !inside {
            deep -= 1;
        }
        pre_byte = item;
    }
    let mut array: Vec<T> = Vec::new();
    for (i, &item) in cs.iter().enumerate() {
        if i == 0 {
            let v = get_json_value(&s[1..item]);
            match T::from_str(v) {
                Ok(value) => array.push(value),
                _ => (),
            }
            continue;
        }
        let v = get_json_value(&s[cs[i - 1] + 1..item]);
        match T::from_str(v) {
            Ok(value) => array.push(value),
            _ => (),
        }
    }
    let v = get_json_value(&s[cs[cs.len() - 1] + 1..sb.len() - 1]);
    match T::from_str(v) {
        Ok(value) => array.push(value),
        _ => (),
    }
    return array;
}

fn get_json_str_array(s: &str) -> Vec<&str> {
    let sb = s.as_bytes();
    let mut inside = false;
    let mut cs: Vec<usize> = Vec::new();
    let mut deep = 0; // 对象搜索深度
    let mut arr_deep = 0;
    let mut pre_byte = b'['; //上一个有效字符
    for (i, &item) in sb.iter().enumerate() {
        if item == b' ' || item == b'\r' || item == b'\n' || item == b'\t' {
            continue;
        }

        if item == b'"' && pre_byte != b'\\' {
            inside = !inside;
        }

        if item == b',' && deep == 0 && arr_deep == 1 && !inside {
            cs.push(i);
        }

        if item == b'[' && !inside {
            arr_deep += 1;
        }
        if item == b']' && !inside {
            arr_deep -= 1;
        }
        if item == b'{' && !inside {
            deep += 1;
        }
        if item == b'}' && !inside {
            deep -= 1;
        }
        pre_byte = item;
    }
    let mut array: Vec<&str> = Vec::new();
    for (i, &item) in cs.iter().enumerate() {
        if i == 0 {
            let v = get_json_value(&s[1..item]);
            array.push(v);
            continue;
        }
        let v = get_json_value(&s[cs[i - 1] + 1..item]);
        array.push(v);
    }
    let v = get_json_value(&s[cs[cs.len() - 1] + 1..sb.len() - 1]);
    array.push(v);
    return array;
}

fn get_json_value<'a>(s: &'a str) -> &'a str {
    let mut fi = 0;
    let mut li = 0;
    let mut is_str = false;
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' || item == b'\r' || item == b'\n' || item == b'\t' {
            continue;
        } else {
            if fi == 0 {
                fi = i;
                if item == b'"' {
                    is_str = true;
                }
            }
            li = i;
        }
    }
    if is_str {
        return &s[fi + 1..li];
    }
    &s[fi..li + 1]
}

fn get_json_array_obj<'a>(s: &'a str) -> &'a str {
    let mut fi = 0;
    let mut li = 0;
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b'[' && fi == 0 {
            fi = i;
        }
        if item == b']' {
            li = i;
        }
    }
    &s[fi..li + 1]
}
fn get_json_obj_str<'a>(s: &'a str) -> &'a str {
    let mut fi = 0;
    let mut li = 0;
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b'{' && fi == 0 {
            fi = i;
        }
        if item == b'}' {
            li = i;
        }
    }
    &s[fi..li + 1]
}
fn json_split<'a>(s: &'a str) -> (&'a str, &'a str) {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b':' {
            return (&s[..i], &s[i + 1..]);
        }
    }
    ("", "")
}
fn get_json_obj(s: &str) -> (&str, ValueType) {
    let mut t = ValueType::UnKonw;
    for (i, &item) in s.as_bytes().iter().enumerate() {
        match t {
            ValueType::UnKonw => {
                if item == b' ' || item == b'\r' || item == b'\n' || item == b'\t' {
                    continue;
                } else {
                    match item {
                        b'{' => t = ValueType::Object,
                        b'[' => t = ValueType::Array,
                        b'"' => t = ValueType::Str,
                        _ => {
                            t = ValueType::Value;
                            break;
                        }
                    }
                }
            }
            _ => break,
        }
    }
    match t {
        ValueType::Str | ValueType::Value => return (get_json_value(s), t),
        ValueType::Array => return (get_json_array_obj(s), t),
        ValueType::Object => return (get_json_obj_str(s), t),
        _ => (),
    }
    ("", t)
}

pub fn get_string(json: &str, key: &str) -> String {
    let keys: Vec<&str> = key.split('.').collect();

    let mut o = json;
    for k in keys {
        let (obj, _) = get_sub(o, k);
        o = obj;
    }
    String::from(o)
}
pub fn get<T>(json: &str, key: &str) -> Result<T, T::Err>
where
    T: FromStr,
{
    let keys: Vec<&str> = key.split('.').collect();

    let mut o = json;
    for k in keys {
        let (obj, _) = get_sub(o, k);
        o = obj;
    }
    T::from_str(o)
}
pub fn get_array<T>(json: &str, key: &str) -> Vec<T>
where
    T: FromStr,
{
    let keys: Vec<&str> = key.split('.').collect();

    let mut o = json;
    for k in keys {
        let (obj, _) = get_sub(o, k);
        o = obj;
    }
    let arr: Vec<T> = get_json_array(o);
    arr
}
pub fn get_string_array<'a>(json: &'a str, key: &str) -> Vec<&'a str> {
    let keys: Vec<&str> = key.split('.').collect();

    let mut o = json;
    for k in keys {
        let (obj, _) = get_sub(o, k);
        o = obj;
    }
    let arr: Vec<&str> = get_json_str_array(o);
    arr
}
