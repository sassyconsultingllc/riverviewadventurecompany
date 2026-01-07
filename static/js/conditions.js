/**
 * Riverview Adventure Company - River Conditions Dashboard
 * Real-time river flow, weather, moon phase, and service status
 */

(function() {
    'use strict';

    // ==========================================================================
    // Configuration
    // ==========================================================================
    
    const CONFIG = {
        apiBase: '/api',
        refreshInterval: 5 * 60 * 1000, // 5 minutes
        thresholds: {
            low: 2000,
            ideal: 6000,
            high: 10000,
            danger: 15000
        }
    };

    // ==========================================================================
    // DOM Elements
    // ==========================================================================
    
    const elements = {
        // Status
        statusBanner: document.getElementById('status-banner'),
        statusText: document.getElementById('status-text'),
        conditionsStatus: document.getElementById('conditions-status'),
        lastUpdated: document.getElementById('last-updated'),
        
        // Flow
        flowValue: document.getElementById('flow-value'),
        flowStatus: document.getElementById('flow-status'),
        flowGauge: document.getElementById('flow-gauge'),
        flowYesterday: document.getElementById('flow-yesterday'),
        flowLastweek: document.getElementById('flow-lastweek'),
        
        // Weather
        airTemp: document.getElementById('air-temp'),
        waterTemp: document.getElementById('water-temp'),
        waterTempValue: document.getElementById('water-temp-value'),
        airTempValue: document.getElementById('air-temp-value'),
        weatherValue: document.getElementById('weather-value'),
        weatherDesc: document.getElementById('weather-desc'),
        weatherIcon: document.getElementById('weather-icon'),
        windSpeed: document.getElementById('wind-speed'),
        humidity: document.getElementById('humidity'),
        
        // Moon
        moonPhase: document.getElementById('moon-phase'),
        moonIllumination: document.getElementById('moon-illumination'),
        moonVisual: document.getElementById('moon-visual'),
        sunrise: document.getElementById('sunrise'),
        sunset: document.getElementById('sunset'),
        
        // Services
        servicesList: document.getElementById('services-list'),
        
        // Alerts
        alertsContainer: document.getElementById('alerts-container'),
        alertsList: document.getElementById('alerts-list'),
        
        // Map
        riverMap: document.getElementById('river-map')
    };

    // ==========================================================================
    // Status Helpers
    // ==========================================================================
    
    function getFlowStatus(cfs) {
        if (cfs < CONFIG.thresholds.low) {
            return { label: 'Low', class: 'warning', color: 'var(--color-warning)' };
        } else if (cfs < CONFIG.thresholds.ideal) {
            return { label: 'Ideal', class: 'success', color: 'var(--color-success)' };
        } else if (cfs < CONFIG.thresholds.high) {
            return { label: 'Moderate', class: 'success', color: 'var(--color-success)' };
        } else if (cfs < CONFIG.thresholds.danger) {
            return { label: 'High', class: 'warning', color: 'var(--color-warning)' };
        } else {
            return { label: 'Dangerous', class: 'danger', color: 'var(--color-danger)' };
        }
    }

    function formatNumber(num) {
        if (num === null || num === undefined || isNaN(num)) return '--';
        return Math.round(num).toLocaleString();
    }

    function formatTime(dateStr) {
        try {
            const date = new Date(dateStr);
            return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' });
        } catch {
            return dateStr || '--';
        }
    }

    function formatDateTime(date) {
        try {
            const d = date instanceof Date ? date : new Date(date);
            return d.toLocaleString('en-US', { 
                month: 'short', 
                day: 'numeric', 
                hour: 'numeric', 
                minute: '2-digit' 
            });
        } catch {
            return '--';
        }
    }

    // ==========================================================================
    // Data Fetching
    // ==========================================================================
    
    async function fetchConditions() {
        try {
            const response = await fetch(`${CONFIG.apiBase}/conditions`);
            if (!response.ok) throw new Error('Failed to fetch conditions');
            return await response.json();
        } catch (error) {
            console.error('Error fetching conditions:', error);
            return null;
        }
    }

    // ==========================================================================
    // UI Updates
    // ==========================================================================
    
    function updateConditionsWidget(data) {
        if (!data) {
            updateError();
            return;
        }

        updateFlowData(data);
        updateWeatherData(data);
        updateMoonData(data);
        updateServicesData(data);
        updateAlerts(data);
        updateStatusBanner(data);
        updateLastUpdated();
    }

    function updateFlowData(data) {
        const flow = data.flow;
        if (!flow) return;

        const cfs = flow.flow_cfs || flow.current || 0;
        const status = getFlowStatus(cfs);

        // Update flow value
        if (elements.flowValue) {
            elements.flowValue.textContent = formatNumber(cfs);
        }

        // Update status badge
        if (elements.flowStatus) {
            elements.flowStatus.textContent = status.label;
            elements.flowStatus.className = `dashboard-card__badge dashboard-card__badge--${status.class}`;
        }

        // Update gauge
        if (elements.flowGauge) {
            const percentage = Math.min(100, (cfs / CONFIG.thresholds.danger) * 100);
            elements.flowGauge.style.width = `${percentage}%`;
            elements.flowGauge.className = `gauge__fill gauge__fill--${status.class}`;
        }

        // Update historical comparisons
        if (elements.flowYesterday && flow.yesterday) {
            const diff = cfs - flow.yesterday;
            const arrow = diff > 0 ? '↑' : diff < 0 ? '↓' : '';
            elements.flowYesterday.textContent = `${formatNumber(flow.yesterday)} ${arrow}`;
        }

        if (elements.flowLastweek && flow.lastWeek) {
            const diff = cfs - flow.lastWeek;
            const arrow = diff > 0 ? '↑' : diff < 0 ? '↓' : '';
            elements.flowLastweek.textContent = `${formatNumber(flow.lastWeek)} ${arrow}`;
        }

        // Update homepage widget status
        if (elements.conditionsStatus) {
            const statusDot = elements.conditionsStatus.querySelector('.conditions-widget__status-dot');
            const statusText = elements.conditionsStatus.querySelector('span:last-child');
            
            if (statusDot) statusDot.style.background = status.color;
            if (statusText) {
                if (status.class === 'danger') {
                    statusText.textContent = 'High Water - Closed';
                } else if (status.class === 'warning' && cfs > CONFIG.thresholds.high) {
                    statusText.textContent = 'Exercise Caution';
                } else if (status.class === 'warning') {
                    statusText.textContent = 'Low Water';
                } else {
                    statusText.textContent = 'Safe for Activities';
                }
            }
        }
    }

    function updateWeatherData(data) {
        const weather = data.weather;
        const flow = data.flow;

        if (weather) {
            // Air temperature
            if (elements.airTemp) {
                elements.airTemp.textContent = Math.round(weather.temperature_f || weather.temperature || 0);
            }
            if (elements.airTempValue) {
                elements.airTempValue.textContent = Math.round(weather.temperature_f || weather.temperature || 0);
            }

            // Weather description
            if (elements.weatherDesc) {
                elements.weatherDesc.textContent = weather.conditions || weather.description || '--';
            }
            if (elements.weatherValue) {
                elements.weatherValue.textContent = weather.conditions || weather.description || '--';
            }

            // Wind speed
            if (elements.windSpeed) {
                elements.windSpeed.textContent = Math.round(weather.wind_speed_mph || weather.windSpeed || 0);
            }

            // Humidity
            if (elements.humidity) {
                elements.humidity.textContent = Math.round(weather.humidity || 0);
            }

            // Weather icon
            if (elements.weatherIcon && weather.icon) {
                elements.weatherIcon.innerHTML = getWeatherIcon(weather.icon);
            }
        }

        // Water temperature (from flow data)
        if (flow && flow.water_temp_f) {
            if (elements.waterTemp) {
                elements.waterTemp.textContent = Math.round(flow.water_temp_f);
            }
            if (elements.waterTempValue) {
                elements.waterTempValue.textContent = Math.round(flow.water_temp_f);
            }
        }
    }

    function getWeatherIcon(code) {
        const icons = {
            'clear': '<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>',
            'cloudy': '<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M18 10h-1.26A8 8 0 109 20h9a5 5 0 000-10z"/></svg>',
            'rain': '<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><line x1="16" y1="13" x2="16" y2="21"/><line x1="8" y1="13" x2="8" y2="21"/><line x1="12" y1="15" x2="12" y2="23"/><path d="M20 16.58A5 5 0 0018 7h-1.26A8 8 0 104 15.25"/></svg>',
            'storm': '<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M19 16.9A5 5 0 0018 7h-1.26a8 8 0 10-11.62 9"/><polyline points="13 11 9 17 15 17 11 23"/></svg>',
            'snow': '<svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M20 17.58A5 5 0 0018 8h-1.26A8 8 0 104 16.25"/><line x1="8" y1="16" x2="8.01" y2="16"/><line x1="8" y1="20" x2="8.01" y2="20"/><line x1="12" y1="18" x2="12.01" y2="18"/><line x1="12" y1="22" x2="12.01" y2="22"/><line x1="16" y1="16" x2="16.01" y2="16"/><line x1="16" y1="20" x2="16.01" y2="20"/></svg>'
        };
        return icons[code] || icons['clear'];
    }

    function updateMoonData(data) {
        const moon = data.moon;
        if (!moon) return;

        if (elements.moonPhase) {
            elements.moonPhase.textContent = moon.phase_name || moon.phaseName || '--';
        }

        if (elements.moonIllumination) {
            elements.moonIllumination.textContent = Math.round(moon.illumination || 0);
        }

        if (elements.moonVisual) {
            const phase = moon.phase || 0;
            elements.moonVisual.innerHTML = getMoonVisual(phase);
        }

        if (elements.sunrise && moon.sunrise) {
            elements.sunrise.textContent = formatTime(moon.sunrise);
        }

        if (elements.sunset && moon.sunset) {
            elements.sunset.textContent = formatTime(moon.sunset);
        }
    }

    function getMoonVisual(phase) {
        // Create a simple moon phase visualization
        const illumination = phase < 0.5 ? phase * 2 : (1 - phase) * 2;
        const isWaxing = phase < 0.5;
        
        return `<svg width="60" height="60" viewBox="0 0 60 60" aria-hidden="true">
            <circle cx="30" cy="30" r="28" fill="#1a1a2e"/>
            <circle cx="30" cy="30" r="28" fill="#f5f5dc" 
                style="clip-path: inset(0 ${isWaxing ? Math.round((1-illumination)*100) : 0}% 0 ${isWaxing ? 0 : Math.round((1-illumination)*100)}%)"/>
        </svg>`;
    }

    function updateServicesData(data) {
        const services = data.services;
        if (!services || !elements.servicesList) return;

        const serviceItems = [
            { name: 'Tubing', key: 'tubing' },
            { name: 'Kayaking', key: 'kayaking' },
            { name: 'Canoeing', key: 'canoeing' },
            { name: 'Bike Rentals', key: 'bikes' }
        ];

        elements.servicesList.innerHTML = serviceItems.map(service => {
            const status = services[service.key];
            const isAvailable = status === true || (status && status.available);
            const badgeClass = isAvailable ? 'success' : 'danger';
            const badgeText = isAvailable ? 'Open' : 'Closed';
            
            return `
                <li class="service-status" role="listitem">
                    <span class="service-status__name">${service.name}</span>
                    <span class="service-status__badge service-status__badge--${badgeClass}">${badgeText}</span>
                </li>
            `;
        }).join('');
    }

    function updateAlerts(data) {
        const alerts = data.alerts;
        
        if (!alerts || alerts.length === 0) {
            if (elements.alertsContainer) {
                elements.alertsContainer.style.display = 'none';
            }
            return;
        }

        if (elements.alertsContainer) {
            elements.alertsContainer.style.display = 'block';
        }

        if (elements.alertsList) {
            elements.alertsList.innerHTML = alerts.map(alert => `
                <div class="alert alert--${alert.severity || 'warning'}">
                    <div class="alert__header">
                        <strong>${alert.event || 'Weather Alert'}</strong>
                        <span>${alert.expires ? `Expires: ${formatDateTime(alert.expires)}` : ''}</span>
                    </div>
                    <p class="alert__body">${alert.description || ''}</p>
                </div>
            `).join('');
        }
    }

    function updateStatusBanner(data) {
        if (!elements.statusBanner || !elements.statusText) return;

        const flow = data.flow;
        const cfs = flow ? (flow.flow_cfs || flow.current || 0) : 0;
        const status = getFlowStatus(cfs);
        
        let message = '';
        let bannerClass = '';

        if (status.class === 'danger') {
            message = 'River activities suspended due to high water levels';
            bannerClass = 'status-banner--danger';
        } else if (status.class === 'warning' && cfs > CONFIG.thresholds.high) {
            message = 'Caution: Elevated river levels. Call ahead to confirm availability.';
            bannerClass = 'status-banner--warning';
        } else if (status.class === 'warning' && cfs < CONFIG.thresholds.low) {
            message = 'Low water levels may affect some activities. Call for details.';
            bannerClass = 'status-banner--warning';
        } else {
            message = 'Conditions are great! Perfect day for river adventures.';
            bannerClass = 'status-banner--success';
        }

        elements.statusText.textContent = message;
        elements.statusBanner.className = `status-banner ${bannerClass}`;
    }

    function updateLastUpdated() {
        if (!elements.lastUpdated) return;
        elements.lastUpdated.textContent = `Last updated: ${formatDateTime(new Date())}`;
    }

    function updateError() {
        if (elements.statusText) {
            elements.statusText.textContent = 'Unable to load conditions. Please refresh or call for current status.';
        }
        if (elements.statusBanner) {
            elements.statusBanner.className = 'status-banner status-banner--warning';
        }
        if (elements.conditionsStatus) {
            const statusDot = elements.conditionsStatus.querySelector('.conditions-widget__status-dot');
            const statusText = elements.conditionsStatus.querySelector('span:last-child');
            
            if (statusDot) statusDot.style.background = 'var(--color-gray-400)';
            if (statusText) statusText.textContent = 'Unable to load';
        }
    }

    // ==========================================================================
    // Map Initialization
    // ==========================================================================
    
    function initMap() {
        if (!elements.riverMap || typeof L === 'undefined') return;

        const map = L.map('river-map').setView([43.2722, -89.7208], 13);

        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        }).addTo(map);

        // Add marker for The Launch Pad
        const launchPadIcon = L.divIcon({
            className: 'custom-marker',
            html: '<div style="background: var(--color-primary); color: white; width: 30px; height: 30px; border-radius: 50%; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.3);"><svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0118 0z"/></svg></div>',
            iconSize: [30, 30],
            iconAnchor: [15, 30]
        });

        L.marker([43.2722, -89.7208], { icon: launchPadIcon })
            .addTo(map)
<<<<<<< HEAD
            .bindPopup('<strong>The Launch Pad</strong><br>740 Water St<br>Sauk City, WI 53583');
=======
            .bindPopup('<strong>The Launch Pad</strong><br>109 Phillips Blvd<br>Sauk City, WI 53583');
>>>>>>> 69f4b5dbcdd74d012c2ebfcef975cf9cdc95e2b2

        // Add tubing route line
        const tubingRoute = [
            [43.2850, -89.7350],
            [43.2800, -89.7300],
            [43.2750, -89.7250],
            [43.2722, -89.7208],
            [43.2680, -89.7150]
        ];

        L.polyline(tubingRoute, {
            color: '#5BC0DE',
            weight: 4,
            opacity: 0.8,
            dashArray: '10, 10'
        }).addTo(map).bindPopup('Popular Tubing Route');
    }

    // ==========================================================================
    // Initialization
    // ==========================================================================
    
    async function init() {
        // Initial fetch
        const data = await fetchConditions();
        updateConditionsWidget(data);

        // Set up refresh interval
        setInterval(async () => {
            const data = await fetchConditions();
            updateConditionsWidget(data);
        }, CONFIG.refreshInterval);

        // Initialize map if on conditions page
        initMap();
    }

    // Start when DOM is ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', init);
    } else {
        init();
    }

})();
