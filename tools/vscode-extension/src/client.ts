import axios, { AxiosInstance, AxiosError } from 'axios';

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
    indicators?: IOC[];
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
    detection?: string;
}

export interface ScanResult {
    iocs_found: number;
    iocs: IOCSearchResult[];
    summary: {
        [key: string]: number;
    };
}

export class VSentinelClient {
    private client: AxiosInstance;

    constructor(baseUrl: string, apiKey: string, timeout: number = 30) {
        this.client = axios.create({
            baseURL: baseUrl,
            timeout: timeout * 1000,
            headers: {
                'Content-Type': 'application/json',
                ...(apiKey && { 'Authorization': `Bearer ${apiKey}` })
            }
        });
    }

    async lookupIOC(value: string): Promise<IOCSearchResult> {
        try {
            const response = await this.client.get('/threat-intel/lookup', {
                params: { value }
            });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async scanContent(content: string): Promise<ScanResult> {
        try {
            const response = await this.client.post('/threat-intel/scan', { content });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async addIOC(ioc: { value: string; type: string; threat_actor?: string; campaign?: string }): Promise<IOC> {
        try {
            const response = await this.client.post('/threat-intel/iocs', ioc);
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getIOCs(limit: number = 100, offset: number = 0): Promise<{ iocs: IOC[]; total: number }> {
        try {
            const response = await this.client.get('/threat-intel/iocs', {
                params: { limit, offset }
            });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getDetections(limit: number = 50, status?: string): Promise<{ detections: Detection[]; total: number }> {
        try {
            const params: any = { limit };
            if (status) {
                params.status = status;
            }
            const response = await this.client.get('/detections', { params });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async searchThreatActor(name: string): Promise<ThreatActor | null> {
        try {
            const response = await this.client.get('/threat-intel/actors/search', {
                params: { name }
            });
            return response.data;
        } catch (error) {
            if (axios.isAxiosError(error) && error.response?.status === 404) {
                return null;
            }
            throw this.handleError(error);
        }
    }

    async getMITRETechniques(techniqueId?: string): Promise<MITRETechnique[]> {
        try {
            const params = techniqueId ? { id: techniqueId } : {};
            const response = await this.client.get('/threat-intel/mitre/techniques', { params });
            return response.data.techniques || [];
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async checkHash(hash: string): Promise<IOCSearchResult> {
        try {
            const response = await this.client.get('/threat-intel/hash/check', {
                params: { hash }
            });
            return response.data;
        } catch (error) {
            throw this.handleError(error);
        }
    }

    async getRecentIOCs(hours: number = 24): Promise<IOC[]> {
        try {
            const response = await this.client.get('/threat-intel/iocs/recent', {
                params: { hours }
            });
            return response.data.iocs || [];
        } catch (error) {
            throw this.handleError(error);
        }
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