# Riverview Adventure Company - Landing Page

Award-winning design inspired by [Inversa](https://inversa.com/) and [Awwwards](https://www.awwwards.com/sites/inversa).

## Design Features

- **Dark cinematic theme** with subtle color accents
- **Bold typography** - Inter font, dramatic scale contrasts
- **Scroll-triggered animations** - Elements reveal on viewport entry
- **Interactive rental showcase** - Click-to-switch equipment display
- **Stats bar** - Key metrics in high-contrast cards
- **Three-column value proposition** - Float/Paddle/Explore
- **Testimonial section** - Customer quotes with counter
- **Step process** - Visual how-it-works flow
- **Grayscale map** - Dark-mode inverted Google Maps
- **Responsive** - Mobile-first, works on all devices

## Quick Deploy

### Cloudflare Pages
1. Push to GitHub
2. Cloudflare Dashboard → Pages → Connect repo
3. Build: none, Output: `/`
4. Add custom domain

### GitHub Pages
1. Push to `main` branch
2. Settings → Pages → Deploy from `main`
3. Add CNAME file

## Form Setup

Replace `YOUR-ENDPOINT` in index.html with your Formspree endpoint:
1. Sign up at [formspree.io](https://formspree.io)
2. Create new form
3. Copy endpoint ID
4. Update the form action URL

## Files

```
riverviewadventurecompany/
├── index.html      # Main page (384 lines)
├── styles.css      # Full Inversa-style CSS (966 lines)
├── sitemap.xml     # SEO sitemap
├── robots.txt      # Crawler rules
├── README.md       # This file
└── assets/
    ├── building.jpg   # Hero background (drone shot)
    ├── kayaks.jpg     # River fleet photo
    ├── bridge.jpg     # Wisconsin River bridge
    ├── bikes.jpg      # Bike rental icon
    ├── logo.png       # Company logo
    ├── sup.png        # SUP silhouette
    └── launchpad.png  # Launch Pad branding
```

## Customization

### Colors (in styles.css)
```css
:root {
    --bg-dark: #0a0a0a;
    --accent-river: #3b82f6;    /* Blue */
    --accent-forest: #22c55e;   /* Green */
    --accent-warm: #f59e0b;     /* Orange */
}
```

### Content
- Edit testimonials in the `.testimonials` section
- Update rental pricing in the JavaScript `rentalData` object
- Swap images in `/assets/`

## Contact

Riverview Adventure Company
740 Water Street, Sauk City, WI 53583
608-515-3456
info@riverviewadventurecompany.com

---

©2026 Riverview Adventure Company