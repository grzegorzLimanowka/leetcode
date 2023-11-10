trait OrderRepository {
    fn save(&self, order: Order);
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

struct OrderService<R, S, B, L, I>
where
    R: OrderRepository,
    S: MessageService,
    B: BillingSystem,
    L: LocationService,
    I: InverntoryManagement,
{
    order_repository: R,
    message_service: S,
    billing_system: B,
    location_service: L,
    inventory_management: I,
}

impl<R, S, B, L, I> OrderService<R, S, B, L, I>
where
    R: OrderRepository,
    S: MessageService,
    B: BillingSystem,
    L: LocationService,
    I: InverntoryManagement,
{
    fn approve_order(&self, order: Order) {
        self.update_order(order.clone());
        self.notify(order);
    }

    fn update_order(&self, mut order: Order) {
        order.approve();
        self.order_repository.save(order)
    }

    fn notify(&self, order: Order) {
        self.message_service.send_receipt(order.clone());
        self.billing_system.notify_accounting(order.clone());
        self.fulfill(order)
    }

    fn fulfill(&self, order: Order) {
        self.location_service.find_warehouse(order.clone());
        self.inventory_management.notify_warehouse(order);
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
