pub(crate) mod git_bump;

#[cfg(test)]
mod git_bump_test;

trait UseCase<Req, Resp, Error> {
    fn execute(&self, params: Req) -> Result<Resp, Error>;
}
