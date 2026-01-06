/**
 * Riverview Adventure Company - Animations JavaScript
 * Scroll-triggered animations and effects
 */

(function() {
    'use strict';

    // Intersection Observer for scroll animations
    const observerOptions = {
        root: null,
        rootMargin: '0px',
        threshold: 0.1
    };

    const animateOnScroll = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('animate-in');
                animateOnScroll.unobserve(entry.target);
            }
        });
    }, observerOptions);

    // Initialize animations
    function init() {
        // Observe elements with animation classes
        const animatedElements = document.querySelectorAll('.animate-on-scroll, .feature, .service-card, .dashboard-card');
        animatedElements.forEach(el => {
            animateOnScroll.observe(el);
        });
    }

    // Start when DOM is ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
