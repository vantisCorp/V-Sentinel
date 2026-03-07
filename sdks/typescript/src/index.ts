/**
 * V-Sentinel TypeScript SDK
 *
 * Official TypeScript SDK for the V-Sentinel security operations platform.
 * Provides a comprehensive client for interacting with V-Sentinel APIs.
 *
 * @packageDocumentation
 */

// Client
export { VSentinelClient, ClientOptions } from './client';

// Services
export { DetectionsService } from './services/detections';
export { HostsService } from './services/hosts';
export { IncidentsService } from './services/incidents';
export { ThreatIntelService } from './services/threat-intel';

// Models
export {
  // Detection types
  Detection,
  DetectionSeverity,
  DetectionStatus,
  DetectionList,
  DetectionQuery,
  Indicator,
  DetectionNote,

  // Host types
  Host,
  HostState,
  HostPlatform,
  HostList,

  // Incident types
  Incident,
  IncidentSeverity,
  IncidentStatus,
  IncidentPhase,
  IncidentList,
  IncidentTimelineEntry,
  IncidentNote,

  // Threat Intel types
  IOC,
  IOCType,
  IOCConfidence,
  IOCList,
  IOCCheckResult,
  ThreatActor,
  ThreatActorList,
  Campaign,
  CampaignList,
} from './models';

// Errors
export {
  VSentinelError,
  AuthenticationError,
  ResourceNotFoundError,
  ValidationError,
  RateLimitError,
  ConnectionError,
  TimeoutError,
  APIError,
} from './errors';

// Version
export const VERSION = '1.0.0';