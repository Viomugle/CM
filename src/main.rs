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
    if let Some(name) = matches.value_of("name") {
        match create_by_name(name) {
            Ok(path) => {
                println!("Create project success! Path:{}", path);
            }
            Err(e) => {
                println!("Create project failed! Error:{}", e);
            }
        }
    }
    if let Some(path) = matches.value_of("clear") {
        clear_by_path(path);
    }
    if let Some(path) = matches.value_of("build") {
        build_project(path);
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
        + "file(GLOB SRC_FILES ${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp)\n"
        +"set(CMAKE_EXPORT_COMPILE_COMMANDS ON)\n"
        + "add_executable(${PROJECT_NAME} ${SRC_FILES})\n";
    CMakeFile.write_all(buf.as_bytes());
    Ok(current_dir)
}
//recursively clear "Makefile" "CMakeCache.txt" "CMakeFiles" "cmake_install.cmake" "CMakeLists.txt" "build" files
fn clear_by_path(pth:&str)
{
   if pth.ends_with("/")// is a dir
    {
         let mut dir=std::fs::read_dir(pth).unwrap();
         for entry in dir
         {
              let entry=entry.unwrap();
              let path=entry.path();
              let path_str=path.display().to_string();
              if path_str.ends_with("Makefile")||path_str.ends_with("CMakeCache.txt")||path_str.ends_with("CMakeFiles")||path_str.ends_with("cmake_install.cmake")||path_str.ends_with("CMakeLists.txt")||path_str.ends_with("build")
              {
                std::fs::remove_dir_all(path.clone());
                println!("remove {}",path_str.clone());
              }
              else
              {
                clear_by_path(&path_str);
              }
         }
    }
    else
    {
        let path=pth.to_string();
        if path.ends_with("Makefile")||path.ends_with("CMakeCache.txt")||path.ends_with("CMakeFiles")||path.ends_with("cmake_install.cmake")||path.ends_with("CMakeLists.txt")||path.ends_with("build")
        {
            println!("remove {}",path);
            std::fs::remove_dir_all(path);
        }
    }   
}
//build project
fn build_project(path: &str) {
    let mut build_path = path.to_string();
    build_path.push_str("/build");
    let mut build = std::process::Command::new("cmake")
        .arg("-S")
        .arg(path)
        .arg("-B")
        .arg(build_path.clone())
        .output()
        .expect("cmake build error");
    println!("{}", string::String::from_utf8_lossy(&build.stdout));
    let mut make = std::process::Command::new("make")
        .arg("-C")
        .arg(build_path.clone())
        .output()
        .expect("make error");
    println!("{}", string::String::from_utf8_lossy(&make.stdout));
}