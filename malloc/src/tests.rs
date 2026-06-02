//some tests

//These were all originally in lib.rs
// i moved them here when i was done so they weren't clogging up the everything
// also you have to run the tests sequentially or they mess each other up
// thats cargo test -- --test-threads=1  
// also cargo test -- --nocapture was helpful


#[test]	
fn debug_reset() {
	
	println!("\nThis is the bitmask before initialization or reset.");

	//first print of bitmask. it shouldnt have been initialized yet.
	
	debug_print_bitmask();
	
	reset();

	// the bus  has now been reset. 
	println!("\nThe bus has now been reset.");
	
	debug_print_bitmask();

	
	println!("\nWe will now run malloc 4 to save 4 bytes.");

	malloc(4);
	debug_print_bitmask();

	println!("\nWe will now run reset again.");

	reset();
	debug_print_bitmask();
}


#[test]
fn test_no_overlap() {
	reset();
	let a = malloc(12);
	let b = malloc(4);
	let c = malloc(5);

	assert_ne!(a,b);
	assert_ne!(b,c);
	assert_ne!(a,c);
}



#[test]
fn test_no_overlap_2() {
    reset();

    debug_print_bitmask();

    let a = malloc(16).unwrap();
    let b = malloc(16).unwrap();

    unsafe {
	let base = (&raw mut BUS as *mut [u8; SIZE]) as *mut u8;

        let a_ptr = base.add(a);
        let b_ptr = base.add(b);

        // write pattern into a
        for i in 0..16 {
            *a_ptr.add(i) = 0xAA;
        }

        // write pattern into b
        for i in 0..16 {
            *b_ptr.add(i) = 0x55;
        }

        // verify a is intact
        for i in 0..16 {
            assert_eq!(*a_ptr.add(i), 0xAA);
        }

        // verify b is intact
        for i in 0..16 {
            assert_eq!(*b_ptr.add(i), 0x55);
        }
    }

}

#[test]
fn test_no_overlap_3(){
	reset();
	println!("\nBitmask after reset");
	debug_print_bitmask();

	let mut ptrs = vec![];

	for x in 0..20 {
		let p = malloc(3).unwrap();
		ptrs.push(p);
                println!("\nBitmask after {x}th malloc(3)");
		debug_print_bitmask();
	}

	//check unique
	for i in 0..ptrs.len() {
		for j in i + 1..ptrs.len() {
			assert_ne!(ptrs[i], ptrs[j]);
		}
	}
}


#[test]
fn test_exact_fit() {
	reset();
	let mut ptrs = vec![];

	for x in 0..(1024 / 16) {
		let p = malloc(16);
		if p.is_none() {break; }
		ptrs.push(p.unwrap());
	}
	assert!(malloc(16).is_none());
}

#[test]
fn test_round_trip() {
	reset();
	let p = malloc(16).unwrap();

	let x: i32 = 0x44332211;
	setter(x, p);

	let y: i32 = getter(p);
	assert_eq!(x, y);
}

#[test]
fn test_multiple_types() {
	reset();
	let p1 = malloc(6).unwrap();
	let p2 = malloc(8).unwrap();

	let a: u32 = 0x12345678;
	let b: u64 = 0x1122334455667788;
	
	setter(a, p1);
	setter(b, p2);

	assert_eq!(getter::<u32>(p1), a);
	assert_eq!(getter::<u64>(p2), b);
}

#[test]
fn test_stuff() {
	reset();
	let p1 = malloc(8).unwrap();
	let p2 = malloc(8).unwrap();

	let a: u64 = 0xFFFFFFFFFFFFFFFF;
	let b: u64 = 0x0000000000000000;

	setter(a, p1);
	setter(b, p2);

	assert_eq!(getter::<u64>(p1), a);
	assert_eq!(getter::<u64>(p2), b);
}


fn debug_print_bitmask() {
    unsafe {
        println!("--- BITMASK DUMP ---");

        for i in 0..BITMASK_SIZE * 8 {
            let bit = get_bit(i);
            if i % 8 == 0 {
                print!("\nbyte {:3}: ", i / 8);
            }

            print!("{}", if bit { "1" } else { "0" });
        }

        println!("\n---------");
    }
}
