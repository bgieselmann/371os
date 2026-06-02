pub const SIZE: usize = 0x80;
static mut BUS: [u8; SIZE] = [0u8; SIZE];
const BITMASK_SIZE: usize = SIZE >> 3;

fn reset() {
    unsafe {
        BUS = [0u8; SIZE];
    }
    init();
}


fn get_bit(i: usize) -> bool {
	unsafe {

		let byte_index = i / 8;
		let bit_index = i % 8;

		let mask = 1u8 << bit_index;

		return (BUS[byte_index] & mask) != 0
	}
}


// sets bit to ONE (1)
fn set_bit(i: usize) {
	unsafe {
		let byte_index = i / 8;
		let bit_index = i % 8;

		let mask = 1u8 << bit_index;

		BUS[byte_index] = BUS[byte_index] | mask;
	}
}


fn init() {
	for i in 0..BITMASK_SIZE {
		set_bit(i);
	}
}


// I originally has find_contiguous_free written in the way i like better
// the way that isn't unoptimal and loser core
// but before i realized tests were done in parallel a bunch of stuff wasn't working
// so i rewrote it like 10 million times in trying to get it to work
// eventually with this more intuitive but also grosser and less efficient version
// that calls each byte times
// instead of once and iterating over the bits per byte
// but i already rewrote it this way and I don't wanna put it back

fn find_contiguous_free(s: usize) -> Option<usize> { 
	let mut count = 0;
	let mut start = 0;

	for i in 0..SIZE {
		if get_bit(i) == false {
			if count == 0 {
				start = i;
			}
			count += 1;
			if count == s {
				return Some(start);
			}
		}
		if get_bit(i) != false {
			count = 0; 
		}
	}
	return None;
}


// Place val at loc
use std::mem::{size_of_val, size_of, MaybeUninit};

pub fn setter<T>(val: T, loc: usize) {
	unsafe {
		let size = size_of_val(&val);

		let src = &val as *const T as *const u8;
		let dst = (&raw mut BUS as *mut u8).add(loc);

		for i in 0..size {
			*dst.add(i) = *src.add(i);
		}
	}
}

pub fn getter<T>(loc: usize) -> T {
	unsafe {
		let size = size_of::<T>();
		let src = (&raw const BUS as *const u8).add(loc);

		let mut out = MaybeUninit::<T>::uninit();
		let dst = out.as_mut_ptr() as *mut u8;

		for i in 0..size {
			*dst.add(i) = *src.add(i);
		}
	
		return out.assume_init();
	}
}


// Return an index in BUS of s reserved bytes
pub fn malloc(s: usize) -> Option<usize> {
	init();

	// find s free bytes
	let reserve_index = match find_contiguous_free(s) {

		Some(i) => i,
		None => return None,
	};	

	//block those bytes off in the bitmask
	for x in 0..s {
		set_bit(reserve_index + x);
	}

	return Some(reserve_index);
}



