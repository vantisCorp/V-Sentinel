/**
 * Hosts API Service
 *
 * Handles host-related API operations.
 */

import { AxiosInstance } from 'axios';
import { handleAPIError } from '../client';
import {
  Host,
  HostList,
  HostPlatform,
  HostState,
  DetectionList,
} from '../models';

/** Options for listing hosts */
export interface HostListOptions {
  query?: string;
  platform?: HostPlatform;
  state?: HostState;
  groupId?: string;
  isIsolated?: boolean;
  limit?: number;
  offset?: number;
}

/** Hosts API service */
export class HostsService {
  private readonly client: AxiosInstance;
  private readonly basePath = '/hosts';

  constructor(client: AxiosInstance) {
    this.client = client;
  }

  /**
   * List hosts with optional filtering
   */
  async list(options?: HostListOptions): Promise<HostList> {
    try {
      const params: Record<string, string | number | boolean> = {};

      if (options?.query) params.query = options.query;
      if (options?.platform) params.platform = options.platform;
      if (options?.state) params.state = options.state;
      if (options?.groupId) params.group_id = options.groupId;
      if (options?.isIsolated !== undefined) params.is_isolated = options.isIsolated;
      if (options?.limit) params.limit = options.limit;
      if (options?.offset) params.offset = options.offset;

      if (!params.limit) params.limit = 50;

      const response = await this.client.get<HostList>(this.basePath, { params });
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get a specific host by ID
   */
  async get(hostId: string): Promise<Host> {
    try {
      const response = await this.client.get<Host>(`${this.basePath}/${hostId}`);
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Search for hosts matching a query
   */
  async search(query: string, limit?: number): Promise<HostList> {
    try {
      const params: Record<string, string | number> = { query };
      if (limit) params.limit = limit;
      else params.limit = 50;

      const response = await this.client.get<HostList>(
        `${this.basePath}/search`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Isolate a host from the network
   */
  async isolate(hostId: string, reason: string): Promise<Host> {
    try {
      const response = await this.client.post<Host>(
        `${this.basePath}/${hostId}/isolate`,
        { reason }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Remove network isolation from a host
   */
  async unisolate(hostId: string): Promise<Host> {
    try {
      const response = await this.client.post<Host>(
        `${this.basePath}/${hostId}/unisolate`
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get detections for a specific host
   */
  async getDetections(hostId: string, limit?: number): Promise<DetectionList> {
    try {
      const params: Record<string, number> = {};
      if (limit) params.limit = limit;
      else params.limit = 20;

      const response = await this.client.get<DetectionList>(
        `${this.basePath}/${hostId}/detections`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Add a tag to a host
   */
  async addTag(hostId: string, tag: string): Promise<Host> {
    try {
      const response = await this.client.post<Host>(
        `${this.basePath}/${hostId}/tags`,
        { tag }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Remove a tag from a host
   */
  async removeTag(hostId: string, tag: string): Promise<void> {
    try {
      await this.client.delete(`${this.basePath}/${hostId}/tags`, {
        data: { tag },
      });
    } catch (error) {
      handleAPIError(error);
    }
  }
}