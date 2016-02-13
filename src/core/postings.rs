use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::prelude::Read;
use core::global::DocId;
use std::cmp::Ordering;
use std::vec;
use std::ptr;


////////////////////////////////////



pub trait Postings: Iterator<Item=DocId> {
    // after skipping position
    // the iterator in such a way that the
    // next call to next() will return a
    // value greater or equal to target.
    fn skip_next(&mut self, target: DocId) -> Option<DocId>;
}

// impl<T: Iterator<Item=DocId>> Postings for T {}


#[derive(Debug)]
pub struct VecPostings {
    doc_ids: Vec<DocId>,
	cursor: usize,
}

impl VecPostings {
    pub fn new(vals: Vec<DocId>) -> VecPostings {
        VecPostings {
            doc_ids: vals,
			cursor: 0,
        }
    }
}

impl Postings for VecPostings {
    // after skipping position
    // the iterator in such a way that the
    // next call to next() will return a
    // value greater or equal to target.
    fn skip_next(&mut self, target: DocId) -> Option<DocId> {
        loop {
            match Iterator::next(self) {
                Some(val) if val >= target => {
                    return Some(val);
                },
                None => {
                    return None;
                },
                _ => {}
            }
        }
    }
}

impl Iterator for VecPostings {
	type Item = DocId;
	fn next(&mut self,) -> Option<DocId> {
		if self.cursor >= self.doc_ids.len() {
			None
		}
		else {
            self.cursor += 1;
			Some(self.doc_ids[self.cursor - 1])
		}
	}
}





pub struct IntersectionPostings<T: Postings> {
    postings: Vec<T>,
}

impl<T: Postings> IntersectionPostings<T> {
    pub fn from_postings(mut postings: Vec<T>) -> IntersectionPostings<T> {
        IntersectionPostings {
            postings: postings,
        }
    }

}

impl<T: Postings> Iterator for IntersectionPostings<T> {
    type Item = DocId;
    fn next(&mut self,) -> Option<DocId> {
        let mut candidate;
        match self.postings[0].next() {
            Some(val) => {
                candidate = val;
            },
            None => {
                return None;
            }
        }
        'outer: loop {
            for i in 1..self.postings.len() {
                let skip_result = self.postings[i].skip_next(candidate);
                match skip_result {
                    None => {
                        return None;
                    },
                    Some(x) if x == candidate => {
                    },
                    Some(greater) => {
                        unsafe {
                            let pa: *mut T = &mut self.postings[i];
                            let pb: *mut T = &mut self.postings[0];
                            ptr::swap(pa, pb);
                        }
                        candidate = greater;
                        continue 'outer;
                    },
                }
            }
            return Some(candidate);
        }

    }
}