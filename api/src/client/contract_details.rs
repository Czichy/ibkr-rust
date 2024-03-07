use crossbeam::channel::Receiver;
use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tracing::{debug, instrument};

use super::{Client, Request, ResponseWithId};
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

    pub fn subscribe_contract_details(&mut self) -> Receiver<ResponseWithId<ContractDetails>> {
        self.contract_events.clone()
    }

    pub async fn get_contract_details(
        &mut self,
        req_id: RequestId,
        contract: Contract,
    ) -> impl Stream<Item = Result<ContractDetails>> + '_ {
        async_stream::try_stream! {
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
        while let Some(response) = rep_rx.recv().await {
            if let Some(response) = response.response {
                tracing::trace!("Received contract details: {:?}", response);
                yield (response);
            } else {
                return;
            }
        }
        }
    }
}
