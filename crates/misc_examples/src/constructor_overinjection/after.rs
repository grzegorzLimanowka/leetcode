use std::sync::{Arc, Mutex};

trait OrderRepository {
    fn save(&self, order: Order);
}

struct LocallySavedOrders {
    orders: Arc<Mutex<Vec<Order>>>,
}

impl OrderRepository for LocallySavedOrders {
    fn save(&self, order: Order) {
        let mut orders = self.orders.lock().unwrap();

        orders.push(order);
    }
}

trait MessageService {
    fn send_receipt(&self, order: Order);
}
trait BillingSystem {
    fn notify_accounting(&self, order: Order);
}
trait LocationService {
    fn find_warehouse(&self, order: Order);
}
trait InverntoryManagement {
    fn notify_warehouse(&self, order: Order);
}

struct OrderService<R, N>
where
    R: OrderRepository,
    N: NotificationService,
{
    order_repository: R,
    notification_service: N,
}

impl<R, N> OrderService<R, N>
where
    R: OrderRepository,
    N: NotificationService,
{
    fn approve_order(&self, order: Order) {
        self.update_order(order.clone());
        self.notification_service.order_approved(order);
        println!("[OrderService]: Sent all approvals");
    }

    fn update_order(&self, mut order: Order) {
        println!("[OrderService]: Updating order");

        order.approve();
        self.order_repository.save(order);
        println!("[OrderService]: Order Saved");
    }
}

#[derive(Clone, Debug)]
struct Order {
    id: String,
    desc: String,
}

impl Order {
    fn approve(&self) {}
}

trait OrderFulfillment {
    /// Fulfills the order
    fn fulfill(&self, order: Order);
}

struct OrderFulfiller<L, I> {
    location_service: L,
    inventory_management: I,
}

impl<L, I> OrderFulfillment for OrderFulfiller<L, I>
where
    L: LocationService,
    I: InverntoryManagement,
{
    fn fulfill(&self, order: Order) {
        self.location_service.find_warehouse(order.clone());
        self.inventory_management.notify_warehouse(order)
    }
}

trait NotificationService {
    fn order_approved(&self, order: Order); // like `broadcast`
}

struct CompositeNotificationService {
    services: Vec<Box<dyn NotificationService>>, // dyn dispatch
}

impl NotificationService for CompositeNotificationService {
    fn order_approved(&self, order: Order) {
        self.services.iter().for_each(|s| {
            s.order_approved(order.clone());
            println!("order {:?} approved", order);
        })
    }
}

struct Billing {}

impl BillingSystem for Billing {
    fn notify_accounting(&self, order: Order) {
        println!("Notified accounting ");
    }
}

impl NotificationService for Billing {
    fn order_approved(&self, order: Order) {
        println!("Order approved for billing ");
    }
}

struct Messaging {}

impl MessageService for Messaging {
    fn send_receipt(&self, order: Order) {
        println!("Sent receipt !")
    }
}

impl NotificationService for Messaging {
    fn order_approved(&self, order: Order) {
        println!("Order approved for messaging");
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::{
        Billing, BillingSystem, CompositeNotificationService, LocallySavedOrders, MessageService,
        Messaging, NotificationService, Order, OrderRepository, OrderService,
    };

    #[test]
    fn testing() {
        let order_repo = LocallySavedOrders {
            orders: Arc::new(Mutex::new(vec![])),
        };

        let billing = Billing {};
        let messaging = Messaging {};

        let notifications = CompositeNotificationService {
            services: vec![Box::new(billing), Box::new(messaging)],
        };

        let order_service = OrderService {
            order_repository: order_repo,
            notification_service: notifications,
        };

        order_service.approve_order(Order {
            id: "1".to_owned(),
            desc: "First order".to_owned(),
        });

        println!("\n\n");

        order_service.approve_order(Order {
            id: "2".to_owned(),
            desc: "second order".to_owned(),
        });
    }
}
