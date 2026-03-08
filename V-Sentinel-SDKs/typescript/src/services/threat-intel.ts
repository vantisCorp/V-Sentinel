/**
 * Threat Intelligence API Service
 *
 * Handles threat intelligence API operations.
 */

import { AxiosInstance } from 'axios';
import { handleAPIError } from '../client';
import {
  IOC,
  IOCList,
  IOCType,
  IOCConfidence,
  IOCCheckResult,
  ThreatActor,
  ThreatActorList,
  Campaign,
  CampaignList,
} from '../models';

/** Options for listing IOCs */
export interface IOCListOptions {
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

/** Options for adding an IOC */
export interface AddIOCOptions {
  value: string;
  type: IOCType;
  confidence?: IOCConfidence;
  threatActor?: string;
  campaign?: string;
  malwareFamily?: string;
  tags?: string[];
  description?: string;
  references?: string[];
  expirationDays?: number;
}

/** Options for updating an IOC */
export interface UpdateIOCOptions {
  confidence?: IOCConfidence;
  threatActor?: string;
  campaign?: string;
  malwareFamily?: string;
  tags?: string[];
  description?: string;
}

/** Options for listing threat actors */
export interface ThreatActorListOptions {
  country?: string;
  motivation?: string;
  tags?: string[];
  limit?: number;
  offset?: number;
}

/** Options for listing campaigns */
export interface CampaignListOptions {
  threatActor?: string;
  status?: string;
  timeRange?: string;
  limit?: number;
  offset?: number;
}

/** Options for exporting IOCs */
export interface ExportOptions {
  type?: IOCType;
  threatActor?: string;
  malwareFamily?: string;
  tags?: string[];
  format?: 'json' | 'stix' | 'csv' | 'txt';
  includeExpired?: boolean;
}

/** Threat Intelligence API service */
export class ThreatIntelService {
  private readonly client: AxiosInstance;
  private readonly basePath = '/threat-intel';

  constructor(client: AxiosInstance) {
    this.client = client;
  }

  // ==================== IOC Operations ====================

  /**
   * List IOCs with optional filtering
   */
  async listIOCs(options?: IOCListOptions): Promise<IOCList> {
    try {
      const params: Record<string, string | number> = {};

      if (options?.type) params.type = options.type;
      if (options?.confidence) params.confidence = options.confidence;
      if (options?.threatActor) params.threat_actor = options.threatActor;
      if (options?.malwareFamily) params.malware_family = options.malwareFamily;
      if (options?.tags?.length) params.tags = options.tags.join(',');
      if (options?.timeRange) params.time_range = options.timeRange;
      if (options?.sortBy) params.sort_by = options.sortBy;
      if (options?.sortOrder) params.sort_order = options.sortOrder;
      if (options?.limit) params.limit = options.limit;
      if (options?.offset) params.offset = options.offset;

      if (!params.limit) params.limit = 50;

      const response = await this.client.get<IOCList>(`${this.basePath}/iocs`, { params });
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get a specific IOC by ID
   */
  async getIOC(iocId: string): Promise<IOC> {
    try {
      const response = await this.client.get<IOC>(
        `${this.basePath}/iocs/${iocId}`
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Check if an indicator is malicious
   */
  async checkIOC(value: string, type?: IOCType): Promise<IOCCheckResult> {
    try {
      const params: Record<string, string> = { value };
      if (type) params.type = type;

      const response = await this.client.get<IOCCheckResult>(
        `${this.basePath}/check`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Check multiple IOCs in bulk
   */
  async checkIOCsBulk(values: string[], type?: IOCType): Promise<IOCCheckResult[]> {
    try {
      const body: Record<string, unknown> = { values };
      if (type) body.type = type;

      const response = await this.client.post<{ results: IOCCheckResult[] }>(
        `${this.basePath}/check/bulk`,
        body
      );
      return response.data.results;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Add a new IOC to the database
   */
  async addIOC(options: AddIOCOptions): Promise<IOC> {
    try {
      const body: Record<string, unknown> = {
        value: options.value,
        type: options.type,
        confidence: options.confidence ?? IOCConfidence.Medium,
      };

      if (options.threatActor) body.threat_actor = options.threatActor;
      if (options.campaign) body.campaign = options.campaign;
      if (options.malwareFamily) body.malware_family = options.malwareFamily;
      if (options.tags?.length) body.tags = options.tags;
      if (options.description) body.description = options.description;
      if (options.references?.length) body.references = options.references;
      if (options.expirationDays) body.expiration_days = options.expirationDays;

      const response = await this.client.post<IOC>(`${this.basePath}/iocs`, body);
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Update an existing IOC
   */
  async updateIOC(iocId: string, options: UpdateIOCOptions): Promise<IOC> {
    try {
      const body: Record<string, unknown> = {};

      if (options.confidence) body.confidence = options.confidence;
      if (options.threatActor) body.threat_actor = options.threatActor;
      if (options.campaign) body.campaign = options.campaign;
      if (options.malwareFamily) body.malware_family = options.malwareFamily;
      if (options.tags) body.tags = options.tags;
      if (options.description) body.description = options.description;

      const response = await this.client.patch<IOC>(
        `${this.basePath}/iocs/${iocId}`,
        body
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Delete an IOC
   */
  async deleteIOC(iocId: string): Promise<void> {
    try {
      await this.client.delete(`${this.basePath}/iocs/${iocId}`);
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Search for IOCs matching a query
   */
  async searchIOCs(query: string, limit?: number): Promise<IOCList> {
    try {
      const params: Record<string, string | number> = { query };
      if (limit) params.limit = limit;
      else params.limit = 50;

      const response = await this.client.get<IOCList>(
        `${this.basePath}/iocs/search`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  // ==================== Threat Actor Operations ====================

  /**
   * List threat actors with optional filtering
   */
  async listThreatActors(options?: ThreatActorListOptions): Promise<ThreatActorList> {
    try {
      const params: Record<string, string | number> = {};

      if (options?.country) params.country = options.country;
      if (options?.motivation) params.motivation = options.motivation;
      if (options?.tags?.length) params.tags = options.tags.join(',');
      if (options?.limit) params.limit = options.limit;
      if (options?.offset) params.offset = options.offset;

      if (!params.limit) params.limit = 50;

      const response = await this.client.get<ThreatActorList>(
        `${this.basePath}/actors`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get a specific threat actor by ID or name
   */
  async getThreatActor(actorId: string): Promise<ThreatActor> {
    try {
      const response = await this.client.get<ThreatActor>(
        `${this.basePath}/actors/${actorId}`
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get IOCs associated with a threat actor
   */
  async getThreatActorIOCs(actorId: string, limit?: number): Promise<IOCList> {
    try {
      const params: Record<string, number> = {};
      if (limit) params.limit = limit;
      else params.limit = 100;

      const response = await this.client.get<IOCList>(
        `${this.basePath}/actors/${actorId}/iocs`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  // ==================== Campaign Operations ====================

  /**
   * List campaigns with optional filtering
   */
  async listCampaigns(options?: CampaignListOptions): Promise<CampaignList> {
    try {
      const params: Record<string, string | number> = {};

      if (options?.threatActor) params.threat_actor = options.threatActor;
      if (options?.status) params.status = options.status;
      if (options?.timeRange) params.time_range = options.timeRange;
      if (options?.limit) params.limit = options.limit;
      if (options?.offset) params.offset = options.offset;

      if (!params.limit) params.limit = 50;

      const response = await this.client.get<CampaignList>(
        `${this.basePath}/campaigns`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get a specific campaign by ID
   */
  async getCampaign(campaignId: string): Promise<Campaign> {
    try {
      const response = await this.client.get<Campaign>(
        `${this.basePath}/campaigns/${campaignId}`
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Get IOCs associated with a campaign
   */
  async getCampaignIOCs(campaignId: string, limit?: number): Promise<IOCList> {
    try {
      const params: Record<string, number> = {};
      if (limit) params.limit = limit;
      else params.limit = 100;

      const response = await this.client.get<IOCList>(
        `${this.basePath}/campaigns/${campaignId}/iocs`,
        { params }
      );
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  // ==================== Export Operations ====================

  /**
   * Export IOCs in the specified format
   */
  async exportIOCs(options?: ExportOptions): Promise<string> {
    try {
      const params: Record<string, string | boolean> = {
        format: options?.format ?? 'json',
      };

      if (options?.type) params.type = options.type;
      if (options?.threatActor) params.threat_actor = options.threatActor;
      if (options?.malwareFamily) params.malware_family = options.malwareFamily;
      if (options?.tags?.length) params.tags = options.tags.join(',');
      if (options?.includeExpired) params.include_expired = options.includeExpired;

      const response = await this.client.get<string>(`${this.basePath}/export`, {
        params,
        responseType: 'text',
      });
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }

  /**
   * Export IOCs formatted for a specific SIEM
   */
  async exportForSIEM(
    siemType: 'splunk' | 'elastic' | 'sentinel' | 'qradar',
    iocTypes?: IOCType[],
    threatActor?: string
  ): Promise<string> {
    try {
      const params: Record<string, string> = { siem: siemType };

      if (iocTypes?.length) params.types = iocTypes.join(',');
      if (threatActor) params.threat_actor = threatActor;

      const response = await this.client.get<string>(`${this.basePath}/export/siem`, {
        params,
        responseType: 'text',
      });
      return response.data;
    } catch (error) {
      handleAPIError(error);
    }
  }
}