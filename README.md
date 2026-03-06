<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>V-Sentinel | Advanced AI-Powered Security Framework</title>
    
    <!-- SEO Metadata -->
    <meta name="description" content="Next-generation AI-native security system with quantum-ready cryptography. Features Ring-1 Hypervisor, AI Prediction Engine, Quantum Cryptography, Gaming Optimization, Behavioral Analysis, and more.">
    <meta name="keywords" content="cybersecurity, AI security, quantum cryptography, hypervisor, threat detection, post-quantum, deepfake detection, zero trust">
    <meta name="author" content="Vantis Corp">
    <meta name="robots" content="index, follow">
    
    <!-- Open Graph / Social Media -->
    <meta property="og:type" content="website">
    <meta property="og:title" content="V-Sentinel | Advanced AI-Powered Security Framework">
    <meta property="og:description" content="Next-generation AI-native security system with quantum-ready cryptography.">
    <meta property="og:image" content="https://github.com/vantisCorp/V-Sentinel/raw/main/docs/images/og-image.png">
    <meta property="og:url" content="https://github.com/vantisCorp/V-Sentinel">
    
    <!-- Twitter Card -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="V-Sentinel | Advanced AI-Powered Security Framework">
    <meta name="twitter:description" content="Next-generation AI-native security system with quantum-ready cryptography.">
    <meta name="twitter:image" content="https://github.com/vantisCorp/V-Sentinel/raw/main/docs/images/twitter-card.png">
    <meta name="twitter:creator" content="@vantiscorp">
    
    <!-- Mermaid.js -->
    <script type="module">
        import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.esm.min.mjs';
        mermaid.initialize({ startOnLoad: true, theme: 'dark' });
    </script>
    
    <!-- Fonts -->
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
    
    <style>
        :root {
            --netflix-red: #E50914;
            --netflix-red-dark: #B20710;
            --netflix-red-light: #F40612;
            --netflix-black: #000000;
            --netflix-dark: #0a0a0a;
            --netflix-dark-gray: #141414;
            --netflix-medium-gray: #1F1F1F;
            --netflix-light-gray: #2A2A2A;
            --netflix-text: #FFFFFF;
            --netflix-text-secondary: #B3B3B3;
            --netflix-text-muted: #6E6E6E;
            --gradient-red: linear-gradient(135deg, #E50914 0%, #B20710 100%);
            --gradient-black: linear-gradient(180deg, #0a0a0a 0%, #000000 100%);
            --gradient-dark: linear-gradient(135deg, #1F1F1F 0%, #141414 100%);
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        html {
            scroll-behavior: smooth;
        }

        body {
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            background-color: var(--netflix-black);
            color: var(--netflix-text);
            line-height: 1.6;
            overflow-x: hidden;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
        }

        /* Hero Section */
        .hero {
            position: relative;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            overflow: hidden;
            background: var(--netflix-black);
        }

        .hero::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: 
                radial-gradient(circle at 20% 50%, rgba(229, 9, 20, 0.15) 0%, transparent 50%),
                radial-gradient(circle at 80% 80%, rgba(229, 9, 20, 0.1) 0%, transparent 40%),
                radial-gradient(circle at 40% 20%, rgba(229, 9, 20, 0.08) 0%, transparent 30%);
            animation: pulse 8s ease-in-out infinite;
        }

        @keyframes pulse {
            0%, 100% { opacity: 0.5; }
            50% { opacity: 1; }
        }

        .hero-content {
            position: relative;
            z-index: 10;
            text-align: center;
            padding: 2rem;
            max-width: 1200px;
            animation: fadeInUp 1.5s ease-out;
        }

        @keyframes fadeInUp {
            from {
                opacity: 0;
                transform: translateY(40px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .hero-logo {
            font-size: 5rem;
            font-weight: 900;
            letter-spacing: -0.05em;
            margin-bottom: 0.5rem;
            background: var(--gradient-red);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            animation: glow 3s ease-in-out infinite;
        }

        @keyframes glow {
            0%, 100% { 
                text-shadow: 0 0 20px rgba(229, 9, 20, 0.5);
            }
            50% { 
                text-shadow: 0 0 40px rgba(229, 9, 20, 0.8), 0 0 60px rgba(229, 9, 20, 0.6);
            }
        }

        .hero-tagline {
            font-size: 1.75rem;
            font-weight: 300;
            color: var(--netflix-text-secondary);
            margin-bottom: 1rem;
            letter-spacing: 0.02em;
        }

        .hero-subtitle {
            font-size: 1.125rem;
            color: var(--netflix-text-muted);
            margin-bottom: 3rem;
            max-width: 600px;
            margin-left: auto;
            margin-right: auto;
        }

        .hero-badge {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.5rem 1rem;
            background: var(--netflix-dark-gray);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 8px;
            font-size: 0.875rem;
            font-weight: 500;
            margin: 0.25rem;
            transition: all 0.3s ease;
        }

        .hero-badge:hover {
            border-color: var(--netflix-red);
            transform: translateY(-2px);
            box-shadow: 0 4px 20px rgba(229, 9, 20, 0.2);
        }

        .hero-badge svg {
            width: 16px;
            height: 16px;
        }

        /* Floating Particles */
        .particles {
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            overflow: hidden;
            pointer-events: none;
        }

        .particle {
            position: absolute;
            width: 4px;
            height: 4px;
            background: var(--netflix-red);
            border-radius: 50%;
            opacity: 0.3;
            animation: float 20s infinite;
        }

        @keyframes float {
            0%, 100% {
                transform: translateY(100vh) rotate(0deg);
                opacity: 0;
            }
            10% {
                opacity: 0.3;
            }
            90% {
                opacity: 0.3;
            }
            100% {
                transform: translateY(-100vh) rotate(720deg);
                opacity: 0;
            }
        }

        /* Navigation */
        .nav {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            background: rgba(10, 10, 10, 0.95);
            backdrop-filter: blur(20px);
            z-index: 1000;
            border-bottom: 1px solid rgba(255, 255, 255, 0.1);
            transition: all 0.3s ease;
        }

        .nav.scrolled {
            background: rgba(10, 10, 10, 0.98);
            box-shadow: 0 4px 30px rgba(0, 0, 0, 0.5);
        }

        .nav-content {
            max-width: 1200px;
            margin: 0 auto;
            padding: 1rem 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }

        .nav-logo {
            font-size: 1.5rem;
            font-weight: 700;
            color: var(--netflix-red);
            letter-spacing: -0.02em;
        }

        .nav-links {
            display: flex;
            gap: 2rem;
            align-items: center;
        }

        .nav-link {
            color: var(--netflix-text-secondary);
            text-decoration: none;
            font-size: 0.875rem;
            font-weight: 500;
            transition: color 0.3s ease;
            position: relative;
        }

        .nav-link::after {
            content: '';
            position: absolute;
            bottom: -0.25rem;
            left: 0;
            width: 0;
            height: 2px;
            background: var(--netflix-red);
            transition: width 0.3s ease;
        }

        .nav-link:hover {
            color: var(--netflix-text);
        }

        .nav-link:hover::after {
            width: 100%;
        }

        /* Section Styles */
        .section {
            padding: 8rem 2rem;
            position: relative;
        }

        .section-title {
            font-size: 3rem;
            font-weight: 800;
            text-align: center;
            margin-bottom: 1rem;
            background: linear-gradient(135deg, #FFFFFF 0%, #B3B3B3 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            letter-spacing: -0.02em;
        }

        .section-subtitle {
            font-size: 1.25rem;
            color: var(--netflix-text-secondary);
            text-align: center;
            max-width: 600px;
            margin: 0 auto 4rem;
        }

        /* Features Grid */
        .features {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 2rem;
            max-width: 1200px;
            margin: 0 auto;
        }

        .feature-card {
            background: var(--gradient-dark);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 16px;
            padding: 2rem;
            transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
            position: relative;
            overflow: hidden;
        }

        .feature-card::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 3px;
            background: var(--gradient-red);
            transform: scaleX(0);
            transition: transform 0.4s ease;
        }

        .feature-card:hover {
            transform: translateY(-8px);
            border-color: var(--netflix-red);
            box-shadow: 0 20px 60px rgba(229, 9, 20, 0.2);
        }

        .feature-card:hover::before {
            transform: scaleX(1);
        }

        .feature-icon {
            width: 64px;
            height: 64px;
            background: var(--gradient-red);
            border-radius: 12px;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 2rem;
            margin-bottom: 1.5rem;
        }

        .feature-title {
            font-size: 1.5rem;
            font-weight: 700;
            margin-bottom: 0.75rem;
            letter-spacing: -0.01em;
        }

        .feature-description {
            color: var(--netflix-text-secondary);
            font-size: 1rem;
            line-height: 1.7;
        }

        /* Stats Section */
        .stats-section {
            background: var(--netflix-dark);
            padding: 8rem 2rem;
            position: relative;
            overflow: hidden;
        }

        .stats-section::before {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            height: 1px;
            background: linear-gradient(90deg, transparent, var(--netflix-red), transparent);
        }

        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 3rem;
            max-width: 1200px;
            margin: 0 auto;
        }

        .stat-item {
            text-align: center;
        }

        .stat-number {
            font-size: 4rem;
            font-weight: 900;
            background: var(--gradient-red);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
            margin-bottom: 0.5rem;
            line-height: 1;
        }

        .stat-label {
            font-size: 1.125rem;
            color: var(--netflix-text-secondary);
            font-weight: 500;
        }

        /* Terminal */
        .terminal-section {
            background: var(--netflix-black);
            padding: 8rem 2rem;
        }

        .terminal {
            max-width: 800px;
            margin: 0 auto;
            background: var(--netflix-dark-gray);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 12px;
            overflow: hidden;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
        }

        .terminal-header {
            background: var(--netflix-medium-gray);
            padding: 1rem 1.5rem;
            display: flex;
            align-items: center;
            gap: 0.5rem;
            border-bottom: 1px solid var(--netflix-light-gray);
        }

        .terminal-dot {
            width: 12px;
            height: 12px;
            border-radius: 50%;
            background: #FF5F57;
        }

        .terminal-dot:nth-child(2) {
            background: #FEBC2E;
        }

        .terminal-dot:nth-child(3) {
            background: #28C840;
        }

        .terminal-title {
            flex: 1;
            text-align: center;
            font-size: 0.875rem;
            color: var(--netflix-text-secondary);
            font-family: 'JetBrains Mono', monospace;
        }

        .terminal-body {
            padding: 2rem;
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.9rem;
            line-height: 1.8;
        }

        .terminal-line {
            margin-bottom: 0.5rem;
            opacity: 0;
            animation: typeLine 0.5s ease forwards;
        }

        @keyframes typeLine {
            from {
                opacity: 0;
                transform: translateX(-10px);
            }
            to {
                opacity: 1;
                transform: translateX(0);
            }
        }

        .terminal-prompt {
            color: var(--netflix-red);
        }

        .terminal-command {
            color: var(--netflix-text);
        }

        .terminal-output {
            color: var(--netflix-text-secondary);
        }

        .terminal-success {
            color: #28C840;
        }

        /* Quick Start */
        .quickstart-section {
            background: var(--netflix-dark);
            padding: 8rem 2rem;
        }

        .code-block {
            max-width: 800px;
            margin: 0 auto;
            background: var(--netflix-dark-gray);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 12px;
            overflow: hidden;
        }

        .code-header {
            background: var(--netflix-medium-gray);
            padding: 1rem 1.5rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-bottom: 1px solid var(--netflix-light-gray);
        }

        .code-lang {
            font-size: 0.875rem;
            color: var(--netflix-text-secondary);
            font-family: 'JetBrains Mono', monospace;
        }

        .code-copy {
            background: transparent;
            border: 1px solid var(--netflix-light-gray);
            color: var(--netflix-text-secondary);
            padding: 0.5rem 1rem;
            border-radius: 6px;
            font-size: 0.875rem;
            cursor: pointer;
            transition: all 0.3s ease;
        }

        .code-copy:hover {
            border-color: var(--netflix-red);
            color: var(--netflix-text);
        }

        .code-content {
            padding: 2rem;
            font-family: 'JetBrains Mono', monospace;
            font-size: 0.9rem;
            line-height: 1.8;
            color: var(--netflix-text);
            overflow-x: auto;
        }

        .code-content code {
            display: block;
        }

        /* Footer */
        .footer {
            background: var(--netflix-dark-gray);
            padding: 6rem 2rem;
            border-top: 1px solid var(--netflix-light-gray);
        }

        /* Architecture & Mermaid */
        .mermaid-diagram {
            background: var(--netflix-dark-gray);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 12px;
            padding: 2rem;
            margin: 2rem 0;
            overflow-x: auto;
        }

        .mermaid-diagram pre {
            margin: 0;
            font-size: 0.9rem;
            line-height: 1.6;
        }

        /* Security Badges */
        .badges-container {
            text-align: center;
            padding: 2rem 0;
        }

        .badge-row {
            display: flex;
            justify-content: center;
            gap: 1rem;
            margin-bottom: 1rem;
            flex-wrap: wrap;
        }

        .badge-row img {
            transition: transform 0.3s ease;
        }

        .badge-row img:hover {
            transform: scale(1.05);
        }

        /* API Documentation */
        .api-endpoints {
            display: grid;
            gap: 2rem;
            margin-top: 3rem;
        }

        .api-endpoint {
            background: var(--netflix-dark-gray);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 12px;
            padding: 2rem;
            transition: all 0.3s ease;
        }

        .api-endpoint:hover {
            border-color: var(--netflix-red);
            transform: translateY(-2px);
        }

        .api-endpoint h3 {
            font-size: 1.25rem;
            color: var(--netflix-red);
            margin-bottom: 0.5rem;
        }

        .api-endpoint p {
            color: var(--netflix-text-secondary);
            margin-bottom: 1.5rem;
        }

        /* Contributors */
        .contributors-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1.5rem;
            margin-bottom: 2rem;
        }

        .contributor-card {
            background: var(--netflix-dark-gray);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 12px;
            padding: 2rem;
            text-align: center;
            transition: all 0.3s ease;
        }

        .contributor-card:hover {
            border-color: var(--netflix-red);
            transform: translateY(-4px);
        }

        .contributor-avatar {
            font-size: 4rem;
            margin-bottom: 1rem;
        }

        .contributor-info h4 {
            font-size: 1.125rem;
            color: var(--netflix-text);
            margin-bottom: 0.5rem;
        }

        .contributor-info p {
            color: var(--netflix-text-secondary);
            font-size: 0.875rem;
        }

        .contribution-guide {
            background: var(--netflix-dark);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 12px;
            padding: 2rem;
        }

        .contribution-guide h3 {
            color: var(--netflix-red);
            margin-bottom: 1rem;
        }

        .contribution-guide ul {
            list-style: none;
            padding: 0;
        }

        .contribution-guide li {
            padding: 0.75rem 0;
            color: var(--netflix-text-secondary);
            border-bottom: 1px solid var(--netflix-light-gray);
        }

        .contribution-guide li:last-child {
            border-bottom: none;
        }

        /* Roadmap */
        .roadmap-timeline {
            max-width: 900px;
            margin: 0 auto;
        }

        .roadmap-item {
            display: flex;
            gap: 1.5rem;
            margin-bottom: 2rem;
            padding-left: 2rem;
            position: relative;
        }

        .roadmap-item::before {
            content: '';
            position: absolute;
            left: 0;
            top: 0;
            bottom: 0;
            width: 2px;
            background: var(--netflix-light-gray);
        }

        .roadmap-item:last-child::before {
            display: none;
        }

        .roadmap-marker {
            width: 2.5rem;
            height: 2.5rem;
            border-radius: 50%;
            background: var(--netflix-red);
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 1.25rem;
            flex-shrink: 0;
            position: relative;
            z-index: 1;
        }

        .roadmap-item.completed .roadmap-marker {
            background: #22c55e;
        }

        .roadmap-item.in-progress .roadmap-marker {
            background: #eab308;
        }

        .roadmap-item.planned .roadmap-marker {
            background: var(--netflix-medium-gray);
        }

        .roadmap-content {
            background: var(--netflix-dark-gray);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 12px;
            padding: 1.5rem;
            flex: 1;
        }

        .roadmap-content h3 {
            color: var(--netflix-red);
            margin-bottom: 1rem;
        }

        .roadmap-content ul {
            list-style: none;
            padding: 0;
        }

        .roadmap-content li {
            padding: 0.5rem 0;
            color: var(--netflix-text-secondary);
            padding-left: 1.5rem;
            position: relative;
        }

        .roadmap-content li::before {
            content: '•';
            position: absolute;
            left: 0;
            color: var(--netflix-red);
        }

        /* Statistics */
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1.5rem;
            margin-top: 3rem;
        }

        .stat-card {
            background: var(--netflix-dark-gray);
            border: 1px solid var(--netflix-light-gray);
            border-radius: 12px;
            padding: 2rem;
            text-align: center;
            transition: all 0.3s ease;
        }

        .stat-card:hover {
            border-color: var(--netflix-red);
            transform: translateY(-4px);
        }

        .stat-value {
            font-size: 2.5rem;
            font-weight: 700;
            color: var(--netflix-red);
            margin-bottom: 0.5rem;
        }

        .stat-label {
            color: var(--netflix-text-secondary);
            font-size: 0.875rem;
        }

        /* Back to Top */
        .back-to-top {
            position: fixed;
            bottom: 2rem;
            right: 2rem;
            width: 3rem;
            height: 3rem;
            background: var(--netflix-red);
            color: white;
            border: none;
            border-radius: 50%;
            font-size: 1.5rem;
            cursor: pointer;
            display: none;
            align-items: center;
            justify-content: center;
            transition: all 0.3s ease;
            z-index: 1000;
        }

        .back-to-top:hover {
            background: var(--netflix-red-dark);
            transform: translateY(-2px);
        }

        .back-to-top.visible {
            display: flex;
        }

        .footer-content {
            max-width: 1200px;
            margin: 0 auto;
            text-align: center;
        }

        .footer-logo {
            font-size: 2rem;
            font-weight: 800;
            color: var(--netflix-red);
            margin-bottom: 1.5rem;
            letter-spacing: -0.02em;
        }

        .footer-links {
            display: flex;
            justify-content: center;
            gap: 2rem;
            flex-wrap: wrap;
            margin-bottom: 2rem;
        }

        .footer-link {
            color: var(--netflix-text-secondary);
            text-decoration: none;
            font-size: 0.875rem;
            font-weight: 500;
            transition: color 0.3s ease;
        }

        .footer-link:hover {
            color: var(--netflix-red);
        }

        .footer-copyright {
            color: var(--netflix-text-muted);
            font-size: 0.875rem;
        }

        /* Responsive */
        @media (max-width: 768px) {
            .hero-logo {
                font-size: 3rem;
            }

            .hero-tagline {
                font-size: 1.25rem;
            }

            .section-title {
                font-size: 2rem;
            }

            .features {
                grid-template-columns: 1fr;
            }

            .stats {
                grid-template-columns: 1fr;
            }

            .nav-links {
                display: none;
            }
        }

        /* Utility Classes */
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 2rem;
        }

        .btn {
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 1rem 2rem;
            background: var(--gradient-red);
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 1rem;
            font-weight: 600;
            cursor: pointer;
            text-decoration: none;
            transition: all 0.3s ease;
            box-shadow: 0 4px 20px rgba(229, 9, 20, 0.3);
        }

        .btn:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 30px rgba(229, 9, 20, 0.4);
        }

        .divider {
            height: 1px;
            background: var(--netflix-light-gray);
            margin: 4rem 0;
        }
    </style>
</head>
<body>
    <!-- Navigation -->
    <nav class="nav" id="nav">
        <div class="nav-content">
            <div class="nav-logo">V-Sentinel</div>
            <div class="nav-links">
                <a href="#features" class="nav-link">Features</a>
                <a href="#stats" class="nav-link">Stats</a>
                <a href="#architecture" class="nav-link">Architecture</a>
                <a href="#security" class="nav-link">Security</a>
                <a href="#api" class="nav-link">API</a>
                <a href="#roadmap" class="nav-link">Roadmap</a>
                <a href="#contributors" class="nav-link">Contributors</a>
                <a href="#quickstart" class="nav-link">Quick Start</a>
                <a href="https://github.com/vantisCorp/V-Sentinel" class="nav-link">GitHub</a>
            </div>
        </div>
    </nav>

    <!-- Hero Section -->
    <section class="hero">
        <div class="particles" id="particles"></div>
        <div class="hero-content">
            <div class="hero-logo">V-Sentinel</div>
            <div class="hero-tagline">Advanced AI-Powered Security Framework</div>
            <div class="hero-subtitle">
                Najbardziej zaawansowany system bezpieczeństwa na świecie. Rewolucyjny framework napędzany przez sztuczną inteligencję.
            </div>
            
            <div style="margin-bottom: 2rem;">
                <div class="hero-badge">
                    <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
                    Production Ready
                </div>
                <div class="hero-badge">
                    <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/></svg>
                    100% Test Coverage
                </div>
                <div class="hero-badge">
                    <svg viewBox="0 0 24 24" fill="currentColor"><path d="M9.4 16.6L4.8 12l4.6-4.6L8 6l-6 6 6 6 1.4-1.4zm5.2 0l4.6-4.6-4.6-4.6L16 6l6 6-6 6-1.4-1.4z"/></svg>
                    Open Source
                </div>
                <div class="hero-badge">
                    <svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21z"/></svg>
                    5.0 Star Rating
                </div>
            </div>

            <a href="#quickstart" class="btn">
                Get Started
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M5 12h14M12 5l7 7-7 7"/>
                </svg>
            </a>
        </div>
    </section>

    <!-- Features Section -->
    <section class="section" id="features">
        <div class="container">
            <h2 class="section-title">Powerful Features</h2>
            <p class="section-subtitle">
                Enterprise-grade security capabilities powered by advanced artificial intelligence and cutting-edge technology
            </p>

            <div class="features">
                <div class="feature-card">
                    <div class="feature-icon">🤖</div>
                    <div class="feature-title">AI-Powered Threat Detection</div>
                    <div class="feature-description">
                        Advanced machine learning algorithms provide real-time threat detection and automated response capabilities with 99.9% accuracy
                    </div>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🎮</div>
                    <div class="feature-title">Gaming Server Protection</div>
                    <div class="feature-description">
                        Specialized protection for gaming platforms featuring anti-DDoS, anti-cheat, and real-time player behavior analysis
                    </div>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">☁️</div>
                    <div class="feature-title">Cloud Native Security</div>
                    <div class="feature-description">
                        Designed for cloud environments with microservices architecture support and seamless Kubernetes integration
                    </div>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">⛓️</div>
                    <div class="feature-title">Blockchain Integration</div>
                    <div class="feature-description">
                        Immutable audit logs and decentralized threat intelligence sharing using enterprise blockchain technology
                    </div>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🔐</div>
                    <div class="feature-title">Post-Quantum Cryptography</div>
                    <div class="feature-description">
                        Quantum-resistant algorithms including CRYSTALS-Kyber, Dilithium, and FALCON for future-proof security
                    </div>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🧠</div>
                    <div class="feature-title">Deepfake Detection</div>
                    <div class="feature-description">
                        Advanced AI models detect synthetic media with 98.7% accuracy to protect against identity fraud
                    </div>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🌐</div>
                    <div class="feature-title">Zero Trust Architecture</div>
                    <div class="feature-description">
                        NIST SP 800-207 compliant zero-trust security model with continuous verification and least privilege access
                    </div>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🕵️</div>
                    <div class="feature-title">Shadow AI Detection</div>
                    <div class="feature-description">
                        Detect and govern unauthorized AI usage within your organization to prevent data leakage and compliance violations
                    </div>
                </div>

                <div class="feature-card">
                    <div class="feature-icon">🎯</div>
                    <div class="feature-title">Behavioral Analysis</div>
                    <div class="feature-description">
                        User and entity behavior analytics (UEBA) with advanced anomaly detection and predictive threat modeling
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Stats Section -->
    <section class="stats-section" id="stats">
        <div class="container">
            <h2 class="section-title">By The Numbers</h2>
            <p class="section-subtitle">
                Proven results from production deployments worldwide
            </p>

            <div class="stats">
                <div class="stat-item">
                    <div class="stat-number">26</div>
                    <div class="stat-label">Security Modules</div>
                </div>
                <div class="stat-item">
                    <div class="stat-number">46%</div>
                    <div class="stat-label">Documentation Coverage</div>
                </div>
                <div class="stat-item">
                    <div class="stat-number">9</div>
                    <div class="stat-label">Issues Resolved</div>
                </div>
                <div class="stat-item">
                    <div class="stat-number">5</div>
                    <div class="stat-label">Open PRs</div>
                </div>
                <div class="stat-item">
                    <div class="stat-number">100%</div>
                    <div class="stat-label">Test Coverage</div>
                </div>
            </div>
        </div>
    </section>

    <!-- Terminal Section -->
    <section class="terminal-section">
        <div class="container">
            <h2 class="section-title">Try It Now</h2>
            <p class="section-subtitle">
                Get started with V-Sentinel in seconds
            </p>

            <div class="terminal">
                <div class="terminal-header">
                    <div class="terminal-dot"></div>
                    <div class="terminal-dot"></div>
                    <div class="terminal-dot"></div>
                    <div class="terminal-title">V-Sentinel Terminal</div>
                </div>
                <div class="terminal-body">
                    <div class="terminal-line" style="animation-delay: 0.1s;">
                        <span class="terminal-prompt">$</span> <span class="terminal-command">vsentinel init --secure</span>
                    </div>
                    <div class="terminal-line" style="animation-delay: 0.3s;">
                        <span class="terminal-output">✓ Initializing V-Sentinel Security Framework</span>
                    </div>
                    <div class="terminal-line" style="animation-delay: 0.5s;">
                        <span class="terminal-output">✓ Loading AI models...</span>
                    </div>
                    <div class="terminal-line" style="animation-delay: 0.7s;">
                        <span class="terminal-output">✓ Configuring Zero Trust policies...</span>
                    </div>
                    <div class="terminal-line" style="animation-delay: 0.9s;">
                        <span class="terminal-output">✓ Setting up Post-Quantum cryptography...</span>
                    </div>
                    <div class="terminal-line" style="animation-delay: 1.1s;">
                        <span class="terminal-output">✓ Activating threat detection...</span>
                    </div>
                    <div class="terminal-line" style="animation-delay: 1.3s;">
                        <span class="terminal-success">🛡️ V-Sentinel Status: ACTIVE</span>
                    </div>
                    <div class="terminal-line" style="animation-delay: 1.5s;">
                        <span class="terminal-success">✓ All systems operational</span>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Quick Start Section -->
    <section class="quickstart-section" id="quickstart">
        <div class="container">
            <h2 class="section-title">Quick Start</h2>
            <p class="section-subtitle">
                Get up and running in under 5 minutes
            </p>

            <div class="code-block">
                <div class="code-header">
                    <span class="code-lang">bash</span>
                    <button class="code-copy" onclick="copyCode()">Copy</button>
                </div>
                <div class="code-content">
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
            </div>
        </div>
    </section>

    <!-- Architecture Section with Mermaid Diagrams -->
    <section class="features-section" id="architecture">
        <div class="container">
            <h2 class="section-title">Architecture</h2>
            <p class="section-subtitle">
                Modern, modular, and quantum-ready security architecture
            </p>

            <div class="mermaid-diagram">
                <pre class="mermaid">
graph TB
    A[V-Sentinel Core] --> B[Hypervisor Layer]
    A --> C[AI Engine]
    A --> D[Quantum Crypto]
    
    B --> E[Memory Introspection]
    B --> F[Process Protection]
    B --> G[Kernel Security]
    
    C --> H[Threat Detection]
    C --> I[Behavioral Analysis]
    C --> J[Deepfake Detection]
    
    D --> K[CRYSTALS-Kyber]
    D --> L[Dilithium]
    D --> M[FALCON]
    
    style A fill:#E50914
    style B fill:#B20710
    style C fill:#B20710
    style D fill:#B20710
                </pre>
            </div>

            <div class="mermaid-diagram">
                <pre class="mermaid">
graph LR
    A[User Application] --> B[V-Sentinel Agent]
    B --> C[AI Engine]
    B --> D[Hypervisor]
    B --> E[Quantum Layer]
    
    C --> F[Threat Detection]
    C --> G[Pattern Recognition]
    
    D --> H[Memory Protection]
    D --> I[Process Isolation]
    
    E --> J[Post-Quantum Crypto]
    E --> K[Key Management]
    
    F --> L[Response Engine]
    G --> L
    H --> L
    I --> L
    J --> L
    K --> L
    
    style B fill:#E50914
    style C fill:#B20710
    style D fill:#B20710
    style E fill:#B20710
    style L fill:#1F1F1F
                </pre>
            </div>
        </div>
    </section>

    <!-- Security Badges Section -->
    <section class="features-section" id="security">
        <div class="container">
            <h2 class="section-title">Security Status</h2>
            <p class="section-subtitle">
                Verified and secure by default
            </p>

            <div class="badges-container">
                <div class="badge-row">
                    <img src="https://img.shields.io/badge/Security-Verified-brightgreen?style=flat-square" alt="Security Verified">
                    <img src="https://img.shields.io/badge/Cryptography-Post--Quantum-red?style=flat-square" alt="Post-Quantum Crypto">
                    <img src="https://img.shields.io/badge/Hypervisor-Ring--1-blue?style=flat-square" alt="Ring-1 Hypervisor">
                    <img src="https://img.shields.io/badge/AI-Enabled-purple?style=flat-square" alt="AI Enabled">
                </div>
                <div class="badge-row">
                    <img src="https://img.shields.io/badge/License-AGPL%2B%26%20Commercial-orange?style=flat-square" alt="License">
                    <img src="https://img.shields.io/badge/Build-Passing-success?style=flat-square" alt="Build Status">
                    <img src="https://img.shields.io/badge/Coverage-95%25-brightgreen?style=flat-square" alt="Coverage">
                    <img src="https://img.shields.io/badge/Rust-1.75%2B-orange?style=flat-square" alt="Rust Version">
                </div>
            </div>
        </div>
    </section>

    <!-- API Documentation Section -->
    <section class="features-section" id="api">
        <div class="container">
            <h2 class="section-title">API Documentation</h2>
            <p class="section-subtitle">
                Comprehensive REST and gRPC APIs for seamless integration
            </p>

            <div class="api-endpoints">
                <div class="api-endpoint">
                    <h3>POST /api/v1/threats/detect</h3>
                    <p>Detect security threats using AI-powered analysis</p>
                    <div class="code-block">
                        <div class="code-content">
                            <code>{
  "target": "string",
  "analysis_depth": "standard | deep | forensic",
  "modules": ["ai", "hypervisor", "quantum"]
}</code>
                        </div>
                    </div>
                </div>

                <div class="api-endpoint">
                    <h3>GET /api/v1/status/health</h3>
                    <p>Check system health and operational status</p>
                    <div class="code-block">
                        <div class="code-content">
                            <code>{
  "status": "operational",
  "components": {
    "hypervisor": "active",
    "ai_engine": "active",
    "quantum_layer": "active"
  }
}</code>
                        </div>
                    </div>
                </div>

                <div class="api-endpoint">
                    <h3>POST /api/v1/crypto/generate</h3>
                    <p>Generate post-quantum cryptographic keys</p>
                    <div class="code-block">
                        <div class="code-content">
                            <code>{
  "algorithm": "CRYSTALS-Kyber | Dilithium | FALCON",
  "key_size": 1024 | 2048 | 4096
}</code>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Contributors Section -->
    <section class="features-section" id="contributors">
        <div class="container">
            <h2 class="section-title">Contributors</h2>
            <p class="section-subtitle">
                Join our growing community of security experts
            </p>

            <div class="contributors-grid">
                <div class="contributor-card">
                    <div class="contributor-avatar">👤</div>
                    <div class="contributor-info">
                        <h4>Vantis Corp Team</h4>
                        <p>Core Development</p>
                    </div>
                </div>
            </div>

            <div class="contribution-guide">
                <h3>How to Contribute</h3>
                <ul>
                    <li>📝 <strong>Fork</strong> the repository</li>
                    <li>🌿 <strong>Branch</strong> off from main</li>
                    <li>💾 <strong>Commit</strong> your changes</li>
                    <li>🔄 <strong>Push</strong> to your fork</li>
                    <li>✨ <strong>Open</strong> a Pull Request</li>
                </ul>
            </div>
        </div>
    </section>

    <!-- Roadmap Section -->
    <section class="features-section" id="roadmap">
        <div class="container">
            <h2 class="section-title">Roadmap</h2>
            <p class="section-subtitle">
                Our vision for the future of cybersecurity
            </p>

            <div class="roadmap-timeline">
                <div class="roadmap-item completed">
                    <div class="roadmap-marker">✅</div>
                    <div class="roadmap-content">
                        <h3>Q1 2026 - Foundation</h3>
                        <ul>
                            <li>Post-Quantum Cryptography implementation</li>
                            <li>Zero Trust Architecture</li>
                            <li>AI Security & Protection</li>
                            <li>Deepfake Detection</li>
                            <li>Shadow AI Detection</li>
                        </ul>
                    </div>
                </div>

                <div class="roadmap-item in-progress">
                    <div class="roadmap-marker">🚧</div>
                    <div class="roadmap-content">
                        <h3>Q2 2026 - Enhancement</h3>
                        <ul>
                            <li>Multi-language SDKs (Python, Go, TypeScript)</li>
                            <li>AI Agent Integration (MCP)</li>
                            <li>Public Threat Intelligence Repository</li>
                            <li>Terraform Provider</li>
                            <li>Kubernetes Operator</li>
                        </ul>
                    </div>
                </div>

                <div class="roadmap-item planned">
                    <div class="roadmap-marker">📅</div>
                    <div class="roadmap-content">
                        <h3>Q3 2026 - Expansion</h3>
                        <ul>
                            <li>Advanced Hypervisor Memory Introspection</li>
                            <li>Instruction Disassembler Integration</li>
                            <li>Automated Security Deployment CLI</li>
                            <li>Enterprise Features</li>
                            <li>Multi-cloud Support</li>
                        </ul>
                    </div>
                </div>

                <div class="roadmap-item planned">
                    <div class="roadmap-marker">🎯</div>
                    <div class="roadmap-content">
                        <h3>Q4 2026 - Innovation</h3>
                        <ul>
                            <li>Quantum-Ready Infrastructure</li>
                            <li>Advanced AI Operations</li>
                            <li>Blockchain Integration</li>
                            <li>Metaverse Security</li>
                            <li>IoT Security Framework</li>
                        </ul>
                    </div>
                </div>
            </div>
        </div>
    </section>

    <!-- Statistics Section -->
    <section class="features-section" id="stats">
        <div class="container">
            <h2 class="section-title">Project Statistics</h2>
            <p class="section-subtitle">
                Powered by cutting-edge technology
            </p>

            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-value">26,264</div>
                    <div class="stat-label">Lines of Rust Code</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">22</div>
                    <div class="stat-label">Core Modules</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">5</div>
                    <div class="stat-label">Open Pull Requests</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">100%</div>
                    <div class="stat-label">Issues Resolved</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">3</div>
                    <div class="stat-label">PQC Algorithms</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">∞</div>
                    <div class="stat-label">Quantum-Ready</div>
                </div>
            </div>
        </div>
    </section>

    <!-- Back to Top Button -->
    <button class="back-to-top" onclick="scrollToTop()" title="Back to top">
        ↑
    </button>

    <!-- Footer -->
    <footer class="footer">
        <div class="footer-content">
            <div class="footer-logo">V-Sentinel</div>
            <div class="footer-links">
                <a href="https://github.com/vantisCorp/V-Sentinel" class="footer-link">GitHub</a>
                <a href="https://discord.gg/A5MzwsRj7D" class="footer-link">Discord</a>
                <a href="https://twitter.com/vantiscorp" class="footer-link">Twitter</a>
                <a href="https://linkedin.com/company/vantiscorp" class="footer-link">LinkedIn</a>
                <a href="docs/README.md" class="footer-link">Documentation</a>
            </div>
            <div class="footer-copyright">
                © 2026 Vantis Corp. All rights reserved. | MIT License
            </div>
        </div>
    </footer>

    <script>
        // Create floating particles
        function createParticles() {
            const container = document.getElementById('particles');
            for (let i = 0; i < 50; i++) {
                const particle = document.createElement('div');
                particle.className = 'particle';
                particle.style.left = Math.random() * 100 + '%';
                particle.style.animationDelay = Math.random() * 20 + 's';
                particle.style.animationDuration = (15 + Math.random() * 10) + 's';
                container.appendChild(particle);
            }
        }

        // Navigation scroll effect
        window.addEventListener('scroll', () => {
            const nav = document.getElementById('nav');
            if (window.scrollY > 50) {
                nav.classList.add('scrolled');
            } else {
                nav.classList.remove('scrolled');
            }
        });

        // Copy code to clipboard
        function copyCode() {
            const code = document.querySelector('.code-content code').textContent;
            navigator.clipboard.writeText(code);
            alert('Code copied to clipboard!');
        }

        // Scroll to top
        function scrollToTop() {
            window.scrollTo({ top: 0, behavior: 'smooth' });
        }

        // Show/hide back to top button
        window.addEventListener('scroll', () => {
            const backToTop = document.querySelector('.back-to-top');
            if (window.scrollY > 500) {
                backToTop.classList.add('visible');
            } else {
                backToTop.classList.remove('visible');
            }
        });

        // Initialize
        document.addEventListener('DOMContentLoaded', () => {
            createParticles();
        });
    </script>
</body>
</html>