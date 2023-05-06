use std::{
    env,
    fs::{self, File},
    io::Write,
    marker::PhantomData,
    path::PathBuf,
    process::{exit, Command},
};

#[derive(PartialEq, Debug)]
enum Lang {
    Interpret,
    C,
    Java,
    Python,
    JavaScript,
    Rust,
    Go,
}

struct Interpret;
struct C;
struct Java;
struct Python;
struct JavaScript;
struct Rust;
struct Go;

trait Brainfuck {
    fn prefix(&self, f: &mut File);
    fn suffix(&self, f: &mut File);

    fn inc(&self, f: &mut File);
    fn dec(&self, f: &mut File);

    fn add(&self, f: &mut File);
    fn sub(&self, f: &mut File);

    fn put(&self, f: &mut File);
    fn get(&self, f: &mut File);

    fn rep(&self, f: &mut File);
    fn end(&self, f: &mut File);

    fn extension(&self) -> String;
    fn compile(&self, file_path: PathBuf) -> PathBuf;
    fn run(&self, file_path: PathBuf);
}
struct Generator<Target> {
    target: PhantomData<Target>,
}

impl<Target> Default for Generator<Target> {
    fn default() -> Self {
        Self {
            target: Default::default(),
        }
    }
}

impl Brainfuck for Generator<C> {
    fn prefix(&self, f: &mut File) {
        f.write_all(b"#include <stdio.h>\n").unwrap();
        f.write_all(b"int main() {").unwrap();
        f.write_all(b"char arr[30000] = {0}; char *ptr = arr;")
            .unwrap();
    }

    fn suffix(&self, f: &mut File) {
        f.write_all(b"return 0;}").unwrap();
    }

    fn inc(&self, f: &mut File) {
        f.write_all(b"++ptr;").unwrap();
    }

    fn dec(&self, f: &mut File) {
        f.write_all(b"--ptr;").unwrap();
    }

    fn add(&self, f: &mut File) {
        f.write_all(b"++*ptr;").unwrap();
    }

    fn sub(&self, f: &mut File) {
        f.write_all(b"--*ptr;").unwrap();
    }

    fn put(&self, f: &mut File) {
        f.write_all(b"putchar(*ptr);").unwrap();
    }

    fn get(&self, f: &mut File) {
        f.write_all(b"*ptr = getchar();").unwrap();
    }

    fn rep(&self, f: &mut File) {
        f.write_all(b"while (*ptr) {").unwrap();
    }

    fn end(&self, f: &mut File) {
        f.write_all(b"}").unwrap();
    }

    fn extension(&self) -> String {
        "c".to_string()
    }

    fn compile(&self, file_path: PathBuf) -> PathBuf {
        println!("  [INFO] Compiling {:?}", file_path);

        let mut out_file = file_path.clone();
        out_file.set_extension("c.exe");

        let out = Command::new("cc")
            .arg(file_path)
            .arg("-O3")
            .args(["-o", out_file.to_str().unwrap()])
            .output()
            .unwrap();

        if !out.stderr.is_empty() {
            eprintln!("{}", String::from_utf8(out.stderr).unwrap());
        }
        if !out.stdout.is_empty() {
            println!("{}", String::from_utf8(out.stdout).unwrap());
        }
        out_file
    }

    fn run(&self, file_path: PathBuf) {
        println!("  [INFO] Running {:?}", file_path);

        let out = Command::new(file_path.to_str().unwrap()).output().unwrap();

        if !out.stderr.is_empty() {
            eprintln!("{}", String::from_utf8(out.stderr).unwrap());
        }
        if !out.stdout.is_empty() {
            println!("{}", String::from_utf8(out.stdout).unwrap());
        }
    }
}

impl Brainfuck for Generator<Go> {
    fn prefix(&self, f: &mut File) {
        f.write_all(b"package main;").unwrap();
        f.write_all(b"import \"bufio\";").unwrap();
        f.write_all(b"import \"fmt\";").unwrap();
        f.write_all(b"import \"os\";").unwrap();

        f.write_all(b"var in = bufio.NewReader(os.Stdin);").unwrap();

        f.write_all(b"func getchar () uint8 { r,_,_ := in.ReadRune(); return uint8(r);};")
            .unwrap();

        f.write_all(b"func main() {").unwrap();

        f.write_all(b"arr := make([]uint8, 30000);").unwrap();
        f.write_all(b"ptr := 0;").unwrap();
    }

    fn suffix(&self, f: &mut File) {
        f.write_all(b"}").unwrap();
    }

    fn inc(&self, f: &mut File) {
        f.write_all(b"ptr+=1;").unwrap();
    }

    fn dec(&self, f: &mut File) {
        f.write_all(b"ptr-=1;").unwrap();
    }

    fn add(&self, f: &mut File) {
        f.write_all(b"arr[ptr]+=1;").unwrap();
    }

    fn sub(&self, f: &mut File) {
        f.write_all(b"arr[ptr]-=1;").unwrap();
    }

    fn put(&self, f: &mut File) {
        f.write_all(b"fmt.Printf(\"%c\", arr[ptr]);").unwrap();
    }

    fn get(&self, f: &mut File) {
        f.write_all(b"arr[ptr] = getchar();").unwrap();
    }

    fn rep(&self, f: &mut File) {
        f.write_all(b"for arr[ptr] != 0 {").unwrap();
    }

    fn end(&self, f: &mut File) {
        f.write_all(b"};").unwrap();
    }

    fn extension(&self) -> String {
        "go".to_string()
    }

    fn compile(&self, file_path: PathBuf) -> PathBuf {
        println!("  [INFO] Compiling {:?}", file_path);

        let mut out_file = file_path.clone();
        out_file.set_extension("go.exe");

        let out = Command::new("go")
            .arg("build")
            .args(["-o", out_file.to_str().unwrap()])
            .arg(file_path)
            .output()
            .unwrap();

        if !out.stderr.is_empty() {
            eprintln!("{}", String::from_utf8(out.stderr).unwrap());
        }
        if !out.stdout.is_empty() {
            println!("{}", String::from_utf8(out.stdout).unwrap());
        }
        out_file
    }

    fn run(&self, file_path: PathBuf) {
        println!("  [INFO] Running {:?}", file_path);

        let out = Command::new(file_path.to_str().unwrap()).output().unwrap();

        if !out.stderr.is_empty() {
            eprint!("{}", String::from_utf8(out.stderr).unwrap());
        }
        if !out.stdout.is_empty() {
            print!("{}", String::from_utf8(out.stdout).unwrap());
        }
    }
}

fn usage() {
    eprintln!("Usage: bf <source> [options...]");
    eprintln!("Supported options:");
    eprintln!("  -r[un]    to run the program after compilation");
    eprintln!("  -c");
    eprintln!("  -j[ava]");
    eprintln!("  -py[thon]");
    eprintln!("  -js");
    eprintln!("  -rust");
    eprintln!("  -go");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    let mut make_lang: Vec<Lang> = Vec::new();
    let mut source_files: Vec<String> = Vec::new();

    let mut run_program = false;

    for arg in args {
        match arg.as_str() {
            "-r" | "-run" => run_program = true,
            "-i" | "-interpret" => make_lang.push(Lang::Interpret),
            "-c" => make_lang.push(Lang::C),
            "-j" | "-java" => make_lang.push(Lang::Java),
            "-py" | "-python" => make_lang.push(Lang::Python),
            "-js" => make_lang.push(Lang::JavaScript),
            "-rust" => make_lang.push(Lang::Rust),
            "-go" => make_lang.push(Lang::Go),
            _ => source_files.push(arg.clone()),
        }
    }

    if source_files.is_empty() || make_lang.is_empty() {
        usage();
        exit(1)
    }

    for target_lang in make_lang {
        println!("[INFO] Target set to {:?}", target_lang);

        let parser: Box<dyn Brainfuck> = match target_lang {
            Lang::Interpret => todo!(),
            Lang::C => Box::<Generator<C>>::default(),
            Lang::Java => todo!(),
            Lang::Python => todo!(),
            Lang::JavaScript => todo!(),
            Lang::Rust => todo!(),
            Lang::Go => Box::<Generator<Go>>::default(),
        };

        for file_path in &source_files {
            let mut file_path = PathBuf::from(&file_path);

            println!("  [INFO] Parsing {:?}", file_path);
            let source = fs::read_to_string(&file_path).unwrap();

            file_path.set_extension(parser.extension());
            let mut f = File::create(&file_path).unwrap();

            parser.prefix(&mut f);

            for c in source.chars() {
                match c {
                    '>' => parser.inc(&mut f),
                    '<' => parser.dec(&mut f),
                    '+' => parser.add(&mut f),
                    '-' => parser.sub(&mut f),
                    '.' => parser.put(&mut f),
                    ',' => parser.get(&mut f),
                    '[' => parser.rep(&mut f),
                    ']' => parser.end(&mut f),
                    _ => {}
                }
            }
            parser.suffix(&mut f);

            let file_path = parser.compile(file_path);

            if run_program {
                parser.run(file_path)
            }
        }
    }
}
