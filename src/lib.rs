use itertools::Itertools;
use std::fmt::{self, Debug, Formatter};
use thiserror::Error;

pub enum Vector {
	MoveTo { x: i32, y: i32 },
	LineTo { x: i32, y: i32 }
}

pub struct Glyph {
	pub vectors: Vec<Vector>,
	pub min_x: i32,
	pub min_y: i32,
	pub max_x: i32,
	pub max_y: i32
}

#[derive(Clone, Copy)]
pub struct Font<'a> {
	data: &'a [&'a str],
	offset: usize
}

/// The requested glyph with character code could not be found.
#[derive(Clone, Copy, Error)]
#[error("no such glyph {}", .0)]
pub struct NoSuchGlyph(char);

impl Debug for NoSuchGlyph {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.debug_tuple("NoSuchGlyph").field(&self.0).finish()
	}
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Pen {
	Idle,
	Up,
	Hovering,
	Down
}

impl<'a> Font<'a> {
	pub const fn new(data: &'a [&'a str], offset: char) -> Self {
		Self {
			data,
			offset: offset as usize
		}
	}

	pub fn glyph(&self, ch: char) -> Result<Glyph, NoSuchGlyph> {
		let idx = ch as usize - self.offset;
		let data = self.data.get(idx).ok_or(NoSuchGlyph(ch))?;

		let mut glyph = Glyph {
			vectors: Vec::new(),
			min_x: 0,
			min_y: 0,
			max_x: 0,
			max_y: 0
		};
		let mut pen = Pen::Idle;
		let mut x = 0;
		let mut y = 0;

		for (nx, ny) in data.chars().tuples() {
			if nx == ' ' && ny == 'R' {
				pen = Pen::Up;
				continue;
			}

			let nx = nx as i32 - 'R' as i32;
			let ny = ny as i32 - 'R' as i32;

			if pen == Pen::Hovering {
				glyph.vectors.push(Vector::MoveTo { x, y });
			}

			x = nx;
			y = ny;

			match pen {
				Pen::Idle => {
					pen = Pen::Up;
				},
				Pen::Up => {
					pen = Pen::Hovering;
				},
				Pen::Hovering | Pen::Down => {
					glyph.vectors.push(Vector::LineTo { x, y });
					pen = Pen::Down;
				}
			}

			glyph.min_x = glyph.min_x.min(x);
			glyph.min_y = glyph.min_y.min(y);
			glyph.max_x = glyph.max_x.max(x);
			glyph.max_y = glyph.max_y.max(y);
		}

		Ok(glyph)
	}
}
