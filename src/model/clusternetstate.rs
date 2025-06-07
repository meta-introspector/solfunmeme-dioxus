#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum ClusterNetState {
    Success,
    #[default]
    Waiting,
    Failure,
}
