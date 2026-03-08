# V-Sentinel TypeScript SDK

[![npm version](https://img.shields.io/npm/v/@vantis/v-sentinel-sdk.svg)](https://www.npmjs.com/package/@vantis/v-sentinel-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.3-blue.svg)](https://www.typescriptlang.org/)

Official TypeScript SDK for the V-Sentinel security operations platform. Provides a comprehensive client for interacting with V-Sentinel APIs with full TypeScript support, async/await patterns, and automatic retries.

## Features

- **Full TypeScript support** - Complete type definitions for all API entities
- **Promise-based** - Modern async/await patterns
- **Automatic retries** - Built-in retry logic with exponential backoff
- **Tree-shakeable** - ES modules support for optimal bundle size
- **Comprehensive API** - Full coverage of Detections, Hosts, Incidents, and Threat Intelligence
- **Error handling** - Specific error types for different scenarios

## Installation

```bash
# Using npm
npm install @vantis/v-sentinel-sdk

# Using yarn
yarn add @vantis/v-sentinel-sdk

# Using pnpm
pnpm add @vantis/v-sentinel-sdk
```

## Quick Start

```typescript
import { VSentinelClient, DetectionSeverity } from '@vantis/v-sentinel-sdk';

// Create client
const client = new VSentinelClient({
  apiKey: 'your-api-key',
});

// List recent critical detections
const detections = await client.detections.list({
  severity: DetectionSeverity.Critical,
  timeRange: '24h',
  limit: 10,
});

for (const detection of detections.items) {
  console.log(`[${detection.severity}] ${detection.title}`);
  console.log(`  Host: ${detection.host?.hostname}`);
  console.log(`  MITRE Techniques: ${detection.techniques?.join(', ')}`);
}

// Check an IOC against threat intelligence
const result = await client.threatIntel.checkIOC('192.0.2.1');

if (result.isMalicious) {
  console.log('Malicious IOC detected!');
  console.log(`  Threat Actor: ${result.threatActor}`);
  console.log(`  Confidence: ${result.confidence}`);
}
```

## Authentication

### API Key

```typescript
import { VSentinelClient } from '@vantis/v-sentinel-sdk';

const client = new VSentinelClient({
  apiKey: 'your-api-key',
});
```

### Environment Variable

```typescript
import { VSentinelClient } from '@vantis/v-sentinel-sdk';

const client = new VSentinelClient({
  apiKey: process.env.V_SENTINEL_API_KEY!,
});
```

### Custom Configuration

```typescript
import { VSentinelClient } from '@vantis/v-sentinel-sdk';

const client = new VSentinelClient({
  apiKey: 'your-api-key',
  baseURL: 'https://api.custom.com/v1',
  timeout: 60000,
  maxRetries: 5,
  proxy: 'http://proxy.company.com:8080',
  debug: true,
});
```

## API Reference

### Detections

Manage security detections and alerts.

```typescript
import { DetectionSeverity, DetectionStatus } from '@vantis/v-sentinel-sdk';

// List detections with filters
const detections = await client.detections.list({
  severity: DetectionSeverity.High,
  status: DetectionStatus.New,
  timeRange: '7d',
  hostId: 'host-123',
  limit: 50,
});

// Get a specific detection
const detection = await client.detections.get('detection-456');

// Update detection status
const updated = await client.detections.updateStatus('detection-456', {
  status: DetectionStatus.InProgress,
  notes: 'Investigating potential false positive',
});

// Assign detection to analyst
const assigned = await client.detections.assign(
  'detection-456',
  'analyst@company.com'
);

// Add note to detection
const note = await client.detections.addNote(
  'detection-456',
  'Confirmed as true positive'
);
```

### Hosts

Manage endpoint inventory and isolation.

```typescript
import { HostPlatform, HostState } from '@vantis/v-sentinel-sdk';

// List hosts
const hosts = await client.hosts.list({
  platform: HostPlatform.Windows,
  state: HostState.Online,
  isIsolated: false,
  limit: 100,
});

// Get host details
const host = await client.hosts.get('host-123');

// Search hosts by hostname
const results = await client.hosts.search('workstation-', 50);

// Isolate a host
const isolated = await client.hosts.isolate(
  'host-123',
  'Malware detected - immediate containment required'
);

// Get host detections
const hostDetections = await client.hosts.getDetections('host-123', 20);

// Add tag to host
const tagged = await client.hosts.addTag('host-123', 'investigated');
```

### Incidents

Manage security incident lifecycle.

```typescript
import {
  IncidentSeverity,
  IncidentStatus,
  IncidentPhase
} from '@vantis/v-sentinel-sdk';

// Create an incident
const incident = await client.incidents.create({
  title: 'Ransomware Detection on Finance Server',
  description: 'LockBit ransomware detected on FIN-SRV-01',
  severity: IncidentSeverity.Critical,
  hosts: ['host-123', 'host-456'],
  detections: ['detection-789'],
  tags: ['ransomware', 'lockbit', 'finance'],
});

// List incidents
const incidents = await client.incidents.list({
  severity: IncidentSeverity.Critical,
  status: IncidentStatus.InProgress,
  timeRange: '30d',
});

// Update incident phase
const updated = await client.incidents.updatePhase(incident.id, {
  phase: IncidentPhase.Containment,
  notes: 'Hosts isolated, ransomware binaries removed',
});

// Add hosts to incident
const withHosts = await client.incidents.addHosts(incident.id, ['host-789']);

// Close incident
const closed = await client.incidents.close(incident.id, {
  resolution: 'Ransomware contained and eradicated. Systems restored from backup.',
  lessonsLearned: 'Consider adding additional email filtering for phishing attachments.',
});
```

### Threat Intelligence

Manage IOCs and threat actor data.

```typescript
import { IOCType, IOCConfidence } from '@vantis/v-sentinel-sdk';

// Check an IOC
const result = await client.threatIntel.checkIOC('malicious.example.com');

if (result.isMalicious) {
  console.log(`Threat Actor: ${result.threatActor}`);
  console.log(`Related IOCs: ${result.relatedIOCs?.length}`);
}

// Bulk check IOCs
const results = await client.threatIntel.checkIOCsBulk([
  '192.0.2.1',
  'malware.hash.example',
  'https://malicious.url/path',
]);

// Add an IOC
const ioc = await client.threatIntel.addIOC({
  value: '192.0.2.100',
  type: IOCType.IP,
  confidence: IOCConfidence.High,
  threatActor: 'APT29',
  campaign: 'CozyBear-2024',
  malwareFamily: 'SUNBURST',
  tags: ['apt29', 'supply-chain'],
  description: 'C2 server identified in SolarWinds campaign',
});

// Get threat actor information
const actor = await client.threatIntel.getThreatActor('APT29');
console.log(`Aliases: ${actor.aliases?.join(', ')}`);
console.log(`MITRE ID: ${actor.mitreId}`);

// Get actor's IOCs
const actorIOCs = await client.threatIntel.getThreatActorIOCs('APT29');

// Export IOCs for SIEM
const siemRules = await client.threatIntel.exportForSIEM(
  'splunk',
  [IOCType.IP, IOCType.Domain, IOCType.URL]
);
```

## Error Handling

The SDK provides specific error types for different scenarios:

```typescript
import {
  VSentinelError,
  AuthenticationError,
  RateLimitError,
  ResourceNotFoundError,
  ValidationError,
  isRateLimitError,
} from '@vantis/v-sentinel-sdk';

try {
  const detection = await client.detections.get('invalid-id');
} catch (error) {
  if (error instanceof ResourceNotFoundError) {
    console.log(`Resource not found: ${error.resourceId}`);
  } else if (error instanceof AuthenticationError) {
    console.log('Invalid API key');
  } else if (isRateLimitError(error)) {
    console.log(`Rate limited. Retry after: ${error.retryAfter} seconds`);
  } else if (error instanceof ValidationError) {
    console.log(`Validation failed: ${error.message}`);
    console.log('Details:', error.details);
  } else {
    console.log('Error:', error);
  }
}
```

## Configuration Options

```typescript
interface ClientOptions {
  /** API key for authentication (required) */
  apiKey: string;

  /** Base URL for API requests */
  baseURL?: string;  // default: 'https://api.vantis.ai/v1'

  /** Request timeout in milliseconds */
  timeout?: number;  // default: 30000

  /** Maximum number of retries */
  maxRetries?: number;  // default: 3

  /** Proxy URL */
  proxy?: string;

  /** Enable debug logging */
  debug?: boolean;

  /** Custom HTTP headers */
  headers?: Record<string, string>;
}
```

## Retries

The SDK automatically retries on transient errors:

- Retries on: 429, 500, 502, 503, 504 status codes
- Exponential backoff: 1s → 2s → 4s → ... (max 30s)
- Respects `Retry-After` header for 429 responses

## Development

### Requirements

- Node.js 18.0.0 or later
- npm, yarn, or pnpm

### Build

```bash
npm run build
```

### Test

```bash
npm test

# With coverage
npm run test:coverage
```

### Lint

```bash
npm run lint
npm run lint:fix
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [https://docs.vantis.ai/sdks/typescript](https://docs.vantis.ai/sdks/typescript)
- **API Reference**: [https://api.vantis.ai/docs](https://api.vantis.ai/docs)
- **GitHub Issues**: [https://github.com/vantis-ai/v-sentinel-sdks/issues](https://github.com/vantis-ai/v-sentinel-sdks/issues)
- **Security Issues**: security@vantis.ai

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

---

Built with ❤️ by the Vantis AI team