use crossbeam::channel::Receiver;
use tokio::sync::mpsc;
use tracing::{debug, instrument};

use super::{Client, ContractDetailsResponse, Request};
use crate::{cmd::RequestContractDetails,
            contract::{Contract, ContractDetails},
            RequestId,
            Result};
impl Client {
    #[instrument(skip(self))]
    pub async fn request_contract_details(
        &mut self,
        req_id: RequestId,
        contract: Contract,
    ) -> Result<()> {
        // Convert the command into a frame
        let frame = RequestContractDetails::new(req_id, contract);

        debug!(request = ?frame);
        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    pub fn subscribe_contract_details(&mut self) -> Receiver<ContractDetailsResponse> {
        self.contract_events.clone()
    }

    #[instrument(skip(self))]
    pub async fn get_contract_details(
        &mut self,
        req_id: RequestId,
        contract: Contract,
    ) -> Result<Vec<ContractDetails>> {
        // Convert the command into a frame
        let frame = RequestContractDetails::new(req_id, contract);
        debug!("request id:\t{}", req_id);
        debug!(request = ?frame);
        // register sender for contract details callback
        let (rep_tx, mut rep_rx) = mpsc::channel(8);
        self.subscribe_handler_tx
            .send(Request::RequestWithId {
                req_id,
                sender: rep_tx,
            })
            .await?;
        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        let mut results = Vec::new();
        while let Some(result) = rep_rx.recv().await {
            let ContractDetailsResponse { req_id: _, details } = result;
            tracing::trace!("Received contract details: {:?}", details);
            match details {
                Some(details) => results.push(details),
                None => break,
            }
        }
        Ok(results)
    }
}
