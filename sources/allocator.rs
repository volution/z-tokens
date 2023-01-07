

#![ allow (dead_code) ]


use ::std::alloc;


use ::std::prelude::v1::*;


use ::std::sync::atomic::{
		AtomicUsize,
		AtomicU64,
		Ordering,
	};




#[ cfg (feature = "allocator") ]
#[ global_allocator ]
pub(crate) static GLOBAL : Allocator = Allocator::new ();




pub(crate) const DEBUG_MAIN : bool = true;
const DEBUG_ALLOC : bool = false;




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
			let _max_current = _counters.max_current.load (Ordering::SeqCst);
			if _now_current <= _max_current {
				break;
			}
			if _counters.max_current.compare_exchange (_max_current, _now_current, Ordering::SeqCst, Ordering::SeqCst) .is_ok () {
				break;
			}
		}
		
		let _pointer = alloc::System.alloc (_layout);
		
		if DEBUG_ALLOC {
			::std::eprintln! ("[dd] [e23d67ac]  alloc:    {:016x}  |  {:6} = {:6} : {:2}", _pointer as usize, _amount, _size, _align);
		}
		
		_pointer
	}
	
	unsafe fn dealloc (&self, _pointer : *mut u8, _layout: alloc::Layout) {
		
		let _counters = &self.counters;
		
		let (_amount, _size, _align) = Allocator::layout (&_layout);
		
		_counters.count_dealloc.fetch_add (1, Ordering::SeqCst);
		_counters.amount_current.fetch_sub (_amount, Ordering::SeqCst);
		
		if DEBUG_ALLOC {
			::std::eprintln! ("[dd] [18d32fb1]  dealloc:  {:016x}  |  {:6} = {:6} : {:2}", _pointer as usize, _amount, _size, _align);
		}
		
		alloc::System.dealloc (_pointer, _layout)
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
			::std::eprintln! ("[ii] [b6c41147]  allocations:  {} K allocs / {} K deallocs / {} KiB total / {} KiB current / {} KiB max",
					_count_alloc / 1000, _count_dealloc / 1000,
					_amount_total / 1024, _amount_current / 1024,
					_max_current / 1024,
				);
		} else {
			::std::eprintln! ("[ii] [6178b820]  allocations:  {} allocs / {} deallocs / {} B total / {} B current / {} B max",
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




pub(crate) fn report () -> () {
	#[ cfg (feature = "allocator") ]
	GLOBAL.report ();
}


pub(crate) fn reset () -> () {
	#[ cfg (feature = "allocator") ]
	GLOBAL.reset ();
}


pub(crate) fn is_empty () -> Option<bool> {
	#[ cfg (feature = "allocator") ]
	return Some (GLOBAL.is_empty ());
	#[ cfg (not (feature = "allocator")) ]
	return None;
}

