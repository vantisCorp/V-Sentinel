/**
 * SENTINEL Gaming Server Protection Example
 * 
 * This Node.js gaming server demonstrates how to integrate SENTINEL Security System
 * for comprehensive gaming protection including:
 * - Trusted Handshake protocol
 * - Real-time player verification
 * - Anti-DDoS protection
 * - RAM defolding optimization
 * - Zero-latency threat blocking
 * 
 * Author: SENTINEL Security Team
 * License: MIT
 */

const http = require('http');
const WebSocket = require('ws');
const crypto = require('crypto');
const { EventEmitter } = require('events');

// SENTINEL SDK Integration (mock for demo)
class SentinelClient {
    constructor(config = {}) {
        this.apiKey = config.apiKey || process.env.SENTINEL_API_KEY || 'demo-key';
        this.baseUrl = config.baseUrl || process.env.SENTINEL_API_URL || 'https://api.sentinel.ai/v1';
        this.connected = true;
    }

    /**
     * Perform trusted handshake verification
     */
    async trustedHandshake(playerData) {
        // In production, this calls the actual SENTINEL API
        const hash = crypto.createHash('sha256')
            .update(JSON.stringify(playerData) + Date.now())
            .digest('hex');
        
        return {
            verified: true,
            trust_score: 0.95,
            session_token: hash.substring(0, 32),
            expires_at: Date.now() + 3600000, // 1 hour
            handshake_time_ms: 3 // Near-instant verification
        };
    }

    /**
     * Check for DDoS attack patterns
     */
    async checkDdosTraffic(sourceIp, requestCount, timeWindow) {
        const requestsPerSecond = requestCount / (timeWindow / 1000);
        const isAttack = requestsPerSecond > 1000;
        
        return {
            is_attack: isAttack,
            threat_level: isAttack ? 'HIGH' : 'LOW',
            requests_per_second: requestsPerSecond,
            recommendation: isAttack ? 'BLOCK' : 'ALLOW',
            mitigation_strategy: isAttack ? 'RATE_LIMIT' : null
        };
    }

    /**
     * Predict threat based on behavior
     */
    async predictThreat(features) {
        const score = Math.random() * 0.1; // Low score for demo
        return {
            threat_score: score,
            threat_type: score > 0.7 ? 'MALWARE' : 'BENIGN',
            confidence: 0.95,
            indicators: []
        };
    }

    /**
     * Optimize memory for gaming (RAM Defolding)
     */
    async optimizeMemory(processes) {
        return {
            optimized: true,
            memory_freed_mb: 256,
            gaming_priority_set: true,
            fps_improvement_percent: 21,
            latency_reduction_percent: 77
        };
    }
}

// Initialize SENTINEL
const sentinel = new SentinelClient();

// Gaming Server Configuration
const CONFIG = {
    port: process.env.PORT || 3000,
    maxPlayers: 100,
    tickRate: 64, // 64 ticks per second
    handshakeTimeout: 5000,
    rateLimit: {
        windowMs: 60000,
        maxRequests: 1000
    }
};

// Security Event Logger
class SecurityLogger {
    constructor() {
        this.events = [];
    }

    log(eventType, severity, details, metadata = {}) {
        const event = {
            timestamp: new Date().toISOString(),
            event_type: eventType,
            severity,
            details,
            ...metadata
        };
        this.events.push(event);
        console.log(`[SECURITY] ${severity}: ${eventType} - ${details}`);
        return event;
    }

    getRecent(limit = 100) {
        return this.events.slice(-limit);
    }
}

const securityLogger = new SecurityLogger();

// Player Session Manager
class PlayerSessionManager extends EventEmitter {
    constructor() {
        super();
        this.sessions = new Map();
        this.blockedIps = new Set();
        this.rateLimits = new Map();
    }

    async createSession(ws, handshakeData) {
        const clientIp = handshakeData.ip || 'unknown';
        
        // Check if IP is blocked
        if (this.blockedIps.has(clientIp)) {
            securityLogger.log('BLOCKED_IP_ATTEMPT', 'HIGH', 
                `Blocked IP attempted connection: ${clientIp}`);
            ws.close(1008, 'IP blocked');
            return null;
        }

        // Perform trusted handshake
        try {
            const handshake = await sentinel.trustedHandshake({
                player_id: handshakeData.player_id,
                client_version: handshakeData.client_version,
                ip: clientIp,
                timestamp: Date.now()
            });

            const session = {
                id: crypto.randomUUID(),
                player_id: handshakeData.player_id,
                ip: clientIp,
                ws,
                trust_score: handshake.trust_score,
                session_token: handshake.session_token,
                connected_at: Date.now(),
                last_activity: Date.now(),
                messages_received: 0,
                rate_limit_counter: 0
            };

            this.sessions.set(session.id, session);
            
            securityLogger.log('PLAYER_CONNECTED', 'LOW',
                `Player ${handshakeData.player_id} connected with trust score ${handshake.trust_score}`,
                { player_id: handshakeData.player_id, ip: clientIp });

            this.emit('session_created', session);
            return session;

        } catch (error) {
            securityLogger.log('HANDSHAKE_FAILED', 'MEDIUM',
                `Handshake failed for ${clientIp}: ${error.message}`);
            ws.close(1008, 'Handshake failed');
            return null;
        }
    }

    validateMessage(session, message) {
        // Rate limiting check
        const now = Date.now();
        const windowStart = now - CONFIG.rateLimit.windowMs;
        
        if (!this.rateLimits.has(session.id)) {
            this.rateLimits.set(session.id, { count: 0, windowStart: now });
        }
        
        const rateLimit = this.rateLimits.get(session.id);
        
        if (rateLimit.windowStart < windowStart) {
            rateLimit.count = 0;
            rateLimit.windowStart = now;
        }
        
        rateLimit.count++;
        
        if (rateLimit.count > CONFIG.rateLimit.maxRequests) {
            securityLogger.log('RATE_LIMIT_EXCEEDED', 'MEDIUM',
                `Rate limit exceeded for player ${session.player_id}`,
                { player_id: session.player_id, requests: rateLimit.count });
            return { valid: false, reason: 'Rate limit exceeded' };
        }

        // Update session activity
        session.last_activity = now;
        session.messages_received++;

        return { valid: true };
    }

    disconnectSession(sessionId, reason = 'Normal closure') {
        const session = this.sessions.get(sessionId);
        if (session) {
            session.ws.close(1000, reason);
            this.sessions.delete(sessionId);
            this.rateLimits.delete(sessionId);
            
            securityLogger.log('PLAYER_DISCONNECTED', 'LOW',
                `Player ${session.player_id} disconnected: ${reason}`,
                { player_id: session.player_id, duration_ms: Date.now() - session.connected_at });
            
            this.emit('session_ended', session);
        }
    }

    blockIp(ip, reason) {
        this.blockedIps.add(ip);
        
        // Disconnect all sessions from this IP
        for (const [id, session] of this.sessions) {
            if (session.ip === ip) {
                this.disconnectSession(id, 'IP blocked');
            }
        }
        
        securityLogger.log('IP_BLOCKED', 'HIGH', `IP blocked: ${ip}. Reason: ${reason}`);
    }

    getStats() {
        return {
            active_sessions: this.sessions.size,
            blocked_ips: this.blockedIps.size,
            total_messages: Array.from(this.sessions.values())
                .reduce((sum, s) => sum + s.messages_received, 0)
        };
    }
}

// DDoS Protection Handler
class DdosProtection {
    constructor() {
        this.requestCounts = new Map();
        this.attackMode = false;
    }

    async checkRequest(ip) {
        const now = Date.now();
        const windowMs = 1000; // 1 second window
        
        if (!this.requestCounts.has(ip)) {
            this.requestCounts.set(ip, { count: 0, windowStart: now });
        }
        
        const data = this.requestCounts.get(ip);
        
        // Reset window if expired
        if (now - data.windowStart > windowMs) {
            data.count = 0;
            data.windowStart = now;
        }
        
        data.count++;
        
        // Check for DDoS pattern
        const result = await sentinel.checkDdosTraffic(ip, data.count, windowMs);
        
        if (result.is_attack) {
            this.attackMode = true;
            securityLogger.log('DDOS_DETECTED', 'CRITICAL',
                `DDoS attack detected from ${ip}: ${result.requests_per_second.toFixed(0)} req/s`);
            return { allowed: false, reason: 'DDoS protection triggered' };
        }
        
        return { allowed: true };
    }

    cleanup() {
        const now = Date.now();
        for (const [ip, data] of this.requestCounts) {
            if (now - data.windowStart > 60000) {
                this.requestCounts.delete(ip);
            }
        }
    }
}

// Game State Manager
class GameStateManager {
    constructor() {
        this.players = new Map();
        this.entities = new Map();
        this.tickCount = 0;
    }

    updatePlayerPosition(sessionId, position) {
        this.players.set(sessionId, {
            ...this.players.get(sessionId),
            position,
            lastUpdate: Date.now()
        });
    }

    getGameState() {
        return {
            tick: this.tickCount,
            players: Object.fromEntries(this.players),
            entities: Object.fromEntries(this.entities)
        };
    }

    tick() {
        this.tickCount++;
        // Game logic here
    }
}

// Initialize components
const sessionManager = new PlayerSessionManager();
const ddosProtection = new DdosProtection();
const gameState = new GameStateManager();

// Create HTTP server
const server = http.createServer((req, res) => {
    // CORS headers
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type');
    
    if (req.method === 'OPTIONS') {
        res.writeHead(204);
        res.end();
        return;
    }

    // Health check endpoint
    if (req.url === '/health') {
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({
            status: 'healthy',
            sentinel_connected: sentinel.connected,
            active_players: sessionManager.sessions.size,
            attack_mode: ddosProtection.attackMode,
            uptime: process.uptime()
        }));
        return;
    }

    // Stats endpoint
    if (req.url === '/stats') {
        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({
            server: {
                tick_rate: CONFIG.tickRate,
                max_players: CONFIG.maxPlayers
            },
            sessions: sessionManager.getStats(),
            game: gameState.getGameState(),
            security: {
                recent_events: securityLogger.getRecent(10),
                attack_mode: ddosProtection.attackMode
            }
        }));
        return;
    }

    // Block IP endpoint
    if (req.url === '/api/block-ip' && req.method === 'POST') {
        let body = '';
        req.on('data', chunk => body += chunk);
        req.on('end', () => {
            try {
                const { ip, reason } = JSON.parse(body);
                sessionManager.blockIp(ip, reason);
                res.writeHead(200, { 'Content-Type': 'application/json' });
                res.end(JSON.stringify({ success: true, ip }));
            } catch (e) {
                res.writeHead(400);
                res.end('Invalid request');
            }
        });
        return;
    }

    res.writeHead(404);
    res.end('Not found');
});

// Create WebSocket server
const wss = new WebSocket.Server({ server });

// Handle new connections
wss.on('connection', async (ws, req) => {
    const clientIp = req.socket.remoteAddress;
    
    // DDoS check
    const ddosCheck = await ddosProtection.checkRequest(clientIp);
    if (!ddosCheck.allowed) {
        ws.close(1008, ddosCheck.reason);
        return;
    }

    // Handle handshake
    ws.once('message', async (data) => {
        try {
            const handshakeData = JSON.parse(data);
            
            if (handshakeData.type !== 'handshake') {
                ws.close(1008, 'Expected handshake');
                return;
            }

            const session = await sessionManager.createSession(ws, {
                ...handshakeData,
                ip: clientIp
            });

            if (!session) return;

            // Send handshake confirmation
            ws.send(JSON.stringify({
                type: 'handshake_complete',
                session_id: session.id,
                trust_score: session.trust_score,
                server_tick_rate: CONFIG.tickRate
            }));

            // Handle subsequent messages
            ws.on('message', (data) => {
                handleGameMessage(session, data);
            });

            ws.on('close', () => {
                sessionManager.disconnectSession(session.id, 'Connection closed');
            });

            ws.on('error', (error) => {
                securityLogger.log('WEBSOCKET_ERROR', 'MEDIUM',
                    `WebSocket error for player ${session.player_id}: ${error.message}`);
            });

        } catch (error) {
            securityLogger.log('HANDSHAKE_PARSE_ERROR', 'MEDIUM',
                `Failed to parse handshake: ${error.message}`);
            ws.close(1008, 'Invalid handshake');
        }
    });

    // Handshake timeout
    setTimeout(() => {
        if (ws.readyState === WebSocket.OPEN) {
            ws.close(1008, 'Handshake timeout');
        }
    }, CONFIG.handshakeTimeout);
});

// Handle game messages
async function handleGameMessage(session, data) {
    // Validate message
    const validation = sessionManager.validateMessage(session, data);
    if (!validation.valid) {
        session.ws.send(JSON.stringify({
            type: 'error',
            message: validation.reason
        }));
        return;
    }

    try {
        const message = JSON.parse(data);
        
        // Threat check for suspicious activity
        if (isSuspiciousActivity(message)) {
            const threatResult = await sentinel.predictThreat({
                input_type: 'network',
                data: message,
                player_id: session.player_id,
                session_id: session.id
            });

            if (threatResult.threat_score > 0.8) {
                securityLogger.log('HIGH_THREAT_MESSAGE', 'CRITICAL',
                    `High threat message from ${session.player_id}: score ${threatResult.threat_score}`);
                session.ws.send(JSON.stringify({
                    type: 'security_alert',
                    message: 'Suspicious activity detected'
                }));
                return;
            }
        }

        // Process game message
        switch (message.type) {
            case 'player_move':
                gameState.updatePlayerPosition(session.id, message.position);
                broadcastGameState();
                break;
            
            case 'player_action':
                handlePlayerAction(session, message);
                break;
            
            case 'chat':
                handleChatMessage(session, message);
                break;
            
            default:
                // Unknown message type
                securityLogger.log('UNKNOWN_MESSAGE_TYPE', 'LOW',
                    `Unknown message type from ${session.player_id}: ${message.type}`);
        }
    } catch (error) {
        securityLogger.log('MESSAGE_PARSE_ERROR', 'LOW',
            `Failed to parse message from ${session.player_id}: ${error.message}`);
    }
}

// Suspicious activity detection
function isSuspiciousActivity(message) {
    // Check for impossible values
    if (message.position) {
        const { x, y, z } = message.position;
        if (Math.abs(x) > 10000 || Math.abs(y) > 10000 || Math.abs(z) > 10000) {
            return true;
        }
    }
    
    // Check for rapid actions
    if (message.timestamp && Date.now() - message.timestamp < 1) {
        return true;
    }
    
    return false;
}

// Broadcast game state to all players
function broadcastGameState() {
    const state = JSON.stringify({
        type: 'game_state',
        ...gameState.getGameState()
    });

    for (const session of sessionManager.sessions.values()) {
        if (session.ws.readyState === WebSocket.OPEN) {
            session.ws.send(state);
        }
    }
}

// Handle player actions
function handlePlayerAction(session, message) {
    // Process player action
    // In a real game, this would interact with the game logic
    
    session.ws.send(JSON.stringify({
        type: 'action_ack',
        action_id: message.action_id,
        success: true
    }));
}

// Handle chat messages
function handleChatMessage(session, message) {
    // Basic profanity filter (use SENTINEL content moderation in production)
    const suspiciousWords = ['hack', 'cheat', 'exploit'];
    const isSuspicious = suspiciousWords.some(word => 
        message.text.toLowerCase().includes(word)
    );

    if (isSuspicious) {
        securityLogger.log('SUSPICIOUS_CHAT', 'MEDIUM',
            `Suspicious chat from ${session.player_id}: ${message.text}`);
    }

    // Broadcast chat to all players
    const chatMessage = JSON.stringify({
        type: 'chat',
        player_id: session.player_id,
        text: message.text,
        timestamp: Date.now()
    });

    for (const sess of sessionManager.sessions.values()) {
        if (sess.ws.readyState === WebSocket.OPEN) {
            sess.ws.send(chatMessage);
        }
    }
}

// Game loop
setInterval(() => {
    gameState.tick();
}, 1000 / CONFIG.tickRate);

// Cleanup interval
setInterval(() => {
    ddosProtection.cleanup();
}, 30000);

// Start server
server.listen(CONFIG.port, () => {
    console.log(`
╔══════════════════════════════════════════════════════════════╗
║        SENTINEL Gaming Server Protection Demo                ║
╠══════════════════════════════════════════════════════════════╣
║  Server running on port ${CONFIG.port}                               ║
║  Tick Rate: ${CONFIG.tickRate}                                        ║
║  Max Players: ${CONFIG.maxPlayers}                                      ║
║  SENTINEL: Connected                                         ║
║  DDoS Protection: Active                                     ║
║  Trusted Handshake: Enabled                                  ║
╚══════════════════════════════════════════════════════════════╝

  Endpoints:
    ws://localhost:${CONFIG.port}          - WebSocket game server
    http://localhost:${CONFIG.port}/health - Health check
    http://localhost:${CONFIG.port}/stats  - Server statistics
    `);
});

// Graceful shutdown
process.on('SIGTERM', () => {
    console.log('Shutting down gracefully...');
    
    // Disconnect all players
    for (const [id] of sessionManager.sessions) {
        sessionManager.disconnectSession(id, 'Server shutdown');
    }
    
    wss.close(() => {
        server.close(() => {
            console.log('Server closed');
            process.exit(0);
        });
    });
});

module.exports = { server, sessionManager, ddosProtection, gameState };