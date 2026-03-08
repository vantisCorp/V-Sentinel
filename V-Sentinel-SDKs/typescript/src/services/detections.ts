/**
 * Detections API Service
 *
 * Handles detection-related API operations.
 */

import { AxiosInstance } from 'axios';
import { handleAPIError } from '../client';
import {
  Detection,
  DetectionList,
  DetectionSeverity,
  DetectionStatus,
  DetectionNote,
} from '../models';

/** Options for listing detections */
export interface DetectionListOptions {
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

/** Options for updating detection status */
export interface UpdateStatusOptions {
  status: DetectionStatus;
  notes?: string;
}

/** Detections API service */
export class DetectionsService {
  private readonly client: AxiosInstance;
  private readonly basePath = '/detections';

  constructor(client: AxiosInstance) {
    this.client = client;
  }

  /**
   * List detections with optional filtering
   */
  async list(options?: DetectionListOptions): Promise<DetectionList> {
    try {
      const params: Record<string, string | number> = {};

      if (options?.query) params.query = options.query;
      if (options?.severity) params.severity = options.severity;
      if (options?.status) params.status = options.status;
      if (options?.hostId) params.host_id = options.hostId;
      if (options?.timeRange) params.time_range = options.timeRange;
      if (options?.sortBy) params.sort_by = options.sortBy;
      if (options?.sortOrder) params.sort_order = options.sortOrder;
      if (options?.limit) params.limit = options.limit;
      if (options?.offset) params.offset = options.offset;

      if (!params.limit) params.limit = 50;

      const response = await this.client.get<DetectionList>(this.basePath, { params });
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get a specific detection by ID
   */
  async get(detectionId: string): Promise<Detection> {
    try {
      const response = await this.client.get<Detection>(
        `${this.basePath}/${detectionId}`
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Search for detections matching a query
   */
  async search(query: string, limit?: number): Promise<DetectionList> {
    try {
      const params: Record<string, string | number> = { query };
      if (limit) params.limit = limit;
      else params.limit = 50;

      const response = await this.client.get<DetectionList>(
        `${this.basePath}/search`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Update the status of a detection
   */
  async updateStatus(
    detectionId: string,
    options: UpdateStatusOptions
  ): Promise<Detection> {
    try {
      const response = await this.client.patch<Detection>(
        `${this.basePath}/${detectionId}/status`,
        options
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Assign a detection to a user
   */
  async assign(detectionId: string, assignee: string): Promise<Detection> {
    try {
      const response = await this.client.patch<Detection>(
        `${this.basePath}/${detectionId}/assign`,
        { assignee }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Add a note to a detection
   */
  async addNote(detectionId: string, note: string): Promise<DetectionNote> {
    try {
      const response = await this.client.post<DetectionNote>(
        `${this.basePath}/${detectionId}/notes`,
        { content: note }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * List notes for a detection
   */
  async listNotes(detectionId: string, limit?: number): Promise<DetectionNote[]> {
    try {
      const params: Record<string, number> = {};
      if (limit) params.limit = limit;

      const response = await this.client.get<{ notes: DetectionNote[] }>(
        `${this.basePath}/${detectionId}/notes`,
        { params }
      );
      return response.data.notes;
    } catch (error) {
      handleAPIError(error);
    }
  }
}