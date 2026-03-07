/**
 * Incidents API Service
 *
 * Handles incident-related API operations.
 */

import { AxiosInstance } from 'axios';
import { handleAPIError } from '../client';
import {
  Incident,
  IncidentList,
  IncidentSeverity,
  IncidentStatus,
  IncidentPhase,
  IncidentNote,
  IncidentTimelineEntry,
} from '../models';

/** Options for listing incidents */
export interface IncidentListOptions {
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

/** Options for creating an incident */
export interface IncidentCreateOptions {
  title: string;
  description: string;
  severity: IncidentSeverity;
  hosts?: string[];
  detections?: string[];
  tags?: string[];
  assignee?: string;
}

/** Options for updating an incident */
export interface IncidentUpdateOptions {
  title?: string;
  description?: string;
  severity?: IncidentSeverity;
  tags?: string[];
}

/** Options for updating incident status */
export interface IncidentUpdateStatusOptions {
  status: IncidentStatus;
  notes?: string;
}

/** Options for updating incident phase */
export interface IncidentUpdatePhaseOptions {
  phase: IncidentPhase;
  notes?: string;
}

/** Options for closing an incident */
export interface IncidentCloseOptions {
  resolution: string;
  lessonsLearned?: string;
}

/** Incidents API service */
export class IncidentsService {
  private readonly client: AxiosInstance;
  private readonly basePath = '/incidents';

  constructor(client: AxiosInstance) {
    this.client = client;
  }

  /**
   * List incidents with optional filtering
   */
  async list(options?: IncidentListOptions): Promise<IncidentList> {
    try {
      const params: Record<string, string | number> = {};

      if (options?.query) params.query = options.query;
      if (options?.severity) params.severity = options.severity;
      if (options?.status) params.status = options.status;
      if (options?.phase) params.phase = options.phase;
      if (options?.assignee) params.assignee = options.assignee;
      if (options?.timeRange) params.time_range = options.timeRange;
      if (options?.sortBy) params.sort_by = options.sortBy;
      if (options?.sortOrder) params.sort_order = options.sortOrder;
      if (options?.limit) params.limit = options.limit;
      if (options?.offset) params.offset = options.offset;

      if (!params.limit) params.limit = 50;

      const response = await this.client.get<IncidentList>(this.basePath, { params });
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get a specific incident by ID
   */
  async get(incidentId: string): Promise<Incident> {
    try {
      const response = await this.client.get<Incident>(
        `${this.basePath}/${incidentId}`
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Create a new incident
   */
  async create(options: IncidentCreateOptions): Promise<Incident> {
    try {
      const response = await this.client.post<Incident>(this.basePath, options);
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Update an incident
   */
  async update(incidentId: string, options: IncidentUpdateOptions): Promise<Incident> {
    try {
      const response = await this.client.patch<Incident>(
        `${this.basePath}/${incidentId}`,
        options
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Update the status of an incident
   */
  async updateStatus(
    incidentId: string,
    options: IncidentUpdateStatusOptions
  ): Promise<Incident> {
    try {
      const response = await this.client.patch<Incident>(
        `${this.basePath}/${incidentId}/status`,
        options
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Update the phase of an incident
   */
  async updatePhase(
    incidentId: string,
    options: IncidentUpdatePhaseOptions
  ): Promise<Incident> {
    try {
      const response = await this.client.patch<Incident>(
        `${this.basePath}/${incidentId}/phase`,
        options
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Assign an incident to a user
   */
  async assign(incidentId: string, assignee: string): Promise<Incident> {
    try {
      const response = await this.client.patch<Incident>(
        `${this.basePath}/${incidentId}/assign`,
        { assignee }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Add a note to an incident
   */
  async addNote(
    incidentId: string,
    note: string,
    visibility?: string
  ): Promise<IncidentNote> {
    try {
      const response = await this.client.post<IncidentNote>(
        `${this.basePath}/${incidentId}/notes`,
        { content: note, visibility: visibility ?? 'team' }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * List notes for an incident
   */
  async listNotes(incidentId: string, limit?: number): Promise<IncidentNote[]> {
    try {
      const params: Record<string, number> = {};
      if (limit) params.limit = limit;

      const response = await this.client.get<{ notes: IncidentNote[] }>(
        `${this.basePath}/${incidentId}/notes`,
        { params }
      );
      return response.data.notes;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get the timeline for an incident
   */
  async getTimeline(incidentId: string): Promise<IncidentTimelineEntry[]> {
    try {
      const response = await this.client.get<{ entries: IncidentTimelineEntry[] }>(
        `${this.basePath}/${incidentId}/timeline`
      );
      return response.data.entries;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Add hosts to an incident
   */
  async addHosts(incidentId: string, hostIds: string[]): Promise<Incident> {
    try {
      const response = await this.client.post<Incident>(
        `${this.basePath}/${incidentId}/hosts`,
        { host_ids: hostIds }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Remove hosts from an incident
   */
  async removeHosts(incidentId: string, hostIds: string[]): Promise<Incident> {
    try {
      const response = await this.client.delete<Incident>(
        `${this.basePath}/${incidentId}/hosts`,
        { data: { host_ids: hostIds } }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Add detections to an incident
   */
  async addDetections(incidentId: string, detectionIds: string[]): Promise<Incident> {
    try {
      const response = await this.client.post<Incident>(
        `${this.basePath}/${incidentId}/detections`,
        { detection_ids: detectionIds }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Remove detections from an incident
   */
  async removeDetections(incidentId: string, detectionIds: string[]): Promise<Incident> {
    try {
      const response = await this.client.delete<Incident>(
        `${this.basePath}/${incidentId}/detections`,
        { data: { detection_ids: detectionIds } }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Close an incident
   */
  async close(incidentId: string, options: IncidentCloseOptions): Promise<Incident> {
    try {
      const response = await this.client.post<Incident>(
        `${this.basePath}/${incidentId}/close`,
        options
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Search for incidents matching a query
   */
  async search(query: string, limit?: number): Promise<IncidentList> {
    try {
      const params: Record<string, string | number> = { query };
      if (limit) params.limit = limit;
      else params.limit = 50;

      const response = await this.client.get<IncidentList>(
        `${this.basePath}/search`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }
}