use leptos::*;
use leptos_markdown::*;


#[component]
pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
    // create a reactive signal with the initial value
    let (value, set_value) = create_signal(initial_value);

    // create event handlers for our buttons
    // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // create user interfaces with the declarative `view!` macro
    view! {
        <div>
            <button on:click=clear>Clear</button>
            <button on:click=decrement>-1</button>
            // text nodes can be quoted or unquoted
            <span>"Value: " {value} "!"</span>
            <button on:click=increment>+1</button>
        </div>
    }
}

fn counter(props: MdComponentProps) -> impl IntoView {
    let initial: i32 = props.attributes.into_iter()
        .find(|(name, _)| name=="initial")
        .and_then(|(_, value)| value.parse().ok())
        .unwrap_or(0);

    view!{
        <SimpleCounter initial_value=initial/>
    }
}

fn box_component(props: MdComponentProps) -> impl IntoView {
    view!{
        <div style="border: 2px solid blue">
            {props.children}
        </div>
    }
}

static MARKDOWN: &'static str = r#"
# The source
```md
## Here is a counter:
<Counter initial="5"/>

## Here is a Box:
<box>

**I am in a blue box !**

</box>
```

---

# The result

## Here is a counter:
<Counter initial="5"/>

## Here is a Box:
<box>

**I am in a blue box !**

</box>
"#;

#[component]
fn App(
    ) -> impl IntoView {
    let components = ComponentMap::new()
        .add("Counter", counter)
        .add("box", box_component);

    view!{
        <Markdown
            components=components
            src=MARKDOWN
        />
    }
}


fn main(){
    mount_to_body(App)
}
