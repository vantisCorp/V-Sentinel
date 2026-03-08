/**
 * V-Sentinel API Client
 *
 * Main client for interacting with V-Sentinel APIs.
 */

import axios, { AxiosInstance, AxiosError } from 'axios';
import axiosRetry from 'axios-retry';

import { DetectionsService } from './services/detections';
import { HostsService } from './services/hosts';
import { IncidentsService } from './services/incidents';
import { ThreatIntelService } from './services/threat-intel';
import {
  APIError,
  AuthenticationError,
  RateLimitError,
  ResourceNotFoundError,
  ValidationError,
  ConnectionError,
  TimeoutError,
  ErrorResponse,
} from './errors';

const DEFAULT_BASE_URL = 'https://api.vantis.ai/v1';
const DEFAULT_TIMEOUT = 30000;
const DEFAULT_MAX_RETRIES = 3;
const VERSION = '1.0.0';

/** Client configuration options */
export interface ClientOptions {
  /** API key for authentication */
  apiKey: string;
  /** Base URL for API requests */
  baseURL?: string;
  /** Request timeout in milliseconds */
  timeout?: number;
  /** Maximum number of retries */
  maxRetries?: number;
  /** Proxy URL */
  proxy?: string;
  /** Enable debug logging */
  debug?: boolean;
  /** Custom HTTP headers */
  headers?: Record<string, string>;
}

/**
 * V-Sentinel API Client
 *
 * Provides access to all V-Sentinel API services.
 *
 * @example
 * ```typescript
 * const client = new VSentinelClient({ apiKey: 'your-api-key' });
 *
 * // List detections
 * const detections = await client.detections.list({
 *   severity: DetectionSeverity.High,
 *   limit: 10,
 * });
 * ```
 */
export class VSentinelClient {
  private readonly httpClient: AxiosInstance;
  private readonly baseURL: string;

  /** Detections API service */
  readonly detections: DetectionsService;

  /** Hosts API service */
  readonly hosts: HostsService;

  /** Incidents API service */
  readonly incidents: IncidentsService;

  /** Threat Intelligence API service */
  readonly threatIntel: ThreatIntelService;

  constructor(options: ClientOptions) {
    if (!options.apiKey) {
      throw new Error('API key is required');
    }

    this.baseURL = options.baseURL ?? DEFAULT_BASE_URL;
    const timeout = options.timeout ?? DEFAULT_TIMEOUT;
    const maxRetries = options.maxRetries ?? DEFAULT_MAX_RETRIES;

    // Create axios instance
    this.httpClient = axios.create({
      baseURL: this.baseURL,
      timeout,
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json',
        'User-Agent': `v-sentinel-sdk-ts/${VERSION}`,
        'Authorization': `Bearer ${options.apiKey}`,
        ...options.headers,
      },
      proxy: options.proxy ? { host: new URL(options.proxy).hostname, port: parseInt(new URL(options.proxy).port) } : undefined,
    });

    // Configure retry
    axiosRetry(this.httpClient, {
      retries: maxRetries,
      retryDelay: (retryCount) => Math.min(retryCount * 1000, 30000),
      retryCondition: (error: AxiosError) => {
        const status = error.response?.status;
        return status === 429 || (status !== undefined && status >= 500);
      },
    });

    // Initialize services
    this.detections = new DetectionsService(this.httpClient);
    this.hosts = new HostsService(this.httpClient);
    this.incidents = new IncidentsService(this.httpClient);
    this.threatIntel = new ThreatIntelService(this.httpClient);
  }

  /**
   * Set the authorization header
   * @param apiKey - New API key
   */
  setApiKey(apiKey: string): void {
    this.httpClient.defaults.headers['Authorization'] = `Bearer ${apiKey}`;
  }

  /**
   * Get the current base URL
   */
  getBaseURL(): string {
    return this.baseURL;
  }
}

/**
 * Handle API error responses
 * @internal
 */
export function handleAPIError(error: unknown): never {
  if (axios.isAxiosError(error)) {
    const response = error.response;
    const status = response?.status;
    const data = response?.data as ErrorResponse | undefined;

    if (status === 401 || status === 403) {
      throw new AuthenticationError(data?.message ?? 'Authentication failed');
    }

    if (status === 404) {
      throw new ResourceNotFoundError(
        data?.message ?? 'Resource not found'
      );
    }

    if (status === 429) {
      const retryAfter = response?.headers['retry-after'];
      throw new RateLimitError(
        data?.message ?? 'Rate limit exceeded',
        retryAfter ? parseInt(retryAfter) : undefined
      );
    }

    if (status === 400) {
      throw new ValidationError(
        data?.message ?? 'Validation failed',
        data?.details
      );
    }

    if (status && status >= 500) {
      throw new APIError(
        status,
        data?.message ?? 'Internal server error',
        data
      );
    }

    if (error.code === 'ECONNABORTED' || error.code === 'ETIMEDOUT') {
      throw new TimeoutError('Request timed out');
    }

    if (error.code === 'ECONNREFUSED' || error.code === 'ENOTFOUND') {
      throw new ConnectionError(`Connection failed: ${error.message}`);
    }

    throw new APIError(
      status ?? 0,
      data?.message ?? error.message,
      data
    );
  }

  if (error instanceof Error) {
    throw error;
  }

  throw new Error('Unknown error occurred');
}