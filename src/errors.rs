use warp::reject::Reject;

#[derive(Debug)]
pub struct IncorrectParameters;
impl Reject for IncorrectParameters {}

#[derive(Debug)]
pub struct Failed(pub eyre::Report);
impl Reject for Failed {}
