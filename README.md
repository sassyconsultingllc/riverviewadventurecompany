# Riverview Adventure Company Website

A modern, high-performance website built with Rust and Cloudflare Workers, featuring real-time river conditions, weather data, and a comprehensive admin dashboard.

## Features

### Public Website
- **Modern, Responsive Design** - Mobile-first design with WCAG 2.1 AA accessibility
- **Real-time River Conditions** - Live flow data from USGS, weather, moon phases
- **SEO Optimized** - Schema.org structured data, optimized meta tags, sitemap
- **Fast Performance** - Edge-delivered via Cloudflare Workers, ~24KB WASM bundle

### Services Showcased
- River Tubing on the Wisconsin River
- Kayak & Canoe Rentals
- Bike Rentals
- E-Bike Sales (Velotric)
- "The Launch Pad" - Complete adventure hub

### Admin Dashboard
- TOTP-secured authentication
- Service availability toggles
- Pricing management
- Content management (announcements, hours)
- Analytics integration (Cloudflare Web Analytics)

### Real-time Data
- **USGS Water Services** - Flow rate (CFS), water temperature, gage height
- **Tomorrow.io Weather** - Current conditions, forecasts
- **NWS Alerts** - Weather warnings for Sauk County (WIZ061)
- **Moon Phase Calculator** - Pure Rust astronomical calculations
- **Sun Times** - Sunrise/sunset for Sauk City coordinates

## Technology Stack

- **Backend**: Rust + Cloudflare Workers (WASM)
- **Storage**: Cloudflare KV (caching, settings)
- **Frontend**: Vanilla HTML/CSS/JS (no framework overhead)
- **APIs**: USGS, Tomorrow.io, NWS
- **Authentication**: TOTP (RFC 6238)

## Project Structure

```
riverview-complete/
├── src/
│   ├── lib.rs              # Main router and entry point
│   ├── api/                # External API clients
│   │   ├── mod.rs
│   │   ├── usgs.rs         # USGS Water Services
│   │   └── weather.rs      # Tomorrow.io & NWS
│   ├── handlers/           # Request handlers
│   │   ├── mod.rs
│   │   ├── pages.rs        # HTML page serving
│   │   ├── static_files.rs # CSS/JS serving
│   │   ├── flow.rs         # River flow API
│   │   ├── weather.rs      # Weather API
│   │   ├── moon.rs         # Moon phase API
│   │   ├── conditions.rs   # Combined conditions
│   │   ├── historical.rs   # Historical data
│   │   ├── services.rs     # Service status
│   │   ├── store.rs        # E-bike inventory
│   │   ├── settings.rs     # Public settings
│   │   ├── admin.rs        # Admin dashboard
│   │   └── contact.rs      # Contact form
│   ├── models/             # Data structures
│   │   ├── mod.rs
│   │   ├── flow.rs
│   │   ├── weather.rs
│   │   ├── moon.rs
│   │   ├── services.rs
│   │   ├── store.rs
│   │   ├── settings.rs
│   │   └── admin.rs
│   └── utils/              # Utilities
│       ├── mod.rs
│       ├── cache.rs        # KV caching
│       └── auth.rs         # TOTP authentication
├── static/
│   ├── index.html          # Homepage
│   ├── about.html          # About page
│   ├── services.html       # Services page
│   ├── tubing.html         # Tubing details
│   ├── bikes.html          # Bike rentals
│   ├── ebikes.html         # E-bike sales
│   ├── conditions.html     # River conditions dashboard
│   ├── contact.html        # Contact form
│   ├── css/
│   │   ├── main.css        # Main styles
│   │   ├── dashboard.css   # Dashboard styles
│   │   └── animations.css  # Animations
│   ├── js/
│   │   ├── main.js         # Main JavaScript
│   │   ├── conditions.js   # Conditions dashboard
│   │   └── animations.js   # Animation scripts
│   ├── images/             # Brand images
│   └── admin/              # Admin dashboard
│       ├── index.html
│       ├── login.html
│       ├── settings.html
│       ├── services.html
│       ├── analytics.html
│       ├── content.html
│       ├── admin.css
│       └── admin.js
├── Cargo.toml              # Rust dependencies
├── wrangler.toml           # Cloudflare config
└── README.md               # This file
```

## Deployment

### Prerequisites
- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [wrangler](https://developers.cloudflare.com/workers/wrangler/) CLI
- [worker-build](https://crates.io/crates/worker-build)
- Cloudflare account

### Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/sassyconsultingllc/riverview-complete.git
   cd riverview-complete
   ```

2. **Configure Cloudflare**
   
   Create KV namespaces:
   ```bash
   wrangler kv:namespace create "CACHE"
   wrangler kv:namespace create "SETTINGS"
   ```
   
   Update `wrangler.toml` with the namespace IDs.

3. **Set secrets**
   ```bash
   wrangler secret put TOMORROW_IO_API_KEY
   wrangler secret put TOTP_SECRET
   ```

4. **Build and deploy**
   ```bash
   worker-build --release
   wrangler deploy
   ```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `USGS_STATION_ID` | USGS station for flow data | `05406000` |
| `LOCATION_LAT` | Latitude for weather | `43.2722` |
| `LOCATION_LON` | Longitude for weather | `-89.7208` |
| `NWS_ZONE` | NWS alert zone | `WIZ061` |

### Secrets

| Secret | Description |
|--------|-------------|
| `TOMORROW_IO_API_KEY` | Tomorrow.io API key for weather |
| `TOTP_SECRET` | Base32-encoded TOTP secret for admin |

## API Endpoints

### Public APIs

| Endpoint | Description |
|----------|-------------|
| `GET /api/flow` | Current river flow data |
| `GET /api/weather` | Current weather conditions |
| `GET /api/alerts` | Active weather alerts |
| `GET /api/moon` | Moon phase and sun times |
| `GET /api/conditions` | Combined conditions (all data) |
| `GET /api/historical/:period` | Historical data (yesterday, week, year) |
| `GET /api/services` | Service availability |
| `GET /api/bikes` | E-bike inventory |
| `GET /api/repairs` | Repair pricing |

### Admin APIs (TOTP Protected)

| Endpoint | Description |
|----------|-------------|
| `POST /api/admin/login` | Verify TOTP and get token |
| `GET /api/admin/settings` | Get all settings |
| `POST /api/admin/settings` | Update settings |
| `POST /api/admin/services` | Update service status |

## SEO Features

- **Schema.org Markup**: LocalBusiness, Product, Service, FAQPage
- **Open Graph Tags**: Full social media preview support
- **Twitter Cards**: Large image cards for sharing
- **Sitemap**: Auto-generated XML sitemap
- **Robots.txt**: Search engine directives
- **Canonical URLs**: Proper URL canonicalization

## Accessibility (WCAG 2.1 AA)

- Semantic HTML5 structure
- ARIA landmarks and labels
- Skip navigation links
- Keyboard navigation support
- Color contrast compliance
- Focus management
- Screen reader optimized

## Performance

- **Bundle Size**: ~24KB WASM (gzipped)
- **Edge Delivery**: Cloudflare's global network
- **Caching**: KV-based response caching (5 min TTL)
- **No Framework**: Vanilla JS for minimal overhead

## Local Development

```bash
# Install dependencies
cargo build

# Run locally with wrangler
wrangler dev

# Build for production
worker-build --release
```

## License

Copyright 2024-2026 Riverview Adventure Company / Sassy Consulting LLC

## Contact

- **Website**: https://riverviewadventurecompany.com
- **Phone**: (608) 643-6363
- **Address**: 106 Water St, Sauk City, WI 53583
