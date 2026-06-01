use std::io::{Result, Write};
use std::fs::{File, read_dir};

static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data().unwrap();
}

fn insert_app_data() -> Result<()> {
    let mut f = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../user/src/bin")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let name = dir_entry.unwrap().file_name().into_string().unwrap();
            name.split('.').next().unwrap().to_string()
        })
        .collect();
    apps.sort();

    let _ = writeln!(f, "    .align 3");
    let _ = writeln!(f, "    .section .data");
    let _ = writeln!(f, "    .global _num_app");
    let _ = writeln!(f, "_num_app:");
    let _ = writeln!(f, "    .quad {}", apps.len());

    for i in 0..apps.len() {
        let _ = writeln!(f, "    .quad app_{}_start", i);
    }
    let _ = writeln!(f, "    .quad app_{}_end", apps.len() - 1);

    for (idx, app) in apps.iter().enumerate() {
        let _ = writeln!(f, "");
        let _ = writeln!(f, "    .section .data");
        let _ = writeln!(f, "    .global app_{0}_start", idx);
        let _ = writeln!(f, "    .global app_{0}_end", idx);
        let _ = writeln!(f, "app_{0}_start:", idx);
        let _ = writeln!(f, "    .incbin \"../user/target/riscv64gc-unknown-none-elf/release/{}.bin\"", app);
        let _ = writeln!(f, "app_{0}_end:", idx);
    }

    Ok(())
}
