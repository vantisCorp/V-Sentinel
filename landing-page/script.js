// Smooth scrolling for navigation links
document.querySelectorAll('a[href^="#"]').forEach(anchor => {
    anchor.addEventListener('click', function (e) {
        e.preventDefault();
        const target = document.querySelector(this.getAttribute('href'));
        if (target) {
            target.scrollIntoView({
                behavior: 'smooth',
                block: 'start'
            });
        }
    });
});

// Navbar scroll effect
window.addEventListener('scroll', function() {
    const navbar = document.querySelector('.navbar');
    if (window.scrollY > 50) {
        navbar.style.backgroundColor = 'rgba(10, 10, 10, 0.98)';
        navbar.style.boxShadow = '0 2px 10px rgba(0, 255, 136, 0.1)';
    } else {
        navbar.style.backgroundColor = 'rgba(10, 10, 10, 0.95)';
        navbar.style.boxShadow = 'none';
    }
});

// Intersection Observer for animations
const observerOptions = {
    threshold: 0.1,
    rootMargin: '0px 0px -50px 0px'
};

const observer = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            entry.target.style.opacity = '1';
            entry.target.style.transform = 'translateY(0)';
        }
    });
}, observerOptions);

// Observe all cards
document.querySelectorAll('.feature-card, .solution-card, .pricing-card').forEach(card => {
    card.style.opacity = '0';
    card.style.transform = 'translateY(30px)';
    card.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
    observer.observe(card);
});

// Counter animation for stats
function animateCounter(element, target, duration = 2000) {
    let start = 0;
    const increment = target / (duration / 16);
    
    function updateCounter() {
        start += increment;
        if (start < target) {
            element.textContent = Math.floor(start).toLocaleString();
            requestAnimationFrame(updateCounter);
        } else {
            element.textContent = target.toLocaleString();
        }
    }
    
    updateCounter();
}

// Observe stats for counter animation
const statsObserver = new IntersectionObserver((entries) => {
    entries.forEach(entry => {
        if (entry.isIntersecting) {
            const statNumbers = entry.target.querySelectorAll('.stat-number, .gaming-value');
            statNumbers.forEach(stat => {
                const text = stat.textContent;
                const match = text.match(/[\d.]+/);
                if (match) {
                    const value = parseFloat(match[0]);
                    animateCounter(stat, value);
                }
            });
            statsObserver.unobserve(entry.target);
        }
    });
}, { threshold: 0.5 });

document.querySelectorAll('.hero-stats, .gaming-stats').forEach(stats => {
    statsObserver.observe(stats);
});

// Form submission handler
const contactForm = document.querySelector('.contact-form form');
if (contactForm) {
    contactForm.addEventListener('submit', function(e) {
        e.preventDefault();
        
        const formData = new FormData(this);
        const data = Object.fromEntries(formData);
        
        // Simulate form submission
        const submitBtn = this.querySelector('button[type="submit"]');
        const originalText = submitBtn.textContent;
        
        submitBtn.textContent = 'Sending...';
        submitBtn.disabled = true;
        
        setTimeout(() => {
            submitBtn.textContent = 'Message Sent!';
            submitBtn.style.backgroundColor = '#00ff88';
            submitBtn.style.color = '#000';
            
            setTimeout(() => {
                submitBtn.textContent = originalText;
                submitBtn.style.backgroundColor = '';
                submitBtn.style.color = '';
                submitBtn.disabled = false;
                this.reset();
            }, 2000);
        }, 1500);
    });
}

// Pricing plan selection
const pricingCards = document.querySelectorAll('.pricing-card button');
pricingCards.forEach(button => {
    button.addEventListener('click', function() {
        const card = this.closest('.pricing-card');
        const plan = card.querySelector('.pricing-header h3').textContent;
        
        // Scroll to contact form and select the interest
        const contactSection = document.getElementById('contact');
        const interestSelect = document.querySelector('#interest');
        
        if (interestSelect) {
            if (plan === 'Gaming') {
                interestSelect.value = 'early-access';
            } else if (plan === 'Privacy') {
                interestSelect.value = 'early-access';
            } else if (plan === 'Tech Enthusiast') {
                interestSelect.value = 'early-access';
            } else if (plan === 'Enterprise') {
                interestSelect.value = 'partner';
            }
        }
        
        contactSection.scrollIntoView({
            behavior: 'smooth',
            block: 'start'
        });
    });
});

// Launch offer claim
const launchOffer = document.querySelector('.launch-offer button');
if (launchOffer) {
    launchOffer.addEventListener('click', function() {
        const contactSection = document.getElementById('contact');
        const interestSelect = document.querySelector('#interest');
        
        if (interestSelect) {
            interestSelect.value = 'early-access';
        }
        
        contactSection.scrollIntoView({
            behavior: 'smooth',
            block: 'start'
        });
    });
}

// Parallax effect for hero section
window.addEventListener('scroll', function() {
    const hero = document.querySelector('.hero');
    const scrolled = window.scrollY;
    
    if (hero) {
        hero.style.backgroundPositionY = scrolled * 0.5 + 'px';
    }
});

// Feature card hover effect enhancement
document.querySelectorAll('.feature-card').forEach(card => {
    card.addEventListener('mouseenter', function() {
        this.style.transform = 'translateY(-10px) scale(1.02)';
    });
    
    card.addEventListener('mouseleave', function() {
        this.style.transform = 'translateY(0) scale(1)';
    });
});

// Loading animation
window.addEventListener('load', function() {
    document.body.style.opacity = '0';
    document.body.style.transition = 'opacity 0.5s ease';
    
    setTimeout(() => {
        document.body.style.opacity = '1';
    }, 100);
});

// Dynamic year in footer
const currentYear = new Date().getFullYear();
const footerYear = document.querySelector('.footer-bottom p');
if (footerYear) {
    footerYear.textContent = footerYear.textContent.replace('2025', currentYear);
}

// Mobile menu toggle (if needed for future implementation)
function toggleMobileMenu() {
    const navLinks = document.querySelector('.nav-links');
    navLinks.style.display = navLinks.style.display === 'flex' ? 'none' : 'flex';
}

// Performance table highlight on hover
document.querySelectorAll('.comparison-table tbody tr').forEach(row => {
    row.addEventListener('mouseenter', function() {
        this.style.backgroundColor = 'rgba(0, 255, 136, 0.05)';
    });
    
    row.addEventListener('mouseleave', function() {
        this.style.backgroundColor = '';
    });
});

// Testimonial animation (if added in future)
function animateTestimonials() {
    const testimonials = document.querySelectorAll('.testimonial-card');
    if (testimonials.length > 0) {
        let current = 0;
        
        setInterval(() => {
            testimonials[current].classList.remove('active');
            current = (current + 1) % testimonials.length;
            testimonials[current].classList.add('active');
        }, 5000);
    }
}

// Initialize animations on page load
document.addEventListener('DOMContentLoaded', function() {
    // Add loading class
    document.body.classList.add('page-loaded');
    
    // Trigger any initial animations
    setTimeout(() => {
        document.querySelectorAll('.feature-card').forEach((card, index) => {
            card.style.animationDelay = `${index * 0.1}s`;
        });
    }, 500);
});

// Performance optimization: Lazy load images
if ('IntersectionObserver' in window) {
    const imageObserver = new IntersectionObserver((entries, observer) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const img = entry.target;
                img.src = img.dataset.src;
                img.classList.add('loaded');
                observer.unobserve(img);
            }
        });
    });
    
    document.querySelectorAll('img[data-src]').forEach(img => {
        imageObserver.observe(img);
    });
}

// Console welcome message
console.log('%c🛡️ SENTINEL - AI-Native Security Platform', 'font-size: 24px; font-weight: bold; color: #00ff88;');
console.log('%cThe future of cybersecurity is here', 'font-size: 14px; color: #0066cc;');
console.log('%cJoin us at https://github.com/vantisCorp/V-Sentinel', 'font-size: 12px; color: #666;');