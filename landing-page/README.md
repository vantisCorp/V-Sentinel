# SENTINEL Landing Page

A modern, responsive landing page for the SENTINEL AI-Native Security Platform.

## Features

- **Modern Design**: Dark theme with gradient accents and smooth animations
- **Responsive Layout**: Fully responsive design that works on all devices
- **Smooth Scrolling**: Smooth navigation between sections
- **Interactive Elements**: Hover effects, animations, and dynamic content
- **Performance Optimized**: Lazy loading images and optimized animations
- **Contact Form**: Interactive contact form with validation
- **Pricing Cards**: Professional pricing plans with hover effects
- **Timeline**: Visual roadmap showing 18-month development timeline
- **Performance Comparison**: Detailed comparison tables with competitor benchmarks
- **Gaming Stats**: Visual representation of gaming performance improvements

## File Structure

```
landing-page/
├── index.html      # Main HTML file
├── styles.css      # Complete styling
├── script.js       # Interactive functionality
└── README.md       # This file
```

## Usage

### Local Development

1. Clone the repository:
```bash
git clone https://github.com/vantisCorp/V-Sentinel.git
cd V-Sentinel/landing-page
```

2. Open `index.html` in your browser:
```bash
# On macOS
open index.html

# On Linux
xdg-open index.html

# On Windows
start index.html
```

Or use a local server:
```bash
# Using Python 3
python -m http.server 8000

# Using Node.js http-server
npx http-server -p 8000

# Using PHP
php -S localhost:8000
```

3. Open `http://localhost:8000` in your browser

### Deployment

The landing page can be deployed to any static hosting service:

#### Netlify
```bash
npm install -g netlify-cli
netlify deploy --prod
```

#### Vercel
```bash
npm install -g vercel
vercel --prod
```

#### GitHub Pages
```bash
# Enable GitHub Pages in repository settings
# Select 'main' branch and 'root' folder
```

#### AWS S3 + CloudFront
```bash
aws s3 sync ./landing-page s3://your-bucket-name --acl public-read
```

## Customization

### Colors
Edit the CSS variables in `styles.css`:

```css
:root {
    --primary-color: #00ff88;
    --secondary-color: #0066cc;
    --dark-bg: #0a0a0a;
    --card-bg: #111;
    /* ... */
}
```

### Content
Edit the content in `index.html`:
- Update company information
- Modify feature descriptions
- Adjust pricing plans
- Update performance statistics
- Change contact information

### Images
Replace placeholder images with actual screenshots:
```html
<img src="your-image.jpg" alt="Description">
```

## Features Breakdown

### Navigation
- Fixed navigation bar with smooth scrolling
- Mobile-responsive design
- Logo and tagline display

### Hero Section
- Eye-catching headline
- Call-to-action buttons
- Performance statistics with counter animations
- Gradient background with pattern overlay

### Features Section
- 8 feature cards with icons
- Hover effects and animations
- Grid layout for responsive design

### Performance Section
- Detailed comparison table
- Gaming performance statistics
- Visual data representation

### Solutions Section
- 4 solution cards for different user segments
- Feature lists for each solution
- Responsive grid layout

### Pricing Section
- 4 pricing plans with different features
- Featured plan highlighting
- Launch offer promotion
- Interactive pricing selection

### Timeline Section
- 18-month development roadmap
- Visual timeline with markers
- Alternating layout for visual interest

### Contact Section
- Contact information
- Interactive contact form
- Form validation and submission handling

### Footer
- Company information
- Navigation links
- Social media links
- Copyright notice

## Browser Support

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)
- Opera (latest)
- Mobile browsers (iOS Safari, Chrome Mobile)

## Performance

- **Page Load Time**: < 2 seconds
- **First Contentful Paint**: < 1 second
- **Time to Interactive**: < 3 seconds
- **Lighthouse Score**: 95+

## SEO Optimization

The landing page includes:
- Semantic HTML structure
- Meta tags for SEO
- Alt text for images
- Proper heading hierarchy
- Mobile-friendly design

## Analytics Integration

To add analytics, edit `script.js`:

```javascript
// Google Analytics
window.dataLayer = window.dataLayer || [];
function gtag(){dataLayer.push(arguments);}
gtag('js', new Date());
gtag('config', 'UA-XXXXXXXXX-X');
```

## Contact Form

The contact form includes:
- Name, email, interest dropdown, and message fields
- Form validation
- Submission feedback
- Interest pre-selection from pricing cards

## Future Enhancements

Potential improvements:
- Add testimonials section
- Integrate with email service (e.g., SendGrid, Mailchimp)
- Add live chat widget
- Implement newsletter signup
- Add video demo
- Create multiple language versions
- Add A/B testing capabilities

## Support

For questions or issues:
- GitHub Issues: https://github.com/vantisCorp/V-Sentinel/issues
- Email: support@sentinel.ai

## License

Copyright © 2025 SENTINEL. All rights reserved.

---

**Built with ❤️ by the SENTINEL team**