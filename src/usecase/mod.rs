pub(crate) mod release;

trait UseCase<Req, Resp, Error> {
    fn execute(&self, params: Req) -> Result<Resp, Error>;
}
