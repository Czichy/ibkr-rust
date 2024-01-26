use crossbeam::channel::Receiver;
use tracing::debug;

use super::Client;
use crate::{
    account::AccountData,
    cmd::{RequestAccountSummary, RequestAccountUpdates},
    prelude::AccountLastUpdate,
    AccountCode, Result,
};
impl Client {
    // pub fn subscribe_account_updates(self) -> AccountReceiver {
    // self.account_tracker }
    pub fn subscribe_account_updates(&mut self) -> Receiver<AccountData> {
        self.account_tracker.clone()
    }

    pub fn subscribe_account_last_updates(&mut self) -> Receiver<AccountLastUpdate> {
        self.account_update_tracker.clone()
    }

    /// Call this function to start getting account values, portfolio,
    /// and last update time information via Wrapper.update_account_value());
    /// Wrapper.update_portfolio() and Wrapper.update_account_time().
    ///
    ///
    /// # Arguments
    /// * subscribe - If set to TRUE, the client will start receiving account
    ///   and Portfoliolio updates. If set to FALSE, the client will stop
    ///   receiving this information.
    /// * acct_code - The account code for which to receive account and
    ///   portfolio updates.
    #[tracing::instrument(skip(self))]
    pub async fn request_account_updates(
        &mut self,
        subscribe: bool,
        account_code: AccountCode,
    ) -> Result<()> {
        let frame = RequestAccountUpdates::new(subscribe, account_code);

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    /// Requests a specific account's summary.
    /// This method will subscribe to the account summary as presented in the
    /// TWS' Account Summary tab. The data is returned at
    /// Wrapper::account_summary https://www.interactivebrokers.com/en/software/tws/accountwindowtop.htm.
    /// Note:   This request is designed for an FA managed account but can be
    ///         used for any multi-account structure.
    ///
    /// # Arguments
    /// * req_id - The ID of the data request. Ensures that responses are
    ///   matched to requests If several requests are in process.
    /// * group_name - Set to All to return account summary data for all
    ///   accounts, or set to a specific Advisor Account Group name that has
    ///   already been created in TWS Global Configuration.
    /// * tags- A comma-separated list of account tags.  See the
    ///   AccountSummaryTags enum for valid values
    #[tracing::instrument(skip(self))]
    pub async fn request_account_summary(
        &mut self,
        group_name: String,
        tags: Vec<String>,
    ) -> Result<()> {
        let req_id = self.get_next_req_id();
        let frame = RequestAccountSummary::new(req_id, group_name, tags);

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }
    //#[tracing::instrument(skip(self))]
    // pub async fn get_account_summary(
    //    &mut self,
    //    group_name:String,
    //    tags: Vec<String>,
    //) -> Result<Vec<ContractDetails>> {
    //    // Convert the command into a frame
    //    let req_id = self.get_next_req_id();
    //    let frame = RequestContractDetails::new(req_id, contract);
    //    debug!("request id:\t{}", req_id);
    //    debug!(request = ?frame);
    //    // register sender for contract details callback
    //    let (rep_tx, mut rep_rx) = mpsc::channel(8);
    //    let _subscribe = self
    //        .subscribe_handler_tx
    //        .send(Request::RequestWithId {
    //            req_id,
    //            sender: rep_tx,
    //        })
    //        .await?;
    //    // Write the frame to the socket
    //    self.writer.write_frame(&frame.into_frame()).await?;
    //    let mut results = Vec::new();
    //    while let Some(result) = rep_rx.recv().await {
    //        match result {
    //            Response::ContractDetails { req_id: _, details } => {
    //                tracing::trace!("Received contract details: {:?}", details);
    //                match details {
    //                    Some(details) => results.push(details),
    //                    None => break,
    //                }
    //            },
    //            //_ => (),
    //        }
    //    }
    //    Ok(results)
    //}
}
