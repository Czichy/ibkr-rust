use tokio::sync::mpsc;
use tracing::{debug, instrument};

use super::{Client, Request};
use crate::{
    cmd::{PlaceOrder, RequestOrders},
    order::{Order, OrderTracker},
    OrderId, Result,
};
impl Client {
    pub fn subscribe_orders(&mut self) -> OrderTracker {
        self.order_tracker.clone()
    }

    #[instrument(skip(self))]
    pub async fn request_completed_orders(&mut self, api_only: bool) -> Result<()> {
        let frame = RequestOrders::Completed { api_only };

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn request_all_open_orders(&mut self) -> Result<()> {
        let frame = RequestOrders::AllOpen;

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn request_auto_open_orders(&mut self, auto_bind: bool) -> Result<()> {
        let frame = RequestOrders::AutoOpen { auto_bind };

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    /// Places or modifies an order
    ///
    /// # Arguments
    ///
    /// * 'id'  -   the order's unique identifier. Use a sequential id starting
    ///   with the id received at the nextValidId method. If a new order is
    ///   placed with an order ID less than or equal to the order ID of a
    ///   previous order an error will occur.
    /// * 'order'   the order
    pub async fn place_order(&mut self, order_id: OrderId, order: Order) -> Result<()> {
        let frame = PlaceOrder::new(order_id, order);

        debug!(request = ?frame);
        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    /// Requests the next valid order ID at the current moment.
    pub async fn request_ids(&mut self) -> Result<()> {
        let frame = RequestOrders::NextOrderId;

        debug!(request = ?frame);

        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        Ok(())
    }

    /// Requests the next valid order ID at the current moment.
    pub async fn get_next_valid_order_id(&mut self) -> Result<OrderId> {
        let frame = RequestOrders::NextOrderId;

        debug!(request = ?frame);

        let (rep_tx, mut rep_rx) = mpsc::channel(8);
        self.subscribe_handler_tx
            .send(Request::OrderId { sender: rep_tx })
            .await?;
        // Write the frame to the socket
        self.writer.write_frame(&frame.into_frame()).await?;
        let mut next_order_id: Option<OrderId> = None;
        while let Some(Some(detail)) = rep_rx.recv().await {
            tracing::trace!("Received order id: {:?}", detail);
            next_order_id = Some(detail);
        }
        next_order_id.ok_or_else(|| "Returned None".into())
    }
}
