use std::{self, io::Write, string};

use clap::{App, Arg};

fn main() {
    let matches = App::new("CMake Manager")
        .version("0.1.0")
        .author("NilnaUggnaw <2451051674@qq.com>")
        .about("Init a CMake Project or build it")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .takes_value(true)
                .help("Create a CMake project with given name"),
        )
        .arg(
            Arg::with_name("clear")
                .short("c")
                .long("clear")
                .takes_value(true)
                .help("Clear a CMake project"),
        )
        .arg(
            Arg::with_name("build")
                .short("b")
                .long("build")
                .takes_value(true)
                .help("Build a CMake project"),
        )
        .get_matches();
    if matches.value_of("name") != None {
        let name = matches.value_of("name").unwrap_or("Default");
        let root_path = create_by_name(name).unwrap();
    } else if matches.value_of("clear") != None {
        let path = matches.value_of("clear").unwrap_or("./");
        if path.starts_with("./")
        //当前目录下面
        {
            clear_by_path(path);
        } else {
            let local_path =format!("./{}",path);
            clear_by_path(&local_path);
        }
    }
}

fn create_by_name(name: &str) -> Result<String, String> {
    let current_dir = std::env::current_dir().unwrap().display().to_string();
    let build_dir = current_dir.clone() + "/build";
    let include_dir = current_dir.clone() + "/include";
    let src_dir = current_dir.clone() + "/src";
    println!("Path: {}\nName:{}", current_dir, name);
    let full_name = current_dir.clone() + "/CMakeLists.txt";
    let mut CMakeFile = std::fs::File::create(full_name).expect("create file error");
    let include_path = std::fs::create_dir_all(include_dir).unwrap();
    let build_path = std::fs::create_dir_all(build_dir).unwrap();
    let src_path = std::fs::create_dir_all(src_dir.clone()).unwrap();
    let main_path=src_dir.clone()+"/main.cpp";
    let mut main_file=std::fs::File::create(main_path).expect("create main.cpp error");
    let mainbuf=b"#include <iostream>\nint main()\n{\n\tstd::cout<<\"hello world!\"<<std::endl;\n\treturn 0;\n}";
    main_file.write_all(mainbuf);
    let buf = format!("project({})\n", name)
        + "include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)\n"
        + "set(EXECUTABLE_OUTPUT_PATH ${CMAKE_CURRENT_SOURCE_DIR}/build)\n"
        + "files(GLOB SRC_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)\n"
        +"set(CMAKE_EXPORT_COMPILE_COMMANDS ON)\n"
        + "add_executable(${PROJECT_NAME} ${SRC_FILES})\n";
    CMakeFile.write_all(buf.as_bytes());
    Ok(current_dir)
}
fn clear_by_path(pth:&str){}