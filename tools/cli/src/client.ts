import axios, { AxiosInstance, AxiosError } from 'axios';
import ConfigStore from 'configstore';

export interface VSentinelConfig {
    apiUrl: string;
    apiKey: string;
    timeout: number;
}

export interface IOC {
    id: string;
    type: string;
    value: string;
    confidence: 'low' | 'medium' | 'high';
    threat_actor?: string;
    campaign?: string;
    malware_family?: string;
    first_seen?: string;
    last_seen?: string;
    tags?: string[];
    references?: string[];
}

export interface IOCSearchResult {
    ioc: IOC;
    is_malicious: boolean;
    threat_actor?: {
        name: string;
        aliases: string[];
        country?: string;
        motivation?: string;
    };
    related_iocs?: IOC[];
}

export interface Detection {
    id: string;
    title: string;
    description: string;
    severity: 'critical' | 'high' | 'medium' | 'low' | 'informational';
    status: 'new' | 'in_progress' | 'resolved' | 'false_positive' | 'ignored';
    host_id?: string;
    techniques?: string[];
    created_at: string;
    updated_at: string;
}

export interface Host {
    id: string;
    hostname: string;
    platform: 'windows' | 'linux' | 'macos' | 'unknown';
    state: 'online' | 'offline' | 'isolated' | 'unknown';
    ip_addresses: string[];
    is_isolated: boolean;
    tags?: string[];
    last_seen: string;
}

export interface Incident {
    id: string;
    title: string;
    description: string;
    severity: 'critical' | 'high' | 'medium' | 'low';
    status: 'new' | 'in_progress' | 'on_hold' | 'resolved' | 'closed' | 'false_positive';
    phase?: string;
    hosts?: string[];
    assignee?: string;
    created_at: string;
    updated_at: string;
}

export interface ThreatActor {
    name: string;
    aliases: string[];
    country?: string;
    motivation?: string;
    mitre_id?: string;
    description?: string;
    campaigns?: string[];
    malware?: string[];
    techniques?: string[];
}

export interface MITRETechnique {
    id: string;
    name: string;
    tactics?: string[];
    platforms?: string[];
    description?: string;
}

export class VSentinelClient {
    private client: AxiosInstance;
    private config: ConfigStore;

    constructor(customConfig?: Partial<VSentinelConfig>) {
        this.config = new ConfigStore('vsentinel');
        
        const apiUrl = customConfig?.apiUrl || this.config.get('apiUrl') || 'https://api.vantis.ai/v1';
        const apiKey = customConfig?.apiKey || this.config.get('apiKey') || '';
        const timeout = customConfig?.timeout || this.config.get('timeout') || 30;

        this.client = axios.create({
            baseURL: apiUrl,
            timeout: timeout * 1000,
            headers: {
                'Content-Type': 'application/json',
                ...(apiKey && { 'Authorization': `Bearer ${apiKey}` })
            }
        });
    }

    // IOC Operations
    async lookupIOC(value: string): Promise<IOCSearchResult> {
        try {
            const response = await this.client.get('/threat-intel/lookup', { params: { value } });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async scanContent(content: string, types?: string[], minConfidence?: string): Promise<{
        iocs_found: number;
        iocs: IOCSearchResult[];
        summary: Record<string, number>;
    }> {
        try {
            const response = await this.client.post('/threat-intel/scan', { 
                content,
                types,
                min_confidence: minConfidence
            });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getIOCs(limit: number = 100, offset: number = 0): Promise<{ iocs: IOC[]; total: number }> {
        try {
            const response = await this.client.get('/threat-intel/iocs', { params: { limit, offset } });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Detections Operations
    async getDetections(options: {
        limit?: number;
        status?: string;
        severity?: string;
        host_id?: string;
    } = {}): Promise<{ detections: Detection[]; total: number }> {
        try {
            const response = await this.client.get('/detections', { params: options });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getDetection(id: string): Promise<Detection> {
        try {
            const response = await this.client.get(`/detections/${id}`);
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async updateDetectionStatus(id: string, status: string, notes?: string): Promise<Detection> {
        try {
            const response = await this.client.patch(`/detections/${id}/status`, { status, notes });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Hosts Operations
    async getHosts(options: {
        limit?: number;
        platform?: string;
        state?: string;
    } = {}): Promise<{ hosts: Host[]; total: number }> {
        try {
            const response = await this.client.get('/hosts', { params: options });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getHost(id: string): Promise<Host> {
        try {
            const response = await this.client.get(`/hosts/${id}`);
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async isolateHost(id: string, reason: string): Promise<Host> {
        try {
            const response = await this.client.post(`/hosts/${id}/isolate`, { reason });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async unisolateHost(id: string): Promise<Host> {
        try {
            const response = await this.client.post(`/hosts/${id}/unisolate`);
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Incidents Operations
    async getIncidents(options: {
        limit?: number;
        status?: string;
        severity?: string;
    } = {}): Promise<{ incidents: Incident[]; total: number }> {
        try {
            const response = await this.client.get('/incidents', { params: options });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getIncident(id: string): Promise<Incident> {
        try {
            const response = await this.client.get(`/incidents/${id}`);
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async createIncident(data: {
        title: string;
        description: string;
        severity: string;
        hosts?: string[];
    }): Promise<Incident> {
        try {
            const response = await this.client.post('/incidents', data);
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async updateIncidentStatus(id: string, status: string): Promise<Incident> {
        try {
            const response = await this.client.patch(`/incidents/${id}/status`, { status });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Threat Intelligence Operations
    async searchThreatActor(name: string): Promise<ThreatActor | null> {
        try {
            const response = await this.client.get('/threat-intel/actors/search', { params: { name } });
            return response.data;
        } catch (error) {
            if (axios.isAxiosError(error) && error.response?.status === 404) {
                return null;
            }
            throw this.handleError(error);
        }
    }

    async getThreatActors(limit: number = 50): Promise<{ actors: ThreatActor[]; total: number }> {
        try {
            const response = await this.client.get('/threat-intel/actors', { params: { limit } });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getMITRETechniques(techniqueId?: string): Promise<{ techniques: MITRETechnique[] }> {
        try {
            const params = techniqueId ? { id: techniqueId } : {};
            const response = await this.client.get('/threat-intel/mitre/techniques', { params });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    // Configuration
    getConfig(): { apiUrl: string; apiKey: string; timeout: number } {
        return {
            apiUrl: this.config.get('apiUrl') || 'https://api.vantis.ai/v1',
            apiKey: this.config.get('apiKey') || '',
            timeout: this.config.get('timeout') || 30
        };
    }

    setConfig(key: string, value: string): void {
        this.config.set(key, value);
    }

    clearConfig(): void {
        this.config.clear();
    }

    private handleError(error: unknown): Error {
        if (axios.isAxiosError(error)) {
            const axiosError = error as AxiosError<any>;
            if (axiosError.response) {
                const message = axiosError.response.data?.message || axiosError.response.data?.error || 'API Error';
                return new Error(`V-Sentinel API Error: ${message}`);
            } else if (axiosError.request) {
                return new Error('V-Sentinel API: No response received. Check your network connection.');
            }
        }
        return error instanceof Error ? error : new Error(String(error));
    }
}