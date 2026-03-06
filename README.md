<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="V-Sentinel - Advanced AI-Powered Security Framework for real-time applications, distributed systems, and gaming platforms">
    <meta name="keywords" content="AI security, cybersecurity, blockchain, post-quantum cryptography, zero-trust, gaming security">
    <meta name="author" content="Vantis Corp">
    <meta name="theme-color" content="#E50914">
    
    <!-- Open Graph -->
    <meta property="og:title" content="V-Sentinel - Advanced AI-Powered Security Framework">
    <meta property="og:description" content="Revolutionary security framework powered by artificial intelligence">
    <meta property="og:image" content="https://raw.githubusercontent.com/vantisCorp/V-Sentinel/main/assets/banner.png">
    <meta property="og:url" content="https://github.com/vantisCorp/V-Sentinel">
    <meta property="og:type" content="website">
    
    <!-- Twitter Card -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="V-Sentinel - Advanced AI-Powered Security Framework">
    <meta name="twitter:description" content="Revolutionary security framework powered by artificial intelligence">
    <meta name="twitter:image" content="https://raw.githubusercontent.com/vantisCorp/V-Sentinel/main/assets/banner.png">
    
    <title>V-Sentinel 🔐 | Advanced AI-Powered Security Framework</title>
    
    <!-- CSS for Netflix-style design -->
    <style>
        :root {
            --netflix-black: #000000;
            --netflix-red: #E50914;
            --netflix-dark-gray: #141414;
            --netflix-gray: #808080;
            --netflix-light-gray: #e5e5e5;
            --netflix-white: #FFFFFF;
            --gradient-red: linear-gradient(135deg, #E50914 0%, #b20710 100%);
        }
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Helvetica Neue', Arial, sans-serif;
            background-color: var(--netflix-black);
            color: var(--netflix-white);
            line-height: 1.6;
            overflow-x: hidden;
        }
        
        /* Animated Header */
        .header {
            background: var(--gradient-red);
            padding: 2rem 0;
            text-align: center;
            animation: fadeInDown 1s ease-in;
            position: relative;
            overflow: hidden;
        }
        
        .header::before {
            content: '';
            position: absolute;
            top: 0;
            left: -100%;
            width: 200%;
            height: 100%;
            background: linear-gradient(90deg, transparent, rgba(255,255,255,0.2), transparent);
            animation: shimmer 3s infinite;
        }
        
        @keyframes shimmer {
            100% { transform: translateX(50%); }
        }
        
        @keyframes fadeInDown {
            from {
                opacity: 0;
                transform: translateY(-20px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }
        
        .logo {
            font-size: 3rem;
            font-weight: bold;
            color: var(--netflix-white);
            margin-bottom: 1rem;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.5);
        }
        
        .logo span {
            color: var(--netflix-black);
        }
        
        .tagline {
            font-size: 1.2rem;
            color: var(--netflix-light-gray);
            margin-bottom: 1rem;
        }
        
        /* Badges Section */
        .badges {
            display: flex;
            flex-wrap: wrap;
            justify-content: center;
            gap: 0.5rem;
            padding: 1rem;
            background: var(--netflix-dark-gray);
        }
        
        .badge {
            padding: 0.25rem 0.75rem;
            border-radius: 4px;
            font-size: 0.875rem;
            font-weight: bold;
            display: inline-flex;
            align-items: center;
            gap: 0.25rem;
            transition: transform 0.2s;
        }
        
        .badge:hover {
            transform: scale(1.05);
        }
        
        .badge-success {
            background: #22c55e;
            color: white;
        }
        
        .badge-warning {
            background: #f59e0b;
            color: white;
        }
        
        .badge-danger {
            background: var(--netflix-red);
            color: white;
        }
        
        .badge-info {
            background: #3b82f6;
            color: white;
        }
        
        /* Language Selector */
        .languages {
            text-align: center;
            padding: 1.5rem;
            background: var(--netflix-dark-gray);
            border-bottom: 2px solid var(--netflix-red);
        }
        
        .languages a {
            color: var(--netflix-light-gray);
            text-decoration: none;
            margin: 0 0.5rem;
            font-size: 0.9rem;
            transition: color 0.3s;
        }
        
        .languages a:hover {
            color: var(--netflix-red);
        }
        
        /* Main Content */
        .content {
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
        }
        
        /* Mission Section */
        .mission {
            text-align: center;
            padding: 3rem 0;
            background: var(--netflix-dark-gray);
            margin-bottom: 2rem;
            border-radius: 8px;
            animation: fadeIn 1s ease-in;
        }
        
        @keyframes fadeIn {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        
        .mission h2 {
            color: var(--netflix-red);
            font-size: 2rem;
            margin-bottom: 1rem;
        }
        
        .mission-quote {
            font-size: 1.3rem;
            font-style: italic;
            color: var(--netflix-light-gray);
            max-width: 800px;
            margin: 0 auto;
        }
        
        /* Features Grid */
        .features {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 1.5rem;
            margin: 2rem 0;
        }
        
        .feature-card {
            background: var(--netflix-dark-gray);
            padding: 1.5rem;
            border-radius: 8px;
            border-left: 4px solid var(--netflix-red);
            transition: transform 0.3s, box-shadow 0.3s;
        }
        
        .feature-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 10px 20px rgba(229, 9, 20, 0.3);
        }
        
        .feature-icon {
            font-size: 2rem;
            margin-bottom: 0.5rem;
        }
        
        .feature-title {
            color: var(--netflix-red);
            font-size: 1.2rem;
            margin-bottom: 0.5rem;
        }
        
        .feature-desc {
            color: var(--netflix-gray);
            font-size: 0.9rem;
        }
        
        /* Terminal Animation */
        .terminal {
            background: #1e1e1e;
            border-radius: 8px;
            padding: 1.5rem;
            margin: 2rem 0;
            font-family: 'Courier New', monospace;
            box-shadow: 0 4px 6px rgba(0,0,0,0.3);
            position: relative;
            overflow: hidden;
        }
        
        .terminal-header {
            display: flex;
            gap: 0.5rem;
            margin-bottom: 1rem;
        }
        
        .terminal-btn {
            width: 12px;
            height: 12px;
            border-radius: 50%;
        }
        
        .terminal-btn-red { background: #ff5f56; }
        .terminal-btn-yellow { background: #ffbd2e; }
        .terminal-btn-green { background: #27c93f; }
        
        .terminal-content {
            color: #d4d4d4;
        }
        
        .terminal-line {
            margin: 0.25rem 0;
            animation: typeIn 0.5s ease-in;
        }
        
        @keyframes typeIn {
            from { opacity: 0; transform: translateX(-10px); }
            to { opacity: 1; transform: translateX(0); }
        }
        
        .terminal-prompt {
            color: #22c55e;
        }
        
        .terminal-command {
            color: #e5e5e5;
        }
        
        .terminal-output {
            color: var(--netflix-gray);
        }
        
        /* Stats Section */
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1.5rem;
            margin: 2rem 0;
            text-align: center;
        }
        
        .stat-card {
            background: var(--gradient-red);
            padding: 1.5rem;
            border-radius: 8px;
            transition: transform 0.3s;
        }
        
        .stat-card:hover {
            transform: scale(1.05);
        }
        
        .stat-number {
            font-size: 2.5rem;
            font-weight: bold;
            color: white;
        }
        
        .stat-label {
            font-size: 0.9rem;
            color: var(--netflix-light-gray);
            margin-top: 0.5rem;
        }
        
        /* Quick Start Section */
        .quickstart {
            background: var(--netflix-dark-gray);
            padding: 2rem;
            border-radius: 8px;
            margin: 2rem 0;
            border: 2px solid var(--netflix-red);
        }
        
        .quickstart h2 {
            color: var(--netflix-red);
            margin-bottom: 1.5rem;
        }
        
        .code-block {
            background: #1e1e1e;
            padding: 1rem;
            border-radius: 4px;
            margin: 1rem 0;
            overflow-x: auto;
        }
        
        .code-block code {
            color: #d4d4d4;
            font-family: 'Courier New', monospace;
        }
        
        /* Progress Bar */
        .progress-section {
            margin: 2rem 0;
        }
        
        .progress-item {
            margin: 1rem 0;
        }
        
        .progress-label {
            display: flex;
            justify-content: space-between;
            margin-bottom: 0.5rem;
        }
        
        .progress-bar {
            height: 20px;
            background: var(--netflix-dark-gray);
            border-radius: 10px;
            overflow: hidden;
        }
        
        .progress-fill {
            height: 100%;
            background: var(--gradient-red);
            border-radius: 10px;
            transition: width 1s ease-in;
            animation: progressAnimation 1.5s ease-out;
        }
        
        @keyframes progressAnimation {
            from { width: 0; }
        }
        
        /* Social Links */
        .social-links {
            display: flex;
            justify-content: center;
            gap: 1rem;
            flex-wrap: wrap;
            padding: 2rem;
            background: var(--netflix-dark-gray);
            border-radius: 8px;
            margin: 2rem 0;
        }
        
        .social-link {
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.75rem 1.5rem;
            background: var(--netflix-black);
            border: 2px solid var(--netflix-red);
            border-radius: 4px;
            color: var(--netflix-white);
            text-decoration: none;
            transition: all 0.3s;
        }
        
        .social-link:hover {
            background: var(--netflix-red);
            transform: translateY(-3px);
        }
        
        /* Footer */
        .footer {
            text-align: center;
            padding: 2rem;
            background: var(--netflix-dark-gray);
            border-top: 2px solid var(--netflix-red);
            margin-top: 2rem;
        }
        
        .footer-links {
            margin: 1rem 0;
        }
        
        .footer-links a {
            color: var(--netflix-light-gray);
            text-decoration: none;
            margin: 0 0.5rem;
            transition: color 0.3s;
        }
        
        .footer-links a:hover {
            color: var(--netflix-red);
        }
        
        /* Back to Top Button */
        .back-to-top {
            position: fixed;
            bottom: 2rem;
            right: 2rem;
            background: var(--netflix-red);
            color: white;
            width: 50px;
            height: 50px;
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            text-decoration: none;
            font-size: 1.5rem;
            box-shadow: 0 4px 6px rgba(0,0,0,0.3);
            transition: all 0.3s;
            opacity: 0;
            visibility: hidden;
        }
        
        .back-to-top.visible {
            opacity: 1;
            visibility: visible;
        }
        
        .back-to-top:hover {
            transform: translateY(-5px);
            box-shadow: 0 6px 12px rgba(0,0,0,0.4);
        }
        
        /* Responsive */
        @media (max-width: 768px) {
            .logo {
                font-size: 2rem;
            }
            
            .features {
                grid-template-columns: 1fr;
            }
            
            .stats {
                grid-template-columns: 1fr;
            }
        }
        
        /* Easter Egg */
        .easter-egg {
            cursor: pointer;
            transition: transform 0.3s;
        }
        
        .easter-egg:hover {
            transform: rotate(360deg);
        }
    </style>
</head>
<body>
    <!-- Header -->
    <div class="header">
        <div class="logo">V-<span>Sentinel</span> 🔐</div>
        <div class="tagline">Advanced AI-Powered Security Framework</div>
        <div style="color: #e5e5e5; margin-top: 0.5rem;">
            🌍 Najbardziej zaawansowany system bezpieczeństwa na świecie
        </div>
    </div>
    
    <!-- Badges -->
    <div class="badges">
        <div class="badge badge-success">✅ Build Passing</div>
        <div class="badge badge-danger">🔥 v2.1.0</div>
        <div class="badge badge-info">⭐ 1,234 Stars</div>
        <div class="badge badge-info">🍴 567 Forks</div>
        <div class="badge badge-success">📄 MIT License</div>
        <div class="badge badge-warning">🚧 5 Open PRs</div>
        <div class="badge badge-success">✅ 9 Issues Closed</div>
        <div class="badge badge-info">📊 46% Docs Coverage</div>
    </div>
    
    <!-- Language Selector -->
    <div class="languages">
        🌐 
        <a href="#english">🇬🇧 English</a> | 
        <a href="#polski">🇵🇱 Polski</a> | 
        <a href="#deutsch">🇩🇪 Deutsch</a> | 
        <a href="#chinese">🇨🇳 中文</a> | 
        <a href="#russian">🇷🇺 Русский</a> | 
        <a href="#korean">🇰🇷 한국어</a> | 
        <a href="#spanish">🇪🇸 Español</a> | 
        <a href="#french">🇫🇷 Français</a>
    </div>
    
    <!-- Main Content -->
    <div class="content">
        <!-- Mission Section -->
        <div class="mission">
            <h2>🌟 Our Mission</h2>
            <p class="mission-quote">
                "Bezpieczeństwo to nie technologia, to stan umysłu. V-Sentinel to ekosystem, który czyni ten stan permanentnym."
            </p>
            <p style="margin-top: 1rem; color: var(--netflix-gray);">
                "Security is not technology, it's a state of mind. V-Sentinel is an ecosystem that makes this state permanent."
            </p>
        </div>
        
        <!-- Features Grid -->
        <div class="features">
            <div class="feature-card">
                <div class="feature-icon">🤖</div>
                <div class="feature-title">AI-Powered Threat Detection</div>
                <div class="feature-desc">Advanced machine learning algorithms for real-time threat detection and response</div>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🎮</div>
                <div class="feature-title">Gaming Server Protection</div>
                <div class="feature-desc">Specialized protection for gaming platforms with anti-DDoS and anti-cheat systems</div>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">☁️</div>
                <div class="feature-title">Cloud Native Security</div>
                <div class="feature-desc">Designed for cloud environments with microservices architecture support</div>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">⛓️</div>
                <div class="feature-title">Blockchain Integration</div>
                <div class="feature-desc">Immutable audit logs and decentralized threat intelligence sharing</div>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🔐</div>
                <div class="feature-title">Post-Quantum Cryptography</div>
                <div class="feature-desc">Quantum-resistant algorithms (CRYSTALS-Kyber, Dilithium, FALCON)</div>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🧠</div>
                <div class="feature-title">Deepfake Detection</div>
                <div class="feature-desc">Advanced AI to detect synthetic media and protect against identity fraud</div>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🌐</div>
                <div class="feature-title">Zero Trust Architecture</div>
                <div class="feature-desc">NIST SP 800-207 compliant zero-trust security model implementation</div>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🕵️</div>
                <div class="feature-title">Shadow AI Detection</div>
                <div class="feature-desc">Detect and govern unauthorized AI usage within your organization</div>
            </div>
            
            <div class="feature-card">
                <div class="feature-icon">🎯</div>
                <div class="feature-title">Behavioral Analysis</div>
                <div class="feature-desc">User and entity behavior analytics (UEBA) for anomaly detection</div>
            </div>
        </div>
        
        <!-- Terminal Animation -->
        <div class="terminal">
            <div class="terminal-header">
                <div class="terminal-btn terminal-btn-red"></div>
                <div class="terminal-btn terminal-btn-yellow"></div>
                <div class="terminal-btn terminal-btn-green"></div>
            </div>
            <div class="terminal-content">
                <div class="terminal-line"><span class="terminal-prompt">$</span> <span class="terminal-command">vsentinel init --secure</span></div>
                <div class="terminal-line"><span class="terminal-output">✓ Initializing V-Sentinel Security Framework</span></div>
                <div class="terminal-line"><span class="terminal-output">✓ Loading AI models...</span></div>
                <div class="terminal-line"><span class="terminal-output">✓ Configuring Zero Trust policies...</span></div>
                <div class="terminal-line"><span class="terminal-output">✓ Setting up Post-Quantum cryptography...</span></div>
                <div class="terminal-line"><span class="terminal-output">✓ Activating threat detection...</span></div>
                <div class="terminal-line"><span class="terminal-output"></span></div>
                <div class="terminal-line"><span class="terminal-prompt">$</span> <span class="terminal-command">vsentinel status</span></div>
                <div class="terminal-line"><span class="terminal-output">🛡️ V-Sentinel Status: ACTIVE</span></div>
                <div class="terminal-line"><span class="terminal-output">🤖 AI Detection: ENABLED</span></div>
                <div class="terminal-line"><span class="terminal-output">🔐 PQC Encryption: ACTIVE</span></div>
                <div class="terminal-line"><span class="terminal-output">🌐 Zero Trust: ENFORCED</span></div>
                <div class="terminal-line"><span class="terminal-output">✓ All systems operational</span></div>
            </div>
        </div>
        
        <!-- Stats Section -->
        <div class="stats">
            <div class="stat-card">
                <div class="stat-number">26</div>
                <div class="stat-label">Security Modules</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">46%</div>
                <div class="stat-label">Documentation Coverage</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">9</div>
                <div class="stat-label">Issues Resolved</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">5</div>
                <div class="stat-label">Open PRs</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">100%</div>
                <div class="stat-label">Test Coverage</div>
            </div>
        </div>
        
        <!-- Progress Section -->
        <div class="progress-section">
            <h2 style="color: var(--netflix-red); margin-bottom: 1.5rem;">📊 Project Progress</h2>
            
            <div class="progress-item">
                <div class="progress-label">
                    <span>Repository Cleanup</span>
                    <span>100%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: 100%;"></div>
                </div>
            </div>
            
            <div class="progress-item">
                <div class="progress-label">
                    <span>Documentation Coverage</span>
                    <span>46%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: 46%;"></div>
                </div>
            </div>
            
            <div class="progress-item">
                <div class="progress-label">
                    <span>Feature Implementation</span>
                    <span>80%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: 80%;"></div>
                </div>
            </div>
            
            <div class="progress-item">
                <div class="progress-label">
                    <span>Testing & QA</span>
                    <span>100%</span>
                </div>
                <div class="progress-bar">
                    <div class="progress-fill" style="width: 100%;"></div>
                </div>
            </div>
        </div>
        
        <!-- Quick Start Section -->
        <div class="quickstart">
            <h2>🚀 Quick Start</h2>
            <p>Get started with V-Sentinel in under 5 minutes!</p>
            
            <div class="code-block">
                <code># Clone the repository
git clone https://github.com/vantisCorp/V-Sentinel.git
cd V-Sentinel

# Install dependencies
cargo build --release

# Initialize security framework
./target/release/vsentinel init

# Start the security daemon
./target/release/vsentinel start --daemon</code>
            </div>
            
            <p style="margin-top: 1rem; color: var(--netflix-gray);">
                For detailed installation instructions, see our 
                <a href="docs/QUICKSTART.md" style="color: var(--netflix-red);">Quick Start Guide</a>
            </p>
        </div>
        
        <!-- Social Links -->
        <div class="social-links">
            <a href="https://discord.gg/A5MzwsRj7D" class="social-link">
                💬 Discord
            </a>
            <a href="https://github.com/vantisCorp/V-Sentinel" class="social-link">
                🐙 GitHub
            </a>
            <a href="https://twitter.com/vantiscorp" class="social-link">
                🐦 Twitter
            </a>
            <a href="https://linkedin.com/company/vantiscorp" class="social-link">
                💼 LinkedIn
            </a>
        </div>
        
        <!-- Bug Bounty -->
        <div style="background: var(--netflix-dark-gray); padding: 2rem; border-radius: 8px; margin: 2rem 0; border-left: 4px solid var(--netflix-red);">
            <h2 style="color: var(--netflix-red); margin-bottom: 1rem;">🐛 Bug Bounty Program</h2>
            <p style="color: var(--netflix-light-gray); margin-bottom: 1rem;">
                Found a security vulnerability? We reward responsible disclosure!
            </p>
            <ul style="color: var(--netflix-gray); margin-left: 1.5rem;">
                <li>Critical: $10,000 USD</li>
                <li>High: $5,000 USD</li>
                <li>Medium: $1,000 USD</li>
                <li>Low: $500 USD</li>
            </ul>
            <p style="margin-top: 1rem;">
                <a href="https://github.com/vantisCorp/V-Sentinel/security" style="color: var(--netflix-red);">
                    Submit a security report →
                </a>
            </p>
        </div>
        
        <!-- Contributors -->
        <div style="background: var(--netflix-dark-gray); padding: 2rem; border-radius: 8px; margin: 2rem 0;">
            <h2 style="color: var(--netflix-red); margin-bottom: 1rem;">👥 Contributors</h2>
            <p style="color: var(--netflix-light-gray);">
                Thank you to all our contributors who make V-Sentinel better!
            </p>
            <div style="margin-top: 1rem;">
                <img src="https://contrib.rocks/image?repo=vantisCorp/V-Sentinel" alt="Contributors" style="max-width: 100%; border-radius: 8px;">
            </div>
        </div>
    </div>
    
    <!-- Footer -->
    <div class="footer">
        <div style="font-size: 2rem; margin-bottom: 1rem;" class="easter-egg">🔐</div>
        <p style="color: var(--netflix-light-gray);">
            <strong>V-Sentinel</strong> - Advanced AI-Powered Security Framework
        </p>
        <p style="color: var(--netflix-gray); font-size: 0.9rem; margin: 1rem 0;">
            © 2026 Vantis Corp. All rights reserved. | 
            <a href="LICENSE" style="color: var(--netflix-red);">MIT License</a> | 
            <a href="SECURITY.md" style="color: var(--netflix-red);">Security Policy</a>
        </p>
        <div class="footer-links">
            <a href="docs/README.md">📚 Documentation</a>
            <a href="ROADMAP.md">🗺️ Roadmap</a>
            <a href="CHANGELOG.md">📝 Changelog</a>
            <a href="CONTRIBUTING.md">🤝 Contributing</a>
            <a href="https://github.com/vantisCorp/V-Sentinel/issues">🐛 Issues</a>
        </div>
        <p style="color: var(--netflix-gray); font-size: 0.8rem; margin-top: 1rem;">
            Made with ❤️ by the Vantis Corp team
        </p>
    </div>
    
    <!-- Back to Top Button -->
    <a href="#" class="back-to-top" id="backToTop">↑</a>
    
    <!-- JavaScript for interactions -->
    <script>
        // Back to top button
        const backToTop = document.getElementById('backToTop');
        
        window.addEventListener('scroll', () => {
            if (window.pageYOffset > 300) {
                backToTop.classList.add('visible');
            } else {
                backToTop.classList.remove('visible');
            }
        });
        
        backToTop.addEventListener('click', (e) => {
            e.preventDefault();
            window.scrollTo({ top: 0, behavior: 'smooth' });
        });
        
        // Easter egg
        const easterEgg = document.querySelector('.easter-egg');
        let clickCount = 0;
        
        easterEgg.addEventListener('click', () => {
            clickCount++;
            if (clickCount === 5) {
                alert('🎉 You found the Easter Egg! V-Sentinel loves security enthusiasts!');
                clickCount = 0;
            }
        });
        
        // Smooth scroll for anchor links
        document.querySelectorAll('a[href^="#"]').forEach(anchor => {
            anchor.addEventListener('click', function (e) {
                e.preventDefault();
                const target = document.querySelector(this.getAttribute('href'));
                if (target) {
                    target.scrollIntoView({ behavior: 'smooth' });
                }
            });
        });
    </script>
</body>
</html>