import * as vscode from 'vscode';
import { VSentinelClient, Detection } from '../client';

export class DetectionsProvider implements vscode.TreeDataProvider<DetectionItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<DetectionItem | undefined | null | void> = new vscode.EventEmitter<DetectionItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<DetectionItem | undefined | null | void> = this._onDidChangeTreeData.event;

    private detections: Detection[] = [];

    constructor(private client: VSentinelClient) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: DetectionItem): vscode.TreeItem {
        return element;
    }

    async getChildren(element?: DetectionItem): Promise<DetectionItem[]> {
        if (element) {
            return element.children || [];
        }

        try {
            const result = await this.client.getDetections(50, 'new,in_progress');
            this.detections = result.detections;
            return this.detections.map(detection => new DetectionItem(detection));
        } catch (error) {
            console.error('Failed to fetch detections:', error);
            return [];
        }
    }
}

class DetectionItem extends vscode.TreeItem {
    children: DetectionItem[] = [];

    constructor(public readonly detection: Detection) {
        super(detection.title, vscode.TreeItemCollapsibleState.None);

        this.label = detection.title;
        this.description = `${detection.severity} • ${detection.status}`;
        this.tooltip = this.formatTooltip(detection);

        this.iconPath = this.getSeverityIcon(detection.severity);

        this.contextValue = 'detection';
    }

    private formatTooltip(detection: Detection): string {
        const lines = [
            `**${detection.title}**`,
            `Severity: ${detection.severity}`,
            `Status: ${detection.status}`,
            detection.host_id ? `Host: ${detection.host_id}` : '',
            detection.description ? `\n${detection.description}` : '',
            detection.techniques && detection.techniques.length > 0 
                ? `\nMITRE ATT&CK: ${detection.techniques.join(', ')}` : ''
        ];
        return lines.filter(Boolean).join('\n');
    }

    private getSeverityIcon(severity: string): vscode.ThemeIcon {
        switch (severity.toLowerCase()) {
            case 'critical':
                return new vscode.ThemeIcon('error', new vscode.ThemeColor('errorForeground'));
            case 'high':
                return new vscode.ThemeIcon('warning', new vscode.ThemeColor('editorWarning.foreground'));
            case 'medium':
                return new vscode.ThemeIcon('info', new vscode.ThemeColor('editorInfo.foreground'));
            case 'low':
            case 'informational':
            default:
                return new vscode.ThemeIcon('circle-outline');
        }
    }
}