

#![ allow (dead_code) ]
#![ allow (unused_imports) ]




use ::std::alloc;
use ::std::ptr;
use ::std::slice;


use ::std::prelude::v1::*;


use ::std::sync::atomic::{
		AtomicBool,
		AtomicUsize,
		AtomicU64,
		Ordering,
	};


use ::std::{
		eprintln,
		debug_assert,
	};




use ::memsec;




#[ global_allocator ]
pub(crate) static GLOBAL : Allocator = Allocator::new ();




pub const DEBUG_REPORT : bool = false;

pub(crate) const DEBUG_ALLOC : bool = false;

pub(crate) const USE_MALLOC : bool = false;
pub(crate) const USE_MEMZERO : bool = true;
pub(crate) const USE_MLOCK : bool = false;




pub(crate) struct Allocator {
	counters : Counters,
}


pub(crate) struct Counters {
	
	count_alloc : AtomicU64,
	count_dealloc : AtomicU64,
	
	amount_total : AtomicUsize,
	amount_current : AtomicUsize,
	max_current : AtomicUsize,
}




unsafe impl alloc::GlobalAlloc for Allocator {
	
	unsafe fn alloc (&self, _layout: alloc::Layout) -> *mut u8 {
		
		let _counters = &self.counters;
		
		let (_amount, _size, _align) = Allocator::layout (&_layout);
		
		_counters.count_alloc.fetch_add (1, Ordering::SeqCst);
		_counters.amount_total.fetch_add (_amount, Ordering::SeqCst);
		let _previous_current = _counters.amount_current.fetch_add (_amount, Ordering::SeqCst);
		
		loop {
			let _now_current = _previous_current + _amount;
			let _max_current = _counters.max_current.load (Ordering::Relaxed);
			if _now_current <= _max_current {
				break;
			}
			if _counters.max_current.compare_exchange (_max_current, _now_current, Ordering::SeqCst, Ordering::SeqCst) .is_ok () {
				break;
			}
		}
		
		let _pointer = if USE_MALLOC {
				if let Some (mut _memory) = memsec::malloc_sized (_amount) {
					_memory.as_mut () .as_mut_ptr ()
				} else {
					::vrl_errors::panic! (unreachable, 0xa4ef09db);
				}
			} else {
				let _layout = unsafe { alloc::Layout::from_size_align_unchecked (_amount, _align) };
				alloc::System.alloc (_layout)
			};
		
		if USE_MLOCK {
			static _PRINTED : AtomicBool = AtomicBool::new (false);
			static _LOCKED : AtomicUsize = AtomicUsize::new (0);
			if ! memsec::mlock (_pointer, _amount) {
				let _locked = _LOCKED.fetch_add (_amount, Ordering::SeqCst);
				if ! _PRINTED.load (Ordering::Relaxed) && _PRINTED.compare_exchange (false, true, Ordering::SeqCst, Ordering::SeqCst) .is_ok () {
					eprintln! ("[!!] [cc90c20b]  mlock failed after {} + {} bytes;  ignoring!", _locked, _amount);
				}
			}
		}
		
		if USE_MEMZERO {
			memsec::memzero (_pointer, _amount);
		}
		
		if DEBUG_ALLOC {
			eprintln! ("[dd] [e23d67ac]  alloc:    {:016x}  |  {:6} = {:6} : {:2}", _pointer as usize, _amount, _size, _align);
		}
		
		_pointer
	}
	
	unsafe fn dealloc (&self, _pointer : *mut u8, _layout: alloc::Layout) {
		
		let _counters = &self.counters;
		
		let (_amount, _size, _align) = Allocator::layout (&_layout);
		
		_counters.count_dealloc.fetch_add (1, Ordering::SeqCst);
		_counters.amount_current.fetch_sub (_amount, Ordering::SeqCst);
		
		if DEBUG_ALLOC {
			eprintln! ("[dd] [18d32fb1]  dealloc:  {:016x}  |  {:6} = {:6} : {:2}", _pointer as usize, _amount, _size, _align);
		}
		
		if USE_MEMZERO {
			memsec::memzero (_pointer, _amount);
		}
		
		if USE_MALLOC {
			let _memory = unsafe { ptr::NonNull::new_unchecked (slice::from_raw_parts_mut (_pointer, _amount)) };
			memsec::free (_memory)
		} else {
			let _layout = unsafe { alloc::Layout::from_size_align_unchecked (_amount, _align) };
			alloc::System.dealloc (_pointer, _layout)
		}
	}
}




impl Allocator {
	
	
	pub(crate) const fn new () -> Self {
		
		let _counters = Counters {
				
				count_alloc : AtomicU64::new (0),
				count_dealloc : AtomicU64::new (0),
				
				amount_total : AtomicUsize::new (0),
				amount_current : AtomicUsize::new (0),
				max_current : AtomicUsize::new (0),
			};
		
		let _allocator = Allocator {
				
				counters : _counters,
			};
		
		_allocator
	}
	
	
	pub(crate) fn report (&self) -> () {
		
		let _counters = &self.counters;
		
		let _count_alloc = _counters.count_alloc.load (Ordering::SeqCst);
		let _count_dealloc = _counters.count_dealloc.load (Ordering::SeqCst);
		let _amount_total = _counters.amount_total.load (Ordering::SeqCst);
		let _amount_current = _counters.amount_current.load (Ordering::SeqCst);
		let _max_current = _counters.max_current.load (Ordering::SeqCst);
		
		if _count_alloc > 100_000 {
			eprintln! ("[ii] [b6c41147]  allocations:  {} K allocs / {} K deallocs / {} KiB total / {} KiB current / {} KiB max",
					_count_alloc / 1000, _count_dealloc / 1000,
					_amount_total / 1024, _amount_current / 1024,
					_max_current / 1024,
				);
		} else {
			eprintln! ("[ii] [6178b820]  allocations:  {} allocs / {} deallocs / {} B total / {} B current / {} B max",
					_count_alloc, _count_dealloc,
					_amount_total, _amount_current,
					_max_current,
				);
		}
	}
	
	
	pub(crate) fn reset (&self) -> () {
		
		let _counters = &self.counters;
		
		_counters.count_alloc.store (0, Ordering::SeqCst);
		_counters.count_dealloc.store (0, Ordering::SeqCst);
		_counters.amount_total.store (0, Ordering::SeqCst);
		_counters.amount_current.store (0, Ordering::SeqCst);
		_counters.max_current.store (0, Ordering::SeqCst);
	}
	
	
	pub(crate) fn is_empty (&self) -> bool {
		
		let _counters = &self.counters;
		
		let _count_alloc = _counters.count_alloc.load (Ordering::SeqCst);
		let _count_dealloc = _counters.count_dealloc.load (Ordering::SeqCst);
		let _amount_current = _counters.amount_current.load (Ordering::SeqCst);
		
		(_amount_current == 0) && (_count_alloc == _count_dealloc)
	}
	
	
	fn layout (_layout : &alloc::Layout) -> (usize, usize, usize) {
		
		let _size = _layout.size ();
		let _align = _layout.align ();
		
		let _amount = if (_size % _align) == 0 {
				_size
			} else {
				((_size / _align) + 1) * _align
			};
		
		(_amount, _size, _align)
	}
}




pub fn report () -> () {
	GLOBAL.report ();
}


pub fn reset () -> () {
	GLOBAL.reset ();
}


pub fn is_empty () -> Option<bool> {
	return Some (GLOBAL.is_empty ());
}

