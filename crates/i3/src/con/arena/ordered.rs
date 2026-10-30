//! Pure ordered identities; liveness, ownership and callbacks belong elsewhere.
use super::ConId;

#[derive(Default)]
pub(super) struct OrderedIds(Vec<ConId>);
impl OrderedIds {
    pub(super) fn as_slice(&self) -> &[ConId] {
        &self.0
    }
    pub(super) fn first(&self) -> Option<ConId> {
        self.0.first().copied()
    }
    pub(super) fn last(&self) -> Option<ConId> {
        self.0.last().copied()
    }
    pub(super) fn len(&self) -> usize {
        self.0.len()
    }
    pub(super) fn position(&self, id: ConId) -> Option<usize> {
        self.0.iter().position(|&item| item == id)
    }
    pub(super) fn at(&self, index: usize) -> Option<ConId> {
        self.0.get(index).copied()
    }
    pub(super) fn insert(&mut self, index: usize, id: ConId) {
        self.0.insert(index, id);
    }
    pub(super) fn remove(&mut self, index: usize) -> ConId {
        self.0.remove(index)
    }
    pub(super) fn replace(&mut self, index: usize, id: ConId) {
        self.0[index] = id;
    }
    pub(super) fn swap(&mut self, first: usize, second: usize) {
        self.0.swap(first, second);
    }
    pub(super) fn move_to(&mut self, from: usize, mut insertion: usize) {
        let id = self.0.remove(from);
        if from < insertion {
            insertion -= 1;
        }
        self.0.insert(insertion, id);
    }
    pub(super) fn is_permutation(&self, order: &[ConId]) -> bool {
        self.0.len() == order.len()
            && order
                .iter()
                .enumerate()
                .all(|(index, id)| self.position(*id).is_some() && !order[..index].contains(id))
    }
    pub(super) fn replace_order(&mut self, order: &[ConId]) {
        self.0 = order.to_vec();
    }
}
