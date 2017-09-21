use trytes::*;
use inner::*;

use super::NonceView;
pub trait Transaction<'a> {
    fn signature_or_message(&self) -> &[Trit];
    fn address(&self) -> HashView<'a>;
    fn value(&self) -> isize;
    fn obsolete_tag(&self) -> TagView<'a>;
    fn timestamp(&self) -> usize;
    fn current_index(&self) -> usize;
    fn last_index(&self) -> usize;
    fn bundle(&self) -> HashView<'a>;
    fn trunk(&self) -> HashView<'a>;
    fn branch(&self) -> HashView<'a>;
    fn tag(&self) -> TagView<'a>;
    fn attachment_timestamp(&self) -> usize;
    fn attachment_timestamp_lower(&self) -> usize;
    fn attachment_timestamp_upper(&self) -> usize;
    fn nonce(&self) -> NonceView<'a>;
    fn essence(&self) -> &[Trit];
}

pub trait TransactionMut<'a> {
    fn set_signature_or_message(&mut self, t: &[Trit]);
    fn set_address(&mut self, h: &HashView);
    fn set_value(&mut self, v: isize);
    fn set_obsolete_tag(&mut self, t: &TagView);
    fn set_timestamp(&mut self, t: usize);
    fn set_current_index(&mut self, idx: usize);
    fn set_last_index(&mut self, idx: usize);
    fn set_bundle(&mut self, h: &HashView);
    fn set_trunk(&mut self, h: &HashView);
    fn set_branch(&mut self, h: &HashView);
    fn set_tag(&mut self, t: &TagView);
    fn set_attachment_timestamp(&mut self, timestamp: usize);
    fn set_attachment_timestamp_lower(&mut self, timestamp: usize);
    fn set_attachment_timestamp_upper(&mut self, timestamp: usize);
    fn set_nonce(&mut self, h: &NonceView);
}

#[derive(Debug, Eq, PartialEq)]
pub enum TransactionParseError {
    InvalidLength,
}
