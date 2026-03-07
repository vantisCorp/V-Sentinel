import * as vscode from 'vscode';
import { VSentinelClient, IOC } from '../client';

export class IOCProvider implements vscode.TreeDataProvider<IOCItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<IOCItem | undefined | null | void> = new vscode.EventEmitter<IOCItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<IOCItem | undefined | null | void> = this._onDidChangeTreeData.event;

    private iocs: IOC[] = [];

    constructor(private client: VSentinelClient) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: IOCItem): vscode.TreeItem {
        return element;
    }

    async getChildren(element?: IOCItem): Promise<IOCItem[]> {
        if (element) {
            return element.children || [];
        }

        try {
            const result = await this.client.getRecentIOCs(24);
            this.iocs = result;
            return this.iocs.map(ioc => new IOCItem(ioc));
        } catch (error) {
            console.error('Failed to fetch IOCs:', error);
            return [];
        }
    }
}

class IOCItem extends vscode.TreeItem {
    children: IOCItem[] = [];

    constructor(public readonly ioc: IOC) {
        super(ioc.value, vscode.TreeItemCollapsibleState.None);

        this.label = ioc.value;
        this.description = `${ioc.type} • ${ioc.confidence || 'unknown'}`;
        this.tooltip = `${ioc.type}: ${ioc.value}\nConfidence: ${ioc.confidence || 'N/A'}\nThreat Actor: ${ioc.threat_actor || 'N/A'}`;

        this.iconPath = this.getIcon(ioc.type);

        this.contextValue = 'ioc';
        this.command = {
            command: 'vsentinel.lookupIOC',
            title: 'Lookup IOC',
            arguments: [ioc.value]
        };
    }

    private getIcon(type: string): vscode.ThemeIcon {
        switch (type.toLowerCase()) {
            case 'ip':
                return new vscode.ThemeIcon('globe');
            case 'domain':
                return new vscode.ThemeIcon('link');
            case 'url':
                return new vscode.ThemeIcon('link-external');
            case 'hash_md5':
            case 'hash_sha1':
            case 'hash_sha256':
            case 'hash':
                return new vscode.ThemeIcon('fingerprint');
            case 'email':
                return new vscode.ThemeIcon('mail');
            default:
                return new vscode.ThemeIcon('info');
        }
    }
}