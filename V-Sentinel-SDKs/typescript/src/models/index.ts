/**
 * V-Sentinel SDK Models
 *
 * Type definitions for all API entities.
 */

// ==================== Enums ====================

/** Detection severity levels */
export enum DetectionSeverity {
  Critical = 'CRITICAL',
  High = 'HIGH',
  Medium = 'MEDIUM',
  Low = 'LOW',
  Informational = 'INFORMATIONAL',
}

/** Detection status values */
export enum DetectionStatus {
  New = 'NEW',
  InProgress = 'IN_PROGRESS',
  Resolved = 'RESOLVED',
  FalsePositive = 'FALSE_POSITIVE',
  Ignored = 'IGNORED',
}

/** Host state values */
export enum HostState {
  Online = 'ONLINE',
  Offline = 'OFFLINE',
  Isolated = 'ISOLATED',
  Unknown = 'UNKNOWN',
}

/** Host platform values */
export enum HostPlatform {
  Windows = 'WINDOWS',
  Linux = 'LINUX',
  MacOS = 'MACOS',
  Unknown = 'UNKNOWN',
}

/** Incident severity levels */
export enum IncidentSeverity {
  Critical = 'CRITICAL',
  High = 'HIGH',
  Medium = 'MEDIUM',
  Low = 'LOW',
}

/** Incident status values */
export enum IncidentStatus {
  New = 'NEW',
  InProgress = 'IN_PROGRESS',
  OnHold = 'ON_HOLD',
  Resolved = 'RESOLVED',
  Closed = 'CLOSED',
  FalsePositive = 'FALSE_POSITIVE',
}

/** Incident phase values */
export enum IncidentPhase {
  Identification = 'IDENTIFICATION',
  Containment = 'CONTAINMENT',
  Eradication = 'ERADICATION',
  Recovery = 'RECOVERY',
  LessonsLearned = 'LESSONS_LEARNED',
}

/** IOC type values */
export enum IOCType {
  IP = 'IP',
  Domain = 'DOMAIN',
  URL = 'URL',
  HashMD5 = 'HASH_MD5',
  HashSHA1 = 'HASH_SHA1',
  HashSHA256 = 'HASH_SHA256',
  Email = 'EMAIL',
  Certificate = 'CERTIFICATE',
}

/** IOC confidence levels */
export enum IOCConfidence {
  Low = 'LOW',
  Medium = 'MEDIUM',
  High = 'HIGH',
}

// ==================== Detection Types ====================

/** Indicator within a detection */
export interface Indicator {
  type: string;
  value: string;
  description?: string;
}

/** Note on a detection */
export interface DetectionNote {
  id: string;
  content: string;
  author: string;
  createdAt: string;
}

/** Security detection */
export interface Detection {
  id: string;
  title: string;
  description?: string;
  severity: DetectionSeverity;
  status: DetectionStatus;
  techniques?: string[];
  tactics?: string[];
  host?: Host;
  indicators?: Indicator[];
  rawData?: Record<string, unknown>;
  assignee?: string;
  notes?: DetectionNote[];
  tags?: string[];
  createdAt: string;
  updatedAt: string;
}

/** Paginated list of detections */
export interface DetectionList {
  items: Detection[];
  total: number;
  limit: number;
  offset: number;
  hasMore: boolean;
}

/** Detection query parameters */
export interface DetectionQuery {
  query?: string;
  severity?: DetectionSeverity;
  status?: DetectionStatus;
  hostId?: string;
  timeRange?: string;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
  limit?: number;
  offset?: number;
}

// ==================== Host Types ====================

/** Managed host/endpoint */
export interface Host {
  id: string;
  hostname: string;
  platform: HostPlatform;
  state: HostState;
  ipAddresses?: string[];
  osVersion?: string;
  agentVersion?: string;
  isIsolated: boolean;
  lastSeenAt?: string;
  firstSeenAt?: string;
  tags?: string[];
  createdAt: string;
  updatedAt: string;
}

/** Paginated list of hosts */
export interface HostList {
  items: Host[];
  total: number;
  limit: number;
  offset: number;
  hasMore: boolean;
}

/** Host query parameters */
export interface HostQuery {
  query?: string;
  platform?: HostPlatform;
  state?: HostState;
  groupId?: string;
  isIsolated?: boolean;
  limit?: number;
  offset?: number;
}

// ==================== Incident Types ====================

/** Incident timeline entry */
export interface IncidentTimelineEntry {
  timestamp: string;
  action: string;
  description?: string;
  user?: string;
}

/** Note on an incident */
export interface IncidentNote {
  id: string;
  content: string;
  author: string;
  visibility?: string;
  createdAt: string;
}

/** Security incident */
export interface Incident {
  id: string;
  title: string;
  description?: string;
  severity: IncidentSeverity;
  status: IncidentStatus;
  phase?: IncidentPhase;
  hosts?: string[];
  detections?: string[];
  assignee?: string;
  timeline?: IncidentTimelineEntry[];
  notes?: IncidentNote[];
  tags?: string[];
  createdAt: string;
  updatedAt: string;
}

/** Paginated list of incidents */
export interface IncidentList {
  items: Incident[];
  total: number;
  limit: number;
  offset: number;
  hasMore: boolean;
}

/** Incident query parameters */
export interface IncidentQuery {
  query?: string;
  severity?: IncidentSeverity;
  status?: IncidentStatus;
  phase?: IncidentPhase;
  assignee?: string;
  timeRange?: string;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
  limit?: number;
  offset?: number;
}

// ==================== Threat Intel Types ====================

/** Indicator of Compromise */
export interface IOC {
  id: string;
  type: IOCType;
  value: string;
  confidence: IOCConfidence;
  threatActor?: string;
  campaign?: string;
  malwareFamily?: string;
  tags?: string[];
  description?: string;
  references?: string[];
  firstSeen?: string;
  lastSeen?: string;
  expiresAt?: string;
  createdAt: string;
  updatedAt: string;
}

/** Paginated list of IOCs */
export interface IOCList {
  items: IOC[];
  total: number;
  limit: number;
  offset: number;
  hasMore: boolean;
}

/** Result of checking an IOC */
export interface IOCCheckResult {
  ioc?: IOC;
  isMalicious: boolean;
  confidence: IOCConfidence;
  threatActor?: string;
  malwareFamily?: string;
  relatedIOCs?: IOC[];
}

/** Threat actor/group */
export interface ThreatActor {
  name: string;
  aliases?: string[];
  country?: string;
  motivation?: string;
  mitreId?: string;
  firstSeen?: string;
  lastSeen?: string;
  tags?: string[];
  description?: string;
  createdAt: string;
  updatedAt: string;
}

/** Paginated list of threat actors */
export interface ThreatActorList {
  items: ThreatActor[];
  total: number;
  limit: number;
  offset: number;
  hasMore: boolean;
}

/** Threat campaign */
export interface Campaign {
  id: string;
  name: string;
  threatActor?: string;
  description?: string;
  status?: string;
  startDate?: string;
  endDate?: string;
  tags?: string[];
  createdAt: string;
  updatedAt: string;
}

/** Paginated list of campaigns */
export interface CampaignList {
  items: Campaign[];
  total: number;
  limit: number;
  offset: number;
  hasMore: boolean;
}

/** IOC query parameters */
export interface IOCQuery {
  type?: IOCType;
  confidence?: IOCConfidence;
  threatActor?: string;
  malwareFamily?: string;
  tags?: string[];
  timeRange?: string;
  sortBy?: string;
  sortOrder?: 'asc' | 'desc';
  limit?: number;
  offset?: number;
}