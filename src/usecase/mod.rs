pub(crate) mod git_bump;
mod git_bump_test;

trait UseCase<Req, Resp, Error> {
    fn execute(&self, params: Req) -> Result<Resp, Error>;
}
