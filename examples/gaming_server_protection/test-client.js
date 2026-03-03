/**
 * SENTINEL Gaming Server Test Client
 * 
 * This script demonstrates how to connect to the SENTINEL-protected gaming server
 * and test various security features.
 * 
 * Usage: node test-client.js
 */

const WebSocket = require('ws');

const SERVER_URL = process.env.SERVER_URL || 'ws://localhost:3000';
const TEST_PLAYER_ID = `test_player_${Math.floor(Math.random() * 1000)}`;

console.log(`
╔══════════════════════════════════════════════════════════════╗
║         SENTINEL Gaming Server Test Client                    ║
╠══════════════════════════════════════════════════════════════╣
║  Connecting to: ${SERVER_URL}                                ║
║  Player ID: ${TEST_PLAYER_ID}                                ║
╚══════════════════════════════════════════════════════════════╝
`);

const ws = new WebSocket(SERVER_URL);

let sessionId = null;
let messageCount = 0;

// Connection established
ws.on('open', () => {
    console.log('✓ WebSocket connected');
    
    // Send handshake
    const handshake = {
        type: 'handshake',
        player_id: TEST_PLAYER_ID,
        client_version: '1.0.0',
        timestamp: Date.now()
    };
    
    console.log('→ Sending handshake...');
    ws.send(JSON.stringify(handshake));
});

// Handle messages from server
ws.on('message', (data) => {
    try {
        const message = JSON.parse(data);
        
        switch (message.type) {
            case 'handshake_complete':
                sessionId = message.session_id;
                console.log('✓ Handshake complete!');
                console.log(`  Session ID: ${sessionId}`);
                console.log(`  Trust Score: ${message.trust_score}`);
                console.log(`  Server Tick Rate: ${message.server_tick_rate}`);
                
                // Start test sequence
                setTimeout(runTestSequence, 1000);
                break;
            
            case 'game_state':
                messageCount++;
                if (messageCount % 60 === 0) { // Every second
                    console.log(`✓ Received game state (tick ${message.tick}, ${Object.keys(message.players).length} players)`);
                }
                break;
            
            case 'action_ack':
                console.log('✓ Action acknowledged:', message.action_id);
                break;
            
            case 'security_alert':
                console.log('⚠ Security alert:', message.message);
                break;
            
            case 'error':
                console.log('✗ Server error:', message.message);
                break;
            
            case 'chat':
                console.log(`💬 [${message.player_id}]: ${message.text}`);
                break;
            
            default:
                console.log('Received message:', message.type);
        }
    } catch (error) {
        console.error('Failed to parse message:', error.message);
    }
});

// Handle connection events
ws.on('close', (code, reason) => {
    console.log(`\n✗ Disconnected (code: ${code}, reason: ${reason})`);
    console.log('\nTest completed.');
});

ws.on('error', (error) => {
    console.error('✗ WebSocket error:', error.message);
});

// Test sequence
function runTestSequence() {
    console.log('\n--- Starting Test Sequence ---\n');
    
    // Test 1: Player movement
    testPlayerMovement();
    
    // Test 2: Player actions
    setTimeout(testPlayerActions, 3000);
    
    // Test 3: Chat messages
    setTimeout(testChatMessages, 6000);
    
    // Test 4: Disconnect
    setTimeout(() => {
        console.log('\n--- Test Sequence Complete ---\n');
        console.log('Disconnecting...');
        ws.close();
    }, 10000);
}

// Test player movement
function testPlayerMovement() {
    console.log('Test 1: Sending player movements...');
    
    let position = { x: 0, y: 0, z: 0 };
    let moves = 0;
    const maxMoves = 120; // 2 seconds at 60 FPS
    
    const interval = setInterval(() => {
        position.x += 0.1;
        position.y += 0.05;
        position.z = Math.sin(moves * 0.1) * 2;
        
        ws.send(JSON.stringify({
            type: 'player_move',
            position: { ...position },
            timestamp: Date.now()
        }));
        
        moves++;
        if (moves >= maxMoves) {
            clearInterval(interval);
            console.log('✓ Player movement test complete');
        }
    }, 16); // ~60 FPS
}

// Test player actions
function testPlayerActions() {
    console.log('\nTest 2: Sending player actions...');
    
    const actions = [
        { type: 'jump', timestamp: Date.now() },
        { type: 'attack', target: 'enemy_1', timestamp: Date.now() },
        { type: 'interact', object: 'door', timestamp: Date.now() },
        { type: 'use_item', item_id: 'health_potion', timestamp: Date.now() }
    ];
    
    actions.forEach((action, index) => {
        setTimeout(() => {
            ws.send(JSON.stringify({
                type: 'player_action',
                action_id: `action_${index}`,
                action: action
            }));
            console.log(`→ Sent action: ${action.type}`);
        }, index * 500);
    });
    
    setTimeout(() => {
        console.log('✓ Player actions test complete');
    }, actions.length * 500);
}

// Test chat messages
function testChatMessages() {
    console.log('\nTest 3: Sending chat messages...');
    
    const messages = [
        'Hello from test client!',
        'This is a normal message',
        'Testing the chat system',
        'Everything looks good!'
    ];
    
    messages.forEach((msg, index) => {
        setTimeout(() => {
            ws.send(JSON.stringify({
                type: 'chat',
                text: msg
            }));
            console.log(`→ Sent chat: ${msg}`);
        }, index * 1000);
    });
    
    setTimeout(() => {
        console.log('✓ Chat messages test complete');
    }, messages.length * 1000);
}

// Handle Ctrl+C
process.on('SIGINT', () => {
    console.log('\n\nInterrupted by user');
    if (ws.readyState === WebSocket.OPEN) {
        ws.close();
    }
    process.exit(0);
});