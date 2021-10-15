// HOWWIDE_UTF8: A simple crate that takes the first byte of a character,
// and uses said byte to determine the width of the character in question.

// How to use this crate:
// 1. Add this crate to your Cargo.toml file.
// 2. Add the line "extern crate howwide_utf8;" to the file you are wanting to
//    use this crate in.
// 3. Invoke the main function of this crate.

// Example:
// assert_eq!(1, howwide_utf8::getw(b'1'));

// If you want to benchmark this crates' performance, simply run the "cargo
// bench" command.

#![no_std]

pub const MINIM_0_1: u8 = 0x80;
pub const MAXIM_0_1: u8 = 0xC1;
pub const MINIM_0_2: u8 = 0xF5;
pub const MAXIM_0_2: u8 = 0xFF;
pub const MINIM_1: u8 = 0x00;
pub const MAXIM_1: u8 = 0x7F;
pub const MINIM_2: u8 = 0xC2;
pub const MAXIM_2: u8 = 0xDF;
pub const MINIM_3: u8 = 0xE0;
pub const MAXIM_3: u8 = 0xEF;
pub const MINIM_4: u8 = 0xF0;
pub const MAXIM_4: u8 = 0xF4;

#[inline]
pub fn isw_1(byte: u8) -> bool
{
	byte <= MAXIM_1
}
#[inline]
pub fn isw_2(byte: u8) -> bool
{
	MINIM_2 <= byte && byte <= MAXIM_2
}
#[inline]
pub fn isw_3(byte: u8) -> bool
{
	MINIM_3 <= byte && byte <= MAXIM_3
}
#[inline]
pub fn isw_4(byte: u8) -> bool
{
	MINIM_4 <= byte && byte <= MAXIM_4
}
#[inline]
pub fn isw_0(byte: u8) -> bool
{
	MINIM_0_1 <= byte && byte <= MAXIM_0_1 || MINIM_0_2 <= byte
}
#[inline]
pub fn getw(byte: u8) -> usize
{
	if isw_1(byte)
	{
		1
	}
	else if isw_2(byte)
	{
		2
	}
	else if byte <= MAXIM_3
	{
		3
	}
	else if byte <= MAXIM_4
	{
		4
	}
	else
	{
		0
	}
}

#[allow(clippy::missing_safety_doc)]
#[inline]
pub unsafe fn getw_assumevalid(byte: u8) -> usize
{
	if byte <= MAXIM_1
	{
		1
	}
	else if byte <= MAXIM_2
	{
		2
	}
	else if byte <= MAXIM_3
	{
		3
	}
	else
	{
		4
	}
}
