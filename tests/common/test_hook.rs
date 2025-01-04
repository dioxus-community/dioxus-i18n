// Lifted from: https://dioxuslabs.com/learn/0.6/cookbook/testing
//
// Much curtialed functionality and massaged to use in the local testing
// here. This hook isn't intended for reuse.
//

use dioxus::{dioxus_core::NoOpMutations, prelude::*};
use futures::FutureExt;

use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub(crate) fn test_hook<V: 'static>(
    initialize: impl FnMut() -> V + 'static,
    check: impl FnMut(V, &mut Assertions) + 'static,
) {
    #[derive(Props)]
    struct MockAppComponent<I: 'static, C: 'static> {
        hook: Rc<RefCell<I>>,
        check: Rc<RefCell<C>>,
    }

    impl<I, C> PartialEq for MockAppComponent<I, C> {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    impl<I, C> Clone for MockAppComponent<I, C> {
        fn clone(&self) -> Self {
            Self {
                hook: self.hook.clone(),
                check: self.check.clone(),
            }
        }
    }

    fn mock_app<I: FnMut() -> V, C: FnMut(V, &mut Assertions), V>(
        props: MockAppComponent<I, C>,
    ) -> Element {
        let value = props.hook.borrow_mut()();

        let mut assertions = Assertions::new();

        props.check.borrow_mut()(value, &mut assertions);

        rsx! { div {} }
    }

    let mut vdom = VirtualDom::new_with_props(
        mock_app,
        MockAppComponent {
            hook: Rc::new(RefCell::new(initialize)),
            check: Rc::new(RefCell::new(check)),
        },
    );

    vdom.rebuild_in_place();

    while vdom.wait_for_work().now_or_never().is_some() {
        vdom.render_immediate(&mut NoOpMutations);
    }

    vdom.in_runtime(|| ScopeId::ROOT.in_runtime(|| {}))
}

#[derive(Debug)]
pub(crate) struct Assertions {}

impl Assertions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn assert<T>(&mut self, actual: T, expected: T, id: &str)
    where
        T: PartialEq + Debug,
    {
        if actual != expected {
            eprintln!(
                "***** ERROR in {}: actual: '{:?}' != expected: '{:?}' *****\n",
                id, actual, expected
            );
            std::process::exit(-1);
        };
    }
}
