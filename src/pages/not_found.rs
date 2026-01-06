use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    html! {
        <div class="page-not-found">
            <div class="not-found-content">
                <div class="error-code">
                    <span class="code-404">{"404"}</span>
                    <span class="code-text">{"NOT_FOUND"}</span>
                </div>
                
                <div class="terminal-error">
                    <pre>
{r#"
$ curl -I https://fossmeet.in/this-page
HTTP/2 404 
content-type: text/html; charset=utf-8
x-error: ENOENT - Page not found

Error: The page you're looking for has been 
moved, deleted, or never existed.

Possible causes:
  - Typo in the URL
  - Outdated bookmark
  - Page has been relocated
  - You're in the wrong timeline

Suggested actions:
  > navigate --to home
  > search --query "what you need"
  > contact --help maintainers
"#}
                    </pre>
                </div>
                
                <div class="not-found-actions">
                    <Link<Route> to={Route::Home} classes="btn btn-primary">
                        {"cd ~  (Go Home)"}
                    </Link<Route>>
                    <a href="javascript:history.back()" class="btn btn-secondary">
                        {"cd ..  (Go Back)"}
                    </a>
                </div>
                
                <div class="not-found-decoration">
                    <pre class="ascii-sad">
{r#"
    ╭━━━━━━━━━━━━━━╮
    ┃   ಠ_ಠ        ┃
    ┃   Page not   ┃
    ┃    found     ┃
    ╰━━━━━━━━━━━━━━╯
"#}
                    </pre>
                </div>
            </div>
        </div>
    }
}
