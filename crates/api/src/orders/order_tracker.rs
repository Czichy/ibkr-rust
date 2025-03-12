use flume::{unbounded, Receiver, Sender};

use crate::{orders::{commission_and_fees_report::CommissionAndFeesReport,
                     execution::Execution,
                     order_state::{OrderState, OrderStatusUpdate},
                     Order},
            OrderId};

#[derive(Debug, Clone)]
pub struct OrderTracker {
    pub order:              Receiver<Order>,
    pub order_id:           Receiver<OrderId>,
    pub error:              Option<(i32, String)>,
    pub order_state:        Receiver<OrderState>,
    pub order_status:       Receiver<OrderStatusUpdate>,
    pub executions:         Receiver<Execution>,
    pub commission_reports: Receiver<CommissionAndFeesReport>,
}
pub(crate) struct OrderTrackerSender {
    pub executions_tx:         Sender<Execution>,
    pub order_tx:              Sender<Order>,
    pub order_id_tx:           Sender<OrderId>,
    pub order_status_tx:       Sender<OrderStatusUpdate>,
    pub order_state_tx:        Sender<OrderState>,
    pub commission_reports_tx: Sender<CommissionAndFeesReport>,
}
impl OrderTracker {
    pub(crate) fn new() -> (OrderTrackerSender, Self) {
        let (executions_tx, executions) = unbounded();
        let (commission_reports_tx, commission_reports) = unbounded();
        let (order_status_tx, order_status) = unbounded();
        let (order_state_tx, order_state) = unbounded();
        let (order_tx, order) = unbounded();
        let (order_id_tx, order_id) = unbounded();
        (
            OrderTrackerSender {
                order_tx,
                order_id_tx,
                executions_tx,
                commission_reports_tx,
                order_status_tx,
                order_state_tx,
            },
            OrderTracker {
                order,
                order_id,
                error: None,
                executions,
                commission_reports,
                order_status,
                order_state,
            },
        )
    }
}
