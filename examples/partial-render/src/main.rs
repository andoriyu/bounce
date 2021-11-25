use std::rc::Rc;

use bounce::prelude::*;
use bounce::BounceRoot;
use log::Level;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Debug)]
pub enum SliceAction {
    Increment,
}

#[derive(Default, PartialEq)]
pub struct SliceA(i64);

impl Slice for SliceA {
    type Action = SliceAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Increment => Self(self.0 + 1).into(),
        }
    }
}

#[derive(Default, PartialEq)]
pub struct SliceB(i64);

impl Slice for SliceB {
    type Action = SliceAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Increment => Self(self.0 + 1).into(),
        }
    }
}

#[derive(Default, PartialEq)]
pub struct SliceC(i64);

impl Slice for SliceC {
    type Action = SliceAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Self::Action::Increment => Self(self.0 + 1).into(),
        }
    }
}

#[styled_component(CompA)]
fn comp_a() -> Html {
    let a = use_slice_value::<SliceA>();

    let ctr = {
        let ctr = use_mut_ref(|| 0);

        let mut ctr = ctr.borrow_mut();

        *ctr += 1;

        *ctr
    };

    html! {
        <div>
            <p>{"Slice A: "}{a.0}</p>
            <p>{format!("Rendered: {} Time(s)", ctr)}</p>
        </div>
    }
}

#[styled_component(CompB)]
fn comp_b() -> Html {
    let b = use_slice_value::<SliceB>();

    let ctr = {
        let ctr = use_mut_ref(|| 0);

        let mut ctr = ctr.borrow_mut();

        *ctr += 1;

        *ctr
    };

    html! {
        <div>
            <p>{"Slice B: "}{b.0}</p>
            <p>{format!("Rendered: {} Time(s)", ctr)}</p>
        </div>
    }
}

#[styled_component(CompC)]
fn comp_c() -> Html {
    let c = use_slice_value::<SliceC>();

    let ctr = {
        let ctr = use_mut_ref(|| 0);

        let mut ctr = ctr.borrow_mut();

        *ctr += 1;

        *ctr
    };

    html! {
        <div>
            <p>{"Slice C: "}{c.0}</p>
            <p>{format!("Rendered: {} Time(s)", ctr)}</p>
        </div>
    }
}

#[styled_component(CompAB)]
fn comp_ab() -> Html {
    let a = use_slice_value::<SliceA>();
    let b = use_slice_value::<SliceB>();

    let ctr = {
        let ctr = use_mut_ref(|| 0);

        let mut ctr = ctr.borrow_mut();

        *ctr += 1;

        *ctr
    };

    html! {
        <div>
            <p>{"Slice A: "}{a.0}</p>
            <p>{"Slice B: "}{b.0}</p>
            <p>{format!("Rendered: {} Time(s)", ctr)}</p>
        </div>
    }
}

#[styled_component(CompAC)]
fn comp_ac() -> Html {
    let a = use_slice_value::<SliceA>();
    let c = use_slice_value::<SliceC>();

    let ctr = {
        let ctr = use_mut_ref(|| 0);

        let mut ctr = ctr.borrow_mut();

        *ctr += 1;

        *ctr
    };

    html! {
        <div>
            <p>{"Slice A: "}{a.0}</p>
            <p>{"Slice C: "}{c.0}</p>
            <p>{format!("Rendered: {} Time(s)", ctr)}</p>
        </div>
    }
}

#[styled_component(CompBC)]
fn comp_bc() -> Html {
    let b = use_slice_value::<SliceB>();
    let c = use_slice_value::<SliceC>();

    let ctr = {
        let ctr = use_mut_ref(|| 0);

        let mut ctr = ctr.borrow_mut();

        *ctr += 1;

        *ctr
    };

    html! {
        <div>
            <p>{"Slice B: "}{b.0}</p>
            <p>{"Slice C: "}{c.0}</p>
            <p>{format!("Rendered: {} Time(s)", ctr)}</p>
        </div>
    }
}

#[styled_component(CompABC)]
fn comp_abc() -> Html {
    let a = use_slice_value::<SliceA>();
    let b = use_slice_value::<SliceB>();
    let c = use_slice_value::<SliceC>();

    let ctr = {
        let ctr = use_mut_ref(|| 0);

        let mut ctr = ctr.borrow_mut();

        *ctr += 1;

        *ctr
    };

    html! {
        <div class={css!(r#"
            grid-column-start: 1;
            grid-column-end: 4;
        "#)}>
            <p>{"Slice A: "}{a.0}</p>
            <p>{"Slice B: "}{b.0}</p>
            <p>{"Slice C: "}{c.0}</p>
            <p>{format!("Rendered: {} Time(s)", ctr)}</p>
        </div>
    }
}

#[styled_component(Setters)]
fn setters() -> Html {
    let dispatch_a = use_dispatch_slice_action::<SliceA>();
    let dispatch_b = use_dispatch_slice_action::<SliceB>();
    let dispatch_c = use_dispatch_slice_action::<SliceC>();

    let inc_a = Callback::from(move |_| dispatch_a(SliceAction::Increment));
    let inc_b = Callback::from(move |_| dispatch_b(SliceAction::Increment));
    let inc_c = Callback::from(move |_| dispatch_c(SliceAction::Increment));

    html! {
        <div class={css!(r#"
        "#)}>
            <button onclick={inc_a}>{"Increase A"}</button>
            <button onclick={inc_b}>{"Increase B"}</button>
            <button onclick={inc_c}>{"Increase C"}</button>
        </div>
    }
}

#[styled_component(App)]
fn app() -> Html {
    html! {
        <BounceRoot>
            <div>
                <div class={css!(r#"
                    grid-template-columns: auto auto auto;
                    display: grid;

                    width: 600px;
                "#)}>
                    <CompA />
                    <CompB />
                    <CompC />

                    <CompAB />
                    <CompAC />
                    <CompBC />

                    <CompABC />
                </div>
                <Setters />
            </div>
        </BounceRoot>
    }
}

fn main() {
    console_log::init_with_level(Level::Trace).expect("Failed to initialise Log!");
    yew::start_app::<App>();
}
