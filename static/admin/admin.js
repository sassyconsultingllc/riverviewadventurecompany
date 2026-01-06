/**
 * Riverview Adventure Company - Admin Dashboard JavaScript
 */

(function() {
    'use strict';

    const API_BASE = '/api/admin';
    let authToken = localStorage.getItem('admin_token');

    // Check authentication on load
    async function checkAuth() {
        if (!authToken) {
            window.location.href = '/admin/login.html';
            return false;
        }

        try {
            const response = await fetch(`${API_BASE}/verify`, {
                headers: { 'Authorization': `Bearer ${authToken}` }
            });
            
            if (!response.ok) {
                localStorage.removeItem('admin_token');
                window.location.href = '/admin/login.html';
                return false;
            }
            return true;
        } catch (error) {
            console.error('Auth check failed:', error);
            return false;
        }
    }

    // Login form handler
    function setupLoginForm() {
        const form = document.getElementById('login-form');
        if (!form) return;

        form.addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const totp = document.getElementById('totp').value;
            const errorEl = document.getElementById('login-error');
            const submitBtn = form.querySelector('button[type="submit"]');
            
            submitBtn.disabled = true;
            submitBtn.textContent = 'Verifying...';
            errorEl.style.display = 'none';

            try {
                const response = await fetch(`${API_BASE}/login`, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({ totp })
                });

                if (response.ok) {
                    const data = await response.json();
                    localStorage.setItem('admin_token', data.token);
                    window.location.href = '/admin/';
                } else {
                    errorEl.textContent = 'Invalid code. Please try again.';
                    errorEl.style.display = 'block';
                }
            } catch (error) {
                errorEl.textContent = 'Connection error. Please try again.';
                errorEl.style.display = 'block';
            } finally {
                submitBtn.disabled = false;
                submitBtn.textContent = 'Login';
            }
        });
    }

    // Load dashboard data
    async function loadDashboard() {
        if (!await checkAuth()) return;

        // Load services status
        await loadServices();
        
        // Load recent activity
        await loadActivity();
        
        // Load stats
        await loadStats();
    }

    async function loadServices() {
        try {
            const response = await fetch('/api/services');
            const services = await response.json();
            
            const container = document.getElementById('services-list');
            if (!container) return;

            const serviceNames = {
                tubing: 'Tubing',
                kayaking: 'Kayaking',
                canoeing: 'Canoeing',
                bikes: 'Bike Rentals',
                ebikes: 'E-Bike Sales'
            };

            container.innerHTML = Object.entries(services).map(([key, value]) => `
                <div class="service-toggle">
                    <span class="service-toggle__name">${serviceNames[key] || key}</span>
                    <label class="toggle">
                        <input type="checkbox" ${value ? 'checked' : ''} data-service="${key}">
                        <span class="toggle__slider"></span>
                    </label>
                </div>
            `).join('');

            // Add toggle handlers
            container.querySelectorAll('input[type="checkbox"]').forEach(input => {
                input.addEventListener('change', async (e) => {
                    const service = e.target.dataset.service;
                    const enabled = e.target.checked;
                    await updateService(service, enabled);
                });
            });
        } catch (error) {
            console.error('Failed to load services:', error);
        }
    }

    async function updateService(service, enabled) {
        try {
            await fetch(`${API_BASE}/services/${service}`, {
                method: 'PUT',
                headers: {
                    'Authorization': `Bearer ${authToken}`,
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ enabled })
            });
        } catch (error) {
            console.error('Failed to update service:', error);
        }
    }

    async function loadActivity() {
        // Placeholder for activity log
        const container = document.getElementById('activity-log');
        if (!container) return;

        container.innerHTML = `
            <div class="activity-item">
                <span class="activity-time">Just now</span>
                <span class="activity-text">Dashboard loaded</span>
            </div>
        `;
    }

    async function loadStats() {
        try {
            const response = await fetch('/api/conditions');
            const data = await response.json();

            // Update flow stat
            const flowEl = document.getElementById('stat-flow');
            if (flowEl && data.flow) {
                flowEl.textContent = `${Math.round(data.flow.flow_cfs).toLocaleString()} CFS`;
            }

            // Update weather stat
            const weatherEl = document.getElementById('stat-weather');
            if (weatherEl && data.weather) {
                weatherEl.textContent = `${Math.round(data.weather.temperature_f)}°F`;
            }
        } catch (error) {
            console.error('Failed to load stats:', error);
        }
    }

    // Logout handler
    function setupLogout() {
        const logoutBtn = document.getElementById('logout-btn');
        if (!logoutBtn) return;

        logoutBtn.addEventListener('click', () => {
            localStorage.removeItem('admin_token');
            window.location.href = '/admin/login.html';
        });
    }

    // Initialize
    function init() {
        if (window.location.pathname.includes('login')) {
            setupLoginForm();
        } else {
            loadDashboard();
            setupLogout();
        }
    }

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }
})();
