import * as vscode from 'vscode';
import { VSentinelClient } from './client';
import { IOCProvider } from './providers/iocProvider';
import { DetectionsProvider } from './providers/detectionsProvider';
import { IOCDecoProvider } from './providers/iocDecorator';
import { lookupIOC, scanFileForIOCs, checkFileHash } from './commands';
import { registerSyntaxHighlighting } from './syntax/highlighter';

let client: VSentinelClient;
let iocProvider: IOCProvider;
let detectionsProvider: DetectionsProvider;

export async function activate(context: vscode.ExtensionContext): Promise<void> {
    console.log('V-Sentinel extension is now active');

    // Initialize the API client
    const config = vscode.workspace.getConfiguration('vsentinel');
    const apiUrl = config.get<string>('apiUrl', 'https://api.vantis.ai/v1');
    const apiKey = config.get<string>('apiKey', '');
    const timeout = config.get<number>('timeout', 30);

    client = new VSentinelClient(apiUrl, apiKey, timeout);

    // Initialize tree data providers
    iocProvider = new IOCProvider(client);
    detectionsProvider = new DetectionsProvider(client);

    // Register tree views
    const iocExplorerView = vscode.window.createTreeView('vsentinelExplorer', {
        treeDataProvider: iocProvider,
        showCollapseAll: true
    });

    const iocsView = vscode.window.createTreeView('vsentinelIOCs', {
        treeDataProvider: iocProvider,
        showCollapseAll: true
    });

    const detectionsView = vscode.window.createTreeView('vsentinelDetections', {
        treeDataProvider: detectionsProvider,
        showCollapseAll: true
    });

    // Register commands
    const commands = [
        vscode.commands.registerCommand('vsentinel.lookupIOC', () => lookupIOC(client)),
        vscode.commands.registerCommand('vsentinel.scanFile', (uri?: vscode.Uri) => scanFileForIOCs(client, uri)),
        vscode.commands.registerCommand('vsentinel.checkHash', () => checkFileHash(client)),
        vscode.commands.registerCommand('vsentinel.addIOC', () => addIOC(client)),
        vscode.commands.registerCommand('vsentinel.refreshExplorer', () => {
            iocProvider.refresh();
            detectionsProvider.refresh();
        }),
        vscode.commands.registerCommand('vsentinel.searchThreatActor', () => searchThreatActor(client)),
        vscode.commands.registerCommand('vsentinel.getMITRETechniques', () => getMITRETechniques(client)),
    ];

    // Register IOC decorator if enabled
    let decorator: IOCDecoProvider | undefined;
    if (config.get<boolean>('highlightIOCs', true)) {
        decorator = new IOCDecoProvider(client);
        context.subscriptions.push(
            vscode.window.onDidChangeActiveTextEditor(editor => {
                if (editor) {
                    decorator?.decorate(editor);
                }
            })
        );
    }

    // Register syntax highlighting
    registerSyntaxHighlighting(context);

    // Auto-scan on file open if enabled
    if (config.get<boolean>('autoScan', false)) {
        context.subscriptions.push(
            vscode.workspace.onDidOpenTextDocument(document => {
                scanFileForIOCs(client, document.uri);
            })
        );
    }

    // Add all subscriptions
    context.subscriptions.push(
        iocExplorerView,
        iocsView,
        detectionsView,
        ...commands
    );

    // Show welcome message
    if (!apiKey) {
        vscode.window.showWarningMessage(
            'V-Sentinel: API key not configured. Set vsentinel.apiKey in settings.',
            'Open Settings'
        ).then(selection => {
            if (selection === 'Open Settings') {
                vscode.commands.executeCommand('workbench.action.openSettings', 'vsentinel');
            }
        });
    }
}

export function deactivate(): void {
    console.log('V-Sentinel extension deactivated');
}

async function addIOC(client: VSentinelClient): Promise<void> {
    const iocValue = await vscode.window.showInputBox({
        prompt: 'Enter IOC value',
        placeHolder: 'e.g., 192.168.1.1, malicious.com, hash...'
    });

    if (!iocValue) {
        return;
    }

    const iocType = await vscode.window.showQuickPick(
        ['ip', 'domain', 'url', 'hash_md5', 'hash_sha1', 'hash_sha256', 'email'],
        { placeHolder: 'Select IOC type' }
    );

    if (!iocType) {
        return;
    }

    try {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'Adding IOC to repository...',
                cancellable: false
            },
            async () => {
                await client.addIOC({ value: iocValue, type: iocType });
            }
        );

        vscode.window.showInformationMessage(`IOC "${iocValue}" added successfully`);
        iocProvider.refresh();
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to add IOC: ${error}`);
    }
}

async function searchThreatActor(client: VSentinelClient): Promise<void> {
    const actorName = await vscode.window.showInputBox({
        prompt: 'Enter threat actor name or alias',
        placeHolder: 'e.g., APT29, Lazarus, Cozy Bear...'
    });

    if (!actorName) {
        return;
    }

    try {
        const actor = await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: `Searching for threat actor: ${actorName}...`,
                cancellable: false
            },
            async () => {
                return await client.searchThreatActor(actorName);
            }
        );

        if (actor) {
            // Display actor information in a new document
            const doc = await vscode.workspace.openTextDocument({
                content: formatThreatActorInfo(actor),
                language: 'markdown'
            });
            await vscode.window.showTextDocument(doc);
        } else {
            vscode.window.showInformationMessage(`No threat actor found matching "${actorName}"`);
        }
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to search threat actor: ${error}`);
    }
}

function formatThreatActorInfo(actor: any): string {
    return `# ${actor.name}

**Aliases:** ${actor.aliases?.join(', ') || 'N/A'}

**Country:** ${actor.country || 'Unknown'}

**Motivation:** ${actor.motivation || 'Unknown'}

**MITRE ATT&CK ID:** ${actor.mitre_id || 'N/A'}

## Description
${actor.description || 'No description available.'}

## Associated Campaigns
${actor.campaigns?.map((c: string) => `- ${c}`).join('\n') || 'No known campaigns'}

## Associated Malware
${actor.malware?.map((m: string) => `- ${m}`).join('\n') || 'No known malware'}

## Techniques (MITRE ATT&CK)
${actor.techniques?.map((t: string) => `- ${t}`).join('\n') || 'No known techniques'}
`;
}

async function getMITRETechniques(client: VSentinelClient): Promise<void> {
    const techniqueId = await vscode.window.showInputBox({
        prompt: 'Enter MITRE ATT&CK technique ID (optional)',
        placeHolder: 'e.g., T1566, T1059 - leave empty for all techniques'
    });

    try {
        const techniques = await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'Fetching MITRE ATT&CK techniques...',
                cancellable: false
            },
            async () => {
                return await client.getMITRETechniques(techniqueId);
            }
        );

        // Display techniques in a new document
        const doc = await vscode.workspace.openTextDocument({
            content: formatMITRETechniques(techniques),
            language: 'markdown'
        });
        await vscode.window.showTextDocument(doc);
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to fetch MITRE techniques: ${error}`);
    }
}

function formatMITRETechniques(techniques: any[]): string {
    const header = `# MITRE ATT&CK Techniques

This document contains MITRE ATT&CK technique information retrieved from V-Sentinel.

---

`;

    const content = techniques.map(t => `## ${t.id}: ${t.name}

**Tactic(s):** ${t.tactics?.join(', ') || 'N/A'}

**Platforms:** ${t.platforms?.join(', ') || 'N/A'}

**Description:** ${t.description || 'No description available.'}

**Detection:** ${t.detection || 'No detection information available.'}

---
`).join('\n');

    return header + content;
}