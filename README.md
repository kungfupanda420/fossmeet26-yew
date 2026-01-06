# FOSSMeet'26 Website

A modern, WebAssembly-powered website for FOSSMeet'26 - the annual Free and Open Source Software gathering at NIT Calicut.

## Tech Stack

- **Rust** - Systems programming language
- **Yew** - Modern Rust framework for creating web apps
- **WebAssembly** - Run Rust code in the browser
- **Trunk** - WASM web application bundler

## Theme

The website features an **esoteric hacker aesthetic** with:
- Orange (#FF6B00), Black (#0A0A0A), and White (#FAFAFA) color palette
- Terminal/code-inspired UI elements
- Matrix rain background effects
- Glitch text animations
- Monospace typography (JetBrains Mono)

## Project Structure

```
fossmeet-26/
├── src/
│   ├── components/     # Reusable UI components
│   │   ├── navbar.rs
│   │   ├── footer.rs
│   │   ├── speaker_card.rs
│   │   ├── workshop_card.rs
│   │   └── ...
│   ├── pages/          # Page components
│   │   ├── home.rs
│   │   ├── speakers.rs
│   │   ├── workshops.rs
│   │   ├── schedule.rs
│   │   └── not_found.rs
│   ├── hooks/          # Custom Yew hooks
│   ├── types.rs        # Type definitions
│   └── lib.rs          # Main application entry
├── styles/
│   ├── main.css        # Global styles
│   └── pages.css       # Page-specific styles
├── assets/             # Static assets
│   ├── speakers/
│   ├── workshops/
│   ├── sponsors/
│   └── icons/
├── index.html          # HTML template
├── Cargo.toml          # Rust dependencies
├── Trunk.toml          # Trunk configuration
└── README.md
```

## Getting Started

### Prerequisites

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Add WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. Install Trunk:
   ```bash
   cargo install trunk
   ```

### Development

1. Clone the repository:
   ```bash
   git clone https://github.com/fosscell/fossmeet-26.git
   cd fossmeet-26
   ```

2. Start the development server:
   ```bash
   trunk serve
   ```

3. Open your browser at `http://127.0.0.1:8080`

### Building for Production

```bash
trunk build --release
```

The compiled website will be in the `dist/` directory.

## Features

- **Home Page**: Hero section, about, speaker/workshop previews, sponsors
- **Speakers Page**: Speaker grid with modal details
- **Workshops Page**: Workshop cards with filtering
- **Schedule Page**: Day-by-day event timeline
- **Responsive Design**: Works on mobile, tablet, and desktop
- **Cool Effects**: Matrix rain, glitch text, terminal aesthetics

## Pages

| Route | Description |
|-------|-------------|
| `/` | Home page with overview |
| `/speakers` | All speakers with details |
| `/workshops` | Workshop listings |
| `/schedule` | Event schedule by day |
| `/404` | Not found page |

## Customization

### Colors

Edit the CSS variables in `styles/main.css`:

```css
:root {
    --color-primary: #FF6B00;
    --color-bg-dark: #0A0A0A;
    --color-text-primary: #FAFAFA;
}
```

### Data

Speaker, workshop, and schedule data is currently hardcoded in the respective page files. To integrate with a CMS:

1. Implement API fetching in `src/lib/`
2. Use `gloo-net` for HTTP requests
3. Update page components to use async data loading

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Credits

- **FOSSCell NIT Calicut** - Organizers
- **Yew Framework** - https://yew.rs
- **Trunk** - https://trunkrs.dev

---

*Free as in Freedom* - FOSSMeet'26
