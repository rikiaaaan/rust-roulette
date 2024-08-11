mod rikiaaan {
	#![allow(unused)]

	pub mod io {
		#![allow(unused)]
		use std::{fmt::{self, Arguments, Display}, io::{self, stdin, stdout, BufRead, Read, StdinLock, StdoutLock, Write}, str::FromStr};

		pub struct Io<R: Read, W: Write> {
			stdin: R,
			stdout: W,
		}

		impl<R: Read, W: Write> Io<R, W> {
			pub fn new(stdin: R, stdout: W) -> Self {
				Self { stdin, stdout }
			}

			pub fn read_line(&mut self) -> Option<String> {
				let t = self.stdin.by_ref().bytes().map(|c| c.unwrap() as char)
					.skip_while(|c| *c == '\n')
					.take_while(|c| *c != '\n')
					.collect::<String>();
				if !t.is_empty() {
					Some(t)
				} else {
					None
				}
			}

			pub fn read<T: FromStr>(&mut self) -> Option<T> {
				let t = self.stdin.by_ref().bytes().map(|c| c.unwrap() as char)
					.skip_while(|c| c.is_ascii_whitespace())
					.take_while(|c| !c.is_ascii_whitespace())
					.collect::<String>();

				if !t.is_empty() {
					t.parse::<T>().ok()
				} else {
					None
				}
			}

			pub fn vec<T: FromStr>(&mut self, len: usize) -> Vec<T> {
				(0..len).map(|_| self.read().unwrap()).collect()
			}

			pub fn mat<T: FromStr>(&mut self, row: usize, col: usize) -> Vec<Vec<T>> {
				(0..row).map(|_| self.vec(col)).collect()
			}

			pub fn write<T: Display + Into<String>>(&mut self, arg: T) -> Result<usize, std::io::Error> {
				self.stdout.write(arg.into().as_bytes())
			}

			pub fn writeln<T: Display>(&mut self, arg: T) -> Result<(), std::io::Error> {
				self.stdout.write_fmt(format_args!("{}\n", arg))
			}
		}

	}
}

use std::io::{stdin, stdout};
use rikiaaan::io::Io;

use rand::{thread_rng, Rng};

fn main() {
	let stdin = stdin();
	let stdout = stdout();
	let mut io = Io::new(stdin.lock(), stdout.lock());

	let mut vec: Vec<String> = Vec::new();

	io.writeln("★★★★★ ルーレット ★★★★★").unwrap();
	io.writeln("ランダムに選びたい項目を入力してってください").unwrap();
	io.writeln("入力し終わったら半角のアンダーバー(_)を入力してください").unwrap();

	loop {
		let str: String = io.read_line().unwrap();
		let str = str.as_str();

		match str.trim() {
			"_" if vec.len() == 0 => io.writeln("なんか打って").unwrap(),
			"_" if vec.len() > 1 => break,
			_ => vec.push(str.to_string()),
		}
	}

	let random_index: usize = thread_rng().gen_range(0..vec.len());

	io.writeln("★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★").unwrap();
	io.writeln("").unwrap();
	io.writeln(format!("結果： {}", vec[random_index])).unwrap();
	io.writeln("").unwrap();
	io.writeln("★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★★").unwrap();

}

