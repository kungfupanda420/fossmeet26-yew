use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TerminalProps {
    #[prop_or("Terminal".to_string())]
    pub title: String,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

/// A terminal-style code display component
#[function_component(Terminal)]
pub fn terminal(props: &TerminalProps) -> Html {
    html! {
        <div class={classes!("terminal", props.class.clone())}>
            <div class="terminal-header">
                <div class="terminal-dot red"></div>
                <div class="terminal-dot yellow"></div>
                <div class="terminal-dot green"></div>
                <span class="terminal-title">{&props.title}</span>
            </div>
            <div class="terminal-body">
                {props.children.clone()}
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TerminalLineProps {
    #[prop_or_default]
    pub prompt: Option<String>,
    #[prop_or_default]
    pub command: Option<String>,
    #[prop_or_default]
    pub output: Option<String>,
    #[prop_or_default]
    pub typing: bool,
}

/// A single line in a terminal
#[function_component(TerminalLine)]
pub fn terminal_line(props: &TerminalLineProps) -> Html {
    html! {
        <div class={classes!("terminal-line", props.typing.then_some("typing"))}>
            if let Some(ref prompt) = props.prompt {
                <span class="terminal-prompt">{prompt}</span>
            }
            if let Some(ref command) = props.command {
                <span class="terminal-command">{command}</span>
            }
            if let Some(ref output) = props.output {
                <span class="terminal-output">{output}</span>
            }
            if props.typing {
                <span class="terminal-cursor">{"_"}</span>
            }
        </div>
    }
}
