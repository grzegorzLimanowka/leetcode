#![allow(unused)]

// https://velog.io/@migorithm/Fluent-Rust-Reflection

// Rust has powerfull, rich type system, that guarantees memory-safety and thread-safety
// Everything comes with a cost, however

macro_rules! is_trait {
    ($name: ty, $trait_name:path) => {{
        trait __InnerMarkerTrait {
            fn __is_trait_inner_method() -> bool {
                false
            }
        }

        struct __TraitTest<T>(T);

        impl<T: $trait_name> __TraitTest<T> {
            fn __is_trait_inner_method() -> bool {
                true
            }
        }

        impl<T> __InnerMarkerTrait for __TraitTest<T> {}
        __TraitTest::<$name>::__is_trait_inner_method()
    }};
}

trait A {}

#[derive(Clone)]
struct B;

#[derive(Clone)]
struct F;

impl A for B {}

trait C {}

impl C for B {}

fn downcast(arg: Box<dyn std::any::Any>) -> impl A {
    *arg.downcast::<B>().unwrap()
}

fn is_downcastable_to_b(arg: Box<dyn std::any::Any>) -> bool {
    let Some(_) = arg.downcast_ref::<B>() else {
        return false;
    };
    true
}

fn is_downcastable_to_c(arg: Box<dyn std::any::Any>) -> bool {
    let Some(_) = arg.downcast_ref::<F>() else {
        return false;
    };
    true
}

// Trait checking - if B is type that implements A

// What we want is:
// - checking logic to see if an instance is of type that implements certain trait
// - if the condition is met, we want to pass them onto different function that consumes argument of the type

// Ex: DDD

trait Aggregate {}

trait Repository {
    type Aggregate: Aggregate;

    fn add(&self, aggregate: &mut Self::Aggregate);

    // opt-in macro
    // #[event_hook]
    fn update(&self, aggregate: &mut Self::Aggregate);

    // fn event_hook(&mut self, aggregate: &mut dyn A) {
    //     println!("event hook caflled! {:?}", aggregate)
    // }
}

struct OrderAggregate;
struct OrderRepository;

// impl Repository for OrderRepository {
// type Aggregate = OrderAggregate;

// fn add(&self, aggregate: &mut Self::Aggregate) {
//     todo!()
// }

// fn update(&self, aggregate: &mut Self::Aggregate) {
//     todo!()
// }
// }

// But then in rust cllient can atach inherent implementation for struct
impl OrderRepository {
    // #[event_hook]
    // fn add_agg_with_args(&self, aggregate: &mut Self::Aggregate, name: String) {
    //     todo!();
    // }
}

impl OrderRepository {
    // #[event_hook]
    fn add_agg_with_args(&mut self, aggregate: &mut OrderAggregate, name: String) {
        trait IsAggregateNotImplemented {
            const IS_AGGREGATE: bool = false;

            // The following takes Any type and return Any type T depending of the context
            fn get_aggregate<T>(_: impl std::any::Any) -> &'static mut T {
                unreachable!()
            }
        }

        impl<T> IsAggregateNotImplemented for T {}
        struct IsAggregate<T>(::core::marker::PhantomData<T>);

        #[allow(unused)]
        impl<T: Aggregate> IsAggregate<T> {
            const IS_AGGREGATE: bool = true;

            fn get_aggregate(data: &mut T) -> &mut T {
                data
            }
        }

        if <IsAggregate<OrderAggregate>>::IS_AGGREGATE {
            // self.event_hook(<IsAggregate<OrderAggregate>>::get_aggregate(aggregate));
        }

        if <IsAggregate<String>>::IS_AGGREGATE {
            // self.event_hook(<IsAggregate<String>>::get_aggregate(name)) // this is unreachable
        }

        // main logic .. .
    }
}

#[cfg(test)]
mod tests {
    use crate::rust_reflection::{downcast, is_downcastable_to_b, A, B, F};

    fn example() {
        assert!(is_trait!(B, A));

        let b = B;
        let c = downcast(Box::new(b.clone()));

        let f = F;
        // let ff = downcast(Box::new(f.clone()));

        assert_eq!(true, is_downcastable_to_b(Box::new(b)));
        assert_eq!(false, is_downcastable_to_b(Box::new(f)));
    }
}
