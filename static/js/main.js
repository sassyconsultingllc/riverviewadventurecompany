/**
 * Riverview Adventure Company - Main JavaScript
 * Accessible, performant, and SEO-friendly
 */

(function() {
    'use strict';

    // ==========================================================================
    // Utility Functions
    // ==========================================================================
    
    /**
     * Debounce function for performance optimization
     */
    function debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    /**
     * Format number with commas
     */
    function formatNumber(num) {
        return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
    }

    /**
     * Format currency
     */
    function formatCurrency(amount) {
        return new Intl.NumberFormat('en-US', {
            style: 'currency',
            currency: 'USD',
            minimumFractionDigits: 0,
            maximumFractionDigits: 0
        }).format(amount);
    }

    // ==========================================================================
    // Header Scroll Effect
    // ==========================================================================
    
    const header = document.querySelector('.header');
    
    if (header) {
        const handleScroll = debounce(() => {
            if (window.scrollY > 50) {
                header.classList.add('scrolled');
            } else {
                header.classList.remove('scrolled');
            }
        }, 10);
        
        window.addEventListener('scroll', handleScroll, { passive: true });
    }

    // ==========================================================================
    // Mobile Navigation
    // ==========================================================================
    
    const menuToggle = document.querySelector('.menu-toggle');
    const mobileNav = document.querySelector('.mobile-nav');
    const mobileNavClose = document.querySelector('.mobile-nav__close');
    
    if (menuToggle && mobileNav) {
        // Open mobile nav
        menuToggle.addEventListener('click', () => {
            mobileNav.classList.add('is-open');
            mobileNav.setAttribute('aria-hidden', 'false');
            menuToggle.setAttribute('aria-expanded', 'true');
            document.body.style.overflow = 'hidden';
            
            // Focus first link
            const firstLink = mobileNav.querySelector('.mobile-nav__link');
            if (firstLink) firstLink.focus();
        });
        
        // Close mobile nav
        const closeMobileNav = () => {
            mobileNav.classList.remove('is-open');
            mobileNav.setAttribute('aria-hidden', 'true');
            menuToggle.setAttribute('aria-expanded', 'false');
            document.body.style.overflow = '';
            menuToggle.focus();
        };
        
        if (mobileNavClose) {
            mobileNavClose.addEventListener('click', closeMobileNav);
        }
        
        // Close on escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && mobileNav.classList.contains('is-open')) {
                closeMobileNav();
            }
        });
        
        // Close on link click
        mobileNav.querySelectorAll('.mobile-nav__link').forEach(link => {
            link.addEventListener('click', closeMobileNav);
        });
        
        // Trap focus in mobile nav
        mobileNav.addEventListener('keydown', (e) => {
            if (e.key !== 'Tab') return;
            
            const focusableElements = mobileNav.querySelectorAll(
                'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
            );
            const firstElement = focusableElements[0];
            const lastElement = focusableElements[focusableElements.length - 1];
            
            if (e.shiftKey && document.activeElement === firstElement) {
                e.preventDefault();
                lastElement.focus();
            } else if (!e.shiftKey && document.activeElement === lastElement) {
                e.preventDefault();
                firstElement.focus();
            }
        });
    }

    // ==========================================================================
    // Smooth Scroll for Anchor Links
    // ==========================================================================
    
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            const href = this.getAttribute('href');
            if (href === '#') return;
            
            const target = document.querySelector(href);
            if (target) {
                e.preventDefault();
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
                
                // Update focus for accessibility
                target.setAttribute('tabindex', '-1');
                target.focus();
            }
        });
    });

    // ==========================================================================
    // Load E-Bikes Grid
    // ==========================================================================
    
    const ebikesGrid = document.getElementById('ebikes-grid');
    
    if (ebikesGrid) {
        fetch('/api/bikes')
            .then(response => response.json())
            .then(bikes => {
                // Show first 4 bikes
                const displayBikes = bikes.slice(0, 4);
                
                ebikesGrid.innerHTML = displayBikes.map(bike => `
                    <article class="product-card" role="listitem">
                        <div class="product-card__image" style="background-image: url('${bike.image_url}');" role="img" aria-label="${bike.name}"></div>
                        <div class="product-card__content">
                            <div class="product-card__category">${bike.category}</div>
                            <h3 class="product-card__title">${bike.name}</h3>
                            <div class="product-card__price">
                                <span class="product-card__price-current">${formatCurrency(bike.price)}</span>
                                ${bike.msrp > bike.price ? `<span class="product-card__price-original">${formatCurrency(bike.msrp)}</span>` : ''}
                            </div>
                        </div>
                    </article>
                `).join('');
            })
            .catch(error => {
                console.error('Failed to load e-bikes:', error);
                ebikesGrid.innerHTML = `
                    <div style="grid-column: 1 / -1; text-align: center; padding: var(--space-8);">
                        <p>Unable to load e-bikes. Please try again later.</p>
                        <a href="/ebikes" class="btn btn--primary mt-4">View All E-Bikes</a>
                    </div>
                `;
            });
    }

    // ==========================================================================
    // Intersection Observer for Animations
    // ==========================================================================
    
    if ('IntersectionObserver' in window) {
        const animateOnScroll = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('is-visible');
                    animateOnScroll.unobserve(entry.target);
                }
            });
        }, {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        });
        
        document.querySelectorAll('.service-card, .product-card, .section__header').forEach(el => {
            el.classList.add('animate-on-scroll');
            animateOnScroll.observe(el);
        });
    }

    // ==========================================================================
    // Form Validation
    // ==========================================================================
    
    const forms = document.querySelectorAll('form[data-validate]');
    
    forms.forEach(form => {
        form.addEventListener('submit', function(e) {
            let isValid = true;
            
            // Clear previous errors
            form.querySelectorAll('.form-error').forEach(error => error.remove());
            form.querySelectorAll('.form-input, .form-textarea').forEach(input => {
                input.classList.remove('is-invalid');
                input.removeAttribute('aria-invalid');
                input.removeAttribute('aria-describedby');
            });
            
            // Validate required fields
            form.querySelectorAll('[required]').forEach(field => {
                if (!field.value.trim()) {
                    isValid = false;
                    showFieldError(field, 'This field is required');
                }
            });
            
            // Validate email fields
            form.querySelectorAll('[type="email"]').forEach(field => {
                if (field.value && !isValidEmail(field.value)) {
                    isValid = false;
                    showFieldError(field, 'Please enter a valid email address');
                }
            });
            
            // Validate phone fields
            form.querySelectorAll('[type="tel"]').forEach(field => {
                if (field.value && !isValidPhone(field.value)) {
                    isValid = false;
                    showFieldError(field, 'Please enter a valid phone number');
                }
            });
            
            if (!isValid) {
                e.preventDefault();
                // Focus first invalid field
                const firstInvalid = form.querySelector('.is-invalid');
                if (firstInvalid) firstInvalid.focus();
            }
        });
    });
    
    function showFieldError(field, message) {
        field.classList.add('is-invalid');
        field.setAttribute('aria-invalid', 'true');
        
        const errorId = `error-${field.name || Math.random().toString(36).substr(2, 9)}`;
        field.setAttribute('aria-describedby', errorId);
        
        const error = document.createElement('div');
        error.id = errorId;
        error.className = 'form-error';
        error.setAttribute('role', 'alert');
        error.textContent = message;
        
        field.parentNode.appendChild(error);
    }
    
    function isValidEmail(email) {
        return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
    }
    
    function isValidPhone(phone) {
        return /^[\d\s\-\+\(\)]{10,}$/.test(phone);
    }

    // ==========================================================================
    // Lazy Loading Images
    // ==========================================================================
    
    if ('loading' in HTMLImageElement.prototype) {
        // Native lazy loading supported
        document.querySelectorAll('img[data-src]').forEach(img => {
            img.src = img.dataset.src;
            img.loading = 'lazy';
        });
    } else {
        // Fallback for older browsers
        const lazyImages = document.querySelectorAll('img[data-src]');
        
        if ('IntersectionObserver' in window) {
            const imageObserver = new IntersectionObserver((entries) => {
                entries.forEach(entry => {
                    if (entry.isIntersecting) {
                        const img = entry.target;
                        img.src = img.dataset.src;
                        img.removeAttribute('data-src');
                        imageObserver.unobserve(img);
                    }
                });
            });
            
            lazyImages.forEach(img => imageObserver.observe(img));
        } else {
            // Fallback: load all images
            lazyImages.forEach(img => {
                img.src = img.dataset.src;
            });
        }
    }

    // ==========================================================================
    // Print Styles
    // ==========================================================================
    
    window.addEventListener('beforeprint', () => {
        // Expand all collapsed content before printing
        document.querySelectorAll('[aria-expanded="false"]').forEach(el => {
            el.setAttribute('aria-expanded', 'true');
        });
    });

    // ==========================================================================
    // Service Worker Registration (for PWA support)
    // ==========================================================================
    
    if ('serviceWorker' in navigator && window.location.protocol === 'https:') {
        window.addEventListener('load', () => {
            navigator.serviceWorker.register('/sw.js')
                .then(registration => {
                    console.log('ServiceWorker registered:', registration.scope);
                })
                .catch(error => {
                    console.log('ServiceWorker registration failed:', error);
                });
        });
    }

    // ==========================================================================
    // Analytics Events (placeholder for Google Analytics)
    // ==========================================================================
    
    // Track outbound links
    document.querySelectorAll('a[target="_blank"]').forEach(link => {
        link.addEventListener('click', () => {
            if (typeof gtag === 'function') {
                gtag('event', 'click', {
                    event_category: 'outbound',
                    event_label: link.href,
                    transport_type: 'beacon'
                });
            }
        });
    });
    
    // Track phone clicks
    document.querySelectorAll('a[href^="tel:"]').forEach(link => {
        link.addEventListener('click', () => {
            if (typeof gtag === 'function') {
                gtag('event', 'click', {
                    event_category: 'contact',
                    event_label: 'phone_call'
                });
            }
        });
    });
    
    // Track email clicks
    document.querySelectorAll('a[href^="mailto:"]').forEach(link => {
        link.addEventListener('click', () => {
            if (typeof gtag === 'function') {
                gtag('event', 'click', {
                    event_category: 'contact',
                    event_label: 'email'
                });
            }
        });
    });

})();
