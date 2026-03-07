import * as vscode from 'vscode';
import { VSentinelClient } from '../client';

interface IOCMatch {
    value: string;
    type: string;
    range: vscode.Range;
}

export class IOCDecoProvider {
    private decorationTypes: Map<string, vscode.TextEditorDecorationType> = new Map();
    private timeout: NodeJS.Timeout | undefined;

    constructor(private client: VSentinelClient) {
        this.initializeDecorations();
    }

    private initializeDecorations(): void {
        const config = vscode.workspace.getConfiguration('vsentinel');
        const colors = config.get<any>('iocColors', {});

        this.decorationTypes.set('ip', vscode.window.createTextEditorDecorationType({
            backgroundColor: colors.ip || '#ff6b6b',
            color: '#000000',
            borderRadius: '3px',
            cursor: 'pointer',
            after: {
                contentText: ' 🔍',
                color: '#666666'
            }
        }));

        this.decorationTypes.set('domain', vscode.window.createTextEditorDecorationType({
            backgroundColor: colors.domain || '#4ecdc4',
            color: '#000000',
            borderRadius: '3px'
        }));

        this.decorationTypes.set('url', vscode.window.createTextEditorDecorationType({
            backgroundColor: colors.url || '#ffe66d',
            color: '#000000',
            borderRadius: '3px'
        }));

        this.decorationTypes.set('hash', vscode.window.createTextEditorDecorationType({
            backgroundColor: colors.hash || '#95e1d3',
            color: '#000000',
            borderRadius: '3px'
        }));

        this.decorationTypes.set('email', vscode.window.createTextEditorDecorationType({
            backgroundColor: colors.email || '#dfe6e9',
            color: '#000000',
            borderRadius: '3px'
        }));
    }

    async decorate(editor: vscode.TextEditor): Promise<void> {
        // Clear any pending decoration
        if (this.timeout) {
            clearTimeout(this.timeout);
        }

        // Debounce decoration
        this.timeout = setTimeout(async () => {
            try {
                const text = editor.document.getText();
                const iocs = await this.findIOCsInText(text);

                // Clear existing decorations
                for (const decorationType of this.decorationTypes.values()) {
                    editor.setDecorations(decorationType, []);
                }

                // Apply new decorations
                const iocsByType = this.groupIOCsByType(iocs);
                for (const [type, matches] of iocsByType) {
                    const decorationType = this.decorationTypes.get(type);
                    if (decorationType) {
                        const decorations: vscode.DecorationOptions[] = matches.map(match => ({
                            range: match.range,
                            hoverMessage: `**IOC Type:** ${type}\n**Value:** ${match.value}\n\nClick to lookup`
                        }));
                        editor.setDecorations(decorationType, decorations);
                    }
                }
            } catch (error) {
                console.error('Failed to decorate IOCs:', error);
            }
        }, 500);
    }

    private async findIOCsInText(text: string): Promise<IOCMatch[]> {
        const iocs: IOCMatch[] = [];
        const lines = text.split('\n');

        for (let lineIndex = 0; lineIndex < lines.length; lineIndex++) {
            const line = lines[lineIndex];

            // Find IPs
            const ipRegex = /\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b/g;
            let match;
            while ((match = ipRegex.exec(line)) !== null) {
                iocs.push({
                    value: match[0],
                    type: 'ip',
                    range: new vscode.Range(
                        new vscode.Position(lineIndex, match.index),
                        new vscode.Position(lineIndex, match.index + match[0].length)
                    )
                });
            }

            // Find domains (simplified regex)
            const domainRegex = /\b(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]{2,}\b/g;
            while ((match = domainRegex.exec(line)) !== null) {
                // Skip if it looks like an IP
                if (!/^\d+\.\d+\.\d+\.\d+$/.test(match[0])) {
                    iocs.push({
                        value: match[0],
                        type: 'domain',
                        range: new vscode.Range(
                            new vscode.Position(lineIndex, match.index),
                            new vscode.Position(lineIndex, match.index + match[0].length)
                        )
                    });
                }
            }

            // Find SHA256 hashes
            const sha256Regex = /\b[a-fA-F0-9]{64}\b/g;
            while ((match = sha256Regex.exec(line)) !== null) {
                iocs.push({
                    value: match[0],
                    type: 'hash',
                    range: new vscode.Range(
                        new vscode.Position(lineIndex, match.index),
                        new vscode.Position(lineIndex, match.index + match[0].length)
                    )
                });
            }

            // Find SHA1 hashes
            const sha1Regex = /\b[a-fA-F0-9]{40}\b/g;
            while ((match = sha1Regex.exec(line)) !== null) {
                iocs.push({
                    value: match[0],
                    type: 'hash',
                    range: new vscode.Range(
                        new vscode.Position(lineIndex, match.index),
                        new vscode.Position(lineIndex, match.index + match[0].length)
                    )
                });
            }

            // Find MD5 hashes
            const md5Regex = /\b[a-fA-F0-9]{32}\b/g;
            while ((match = md5Regex.exec(line)) !== null) {
                iocs.push({
                    value: match[0],
                    type: 'hash',
                    range: new vscode.Range(
                        new vscode.Position(lineIndex, match.index),
                        new vscode.Position(lineIndex, match.index + match[0].length)
                    )
                });
            }

            // Find URLs
            const urlRegex = /https?:\/\/[^\s<>"{}|\\^`\[\]]+/g;
            while ((match = urlRegex.exec(line)) !== null) {
                iocs.push({
                    value: match[0],
                    type: 'url',
                    range: new vscode.Range(
                        new vscode.Position(lineIndex, match.index),
                        new vscode.Position(lineIndex, match.index + match[0].length)
                    )
                });
            }

            // Find emails
            const emailRegex = /\b[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}\b/g;
            while ((match = emailRegex.exec(line)) !== null) {
                iocs.push({
                    value: match[0],
                    type: 'email',
                    range: new vscode.Range(
                        new vscode.Position(lineIndex, match.index),
                        new vscode.Position(lineIndex, match.index + match[0].length)
                    )
                });
            }
        }

        return iocs;
    }

    private groupIOCsByType(iocs: IOCMatch[]): Map<string, IOCMatch[]> {
        const grouped = new Map<string, IOCMatch[]>();
        for (const ioc of iocs) {
            const existing = grouped.get(ioc.type) || [];
            existing.push(ioc);
            grouped.set(ioc.type, existing);
        }
        return grouped;
    }

    dispose(): void {
        for (const decorationType of this.decorationTypes.values()) {
            decorationType.dispose();
        }
    }
}