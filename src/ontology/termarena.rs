#![allow(clippy::slow_vector_initialization)]
use log::trace;

use crate::term::internal::HpoTermInternal;
use crate::HpoTermId;

use crate::MAX_HPO_ID_INTEGER as HPO_TERM_NUMBERS;

pub(crate) struct Arena {
    terms: Vec<HpoTermInternal>,
    ids: Vec<usize>,
}

impl Default for Arena {
    fn default() -> Self {
        let mut ids = Vec::with_capacity(HPO_TERM_NUMBERS);
        ids.resize(HPO_TERM_NUMBERS, 0);
        let mut s = Self {
            terms: Vec::with_capacity(18_000),
            ids,
        };
        s.terms.push(HpoTermInternal::default());
        s
    }
}

impl Arena {
    pub fn len(&self) -> usize {
        self.terms.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn insert(&mut self, id: HpoTermId, term: HpoTermInternal) {
        let id = id.to_usize();
        if self.ids[id] == 0 {
            let idx = self.terms.len();
            self.terms.push(term);
            self.ids[id] = idx
        }
    }

    pub fn get(&self, id: &HpoTermId) -> Option<&HpoTermInternal> {
        match self.ids[id.to_usize()] {
            0 => {
                trace!("Term does not exist in Arena: {}", id);
                None
            }
            n => Some(&self.terms[n]),
        }
    }

    pub fn get_unchecked(&self, id: &HpoTermId) -> &HpoTermInternal {
        &self.terms[self.ids[id.to_usize()]]
    }

    pub fn get_unchecked_mut(&mut self, id: &HpoTermId) -> &mut HpoTermInternal {
        &mut self.terms[self.ids[id.to_usize()]]
    }

    pub fn get_mut(&mut self, id: &HpoTermId) -> Option<&mut HpoTermInternal> {
        match self.ids[id.to_usize()] {
            0 => None,
            n => Some(&mut self.terms[n]),
        }
    }

    pub fn shrink_to_fit(&mut self) {}

    pub fn values(&self) -> &[HpoTermInternal] {
        &self.terms[1..]
    }

    pub fn values_mut(&mut self) -> &mut [HpoTermInternal] {
        &mut self.terms[1..]
    }

    /// Returns the keys of this [`Arena`].
    pub fn keys(&mut self) -> Vec<HpoTermId> {
        self.terms[1..].iter().map(|term| *term.id()).collect()
    }
}
