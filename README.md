# xjson
json tools,

## usage :

``` json

example json string:
{
    "name": "十五",
    "age": 14,
    "gender": "male",
    "height": 165,
    "grade": null,
    "vip": false,
    "info": {
        "country": "China",
        "phone": "922"
    },
    "school": "\"LearnRust\" School",
    "skills": [
        "Rust",
        "Golang",
        "C#",
        "Python"
    ],
    "own":[ 
        10,
        20,
        30
    ] 
}

```

``` rust 

[dependencies]
xjson={git ="https://github.com/yixinin/xjson" } 




extern crate xjson;


fn main() {
    let phone = xjson::get_string(json, "info.phone"); 
    let age: i32 = xjson::get(json, "age").unwrap();
    let skills: Vec<&str> = xjson::get_string_array(json, "skills");
    let own: Vec<i32> = xjson::get_array(json, "own");
}
```
