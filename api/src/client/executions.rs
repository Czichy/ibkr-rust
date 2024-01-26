use tracing::debug;

use super::Client;
use crate::{cmd::RequestExecutions, order::ExecutionFilter, RequestId, Result};
impl Client {
    //#########################################################################
    //################## Executions
    //################## #######################################################

    /// When this function is called, the execution reports that meet the
    /// filter criteria are downloaded to the client via the execDetails()
    /// function. To view executions beyond the past 24 hours, open the
    /// Trade Log in TWS and, while the Trade Log is displayed, request
    /// the executions again from the API.
    ///
    /// # Arguments
    /// * req_id - The ID of the data request. Ensures that responses are
    ///   matched to requests if several requests are in process.
    /// * exec_filter - This object contains attributes that describe the filter
    ///   criteria used to determine which execution reports are returned.
    ///
    /// NOTE: Time format must be 'yyyymmdd-hh:mm:ss' Eg: '20030702-14:55'
    #[tracing::instrument(skip(self))]
    pub async fn request_executions(
        &mut self,
        req_id: RequestId,
        exec_filter: Option<ExecutionFilter>,
    ) -> Result<()> {
        // let req_id = self.get_next_req_id();
        let frame = RequestExecutions::new(req_id, exec_filter);

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }
}
