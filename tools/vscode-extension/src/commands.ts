import * as vscode from 'vscode';
import * as fs from 'fs';
import * as crypto from 'crypto';
import { VSentinelClient } from './client';

export async function lookupIOC(client: VSentinelClient): Promise<void> {
    const editor = vscode.window.activeTextEditor;
    let selectedText = '';

    // Get selected text or prompt for input
    if (editor && editor.selection && !editor.selection.isEmpty) {
        selectedText = editor.document.getText(editor.selection);
    }

    const iocValue = await vscode.window.showInputBox({
        prompt: 'Enter IOC value to lookup',
        value: selectedText,
        placeHolder: 'IP, domain, URL, hash, or email...'
    });

    if (!iocValue) {
        return;
    }

    try {
        const result = await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: `Looking up IOC: ${iocValue}...`,
                cancellable: false
            },
            async () => {
                return await client.lookupIOC(iocValue);
            }
        );

        // Display results
        await displayIOCResult(result);
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to lookup IOC: ${error}`);
    }
}

export async function scanFileForIOCs(client: VSentinelClient, uri?: vscode.Uri): Promise<void> {
    let fileUri: vscode.Uri;

    if (uri) {
        fileUri = uri;
    } else {
        const activeEditor = vscode.window.activeTextEditor;
        if (!activeEditor) {
            vscode.window.showErrorMessage('No file selected to scan');
            return;
        }
        fileUri = activeEditor.document.uri;
    }

    try {
        // Read file content
        const content = await vscode.workspace.fs.readFile(fileUri);
        const text = Buffer.from(content).toString('utf-8');

        const result = await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: `Scanning ${fileUri.path.split('/').pop()} for IOCs...`,
                cancellable: false
            },
            async () => {
                return await client.scanContent(text);
            }
        );

        if (result.iocs_found === 0) {
            vscode.window.showInformationMessage('No IOCs found in the file.');
            return;
        }

        // Show summary
        const summary = Object.entries(result.summary)
            .map(([type, count]) => `${type}: ${count}`)
            .join(', ');

        const action = await vscode.window.showInformationMessage(
            `Found ${result.iocs_found} IOC(s) - ${summary}`,
            'View Details',
            'Open Report'
        );

        if (action === 'View Details') {
            await displayScanResult(result);
        } else if (action === 'Open Report') {
            await openScanReport(fileUri, result);
        }
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to scan file: ${error}`);
    }
}

export async function checkFileHash(client: VSentinelClient): Promise<void> {
    const options = await vscode.window.showQuickPick(
        [
            { label: 'Enter hash value', description: 'Manually enter a hash to check' },
            { label: 'Check current file', description: 'Calculate and check hash of the current file' }
        ],
        { placeHolder: 'Select hash check method' }
    );

    if (!options) {
        return;
    }

    let hashValue: string | undefined;

    if (options.label === 'Enter hash value') {
        hashValue = await vscode.window.showInputBox({
            prompt: 'Enter hash value (MD5, SHA1, or SHA256)',
            placeHolder: 'e.g., 5d41402abc4b2a76b9719d911017c592'
        });
    } else {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            vscode.window.showErrorMessage('No file open to calculate hash');
            return;
        }

        try {
            const content = await vscode.workspace.fs.readFile(editor.document.uri);
            const fileBuffer = Buffer.from(content);

            // Calculate all hashes
            const md5 = crypto.createHash('md5').update(fileBuffer).digest('hex');
            const sha1 = crypto.createHash('sha1').update(fileBuffer).digest('hex');
            const sha256 = crypto.createHash('sha256').update(fileBuffer).digest('hex');

            const selectedHash = await vscode.window.showQuickPick(
                [
                    { label: `MD5: ${md5}`, hash: md5 },
                    { label: `SHA1: ${sha1}`, hash: sha1 },
                    { label: `SHA256: ${sha256}`, hash: sha256 }
                ],
                { placeHolder: 'Select hash to check' }
            );

            hashValue = selectedHash?.hash;
        } catch (error) {
            vscode.window.showErrorMessage(`Failed to calculate file hash: ${error}`);
            return;
        }
    }

    if (!hashValue) {
        return;
    }

    try {
        const result = await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: `Checking hash: ${hashValue.substring(0, 16)}...`,
                cancellable: false
            },
            async () => {
                return await client.checkHash(hashValue);
            }
        );

        await displayIOCResult(result);
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to check hash: ${error}`);
    }
}

async function displayIOCResult(result: any): Promise<void> {
    const doc = await vscode.workspace.openTextDocument({
        content: formatIOCResult(result),
        language: 'markdown'
    });
    await vscode.window.showTextDocument(doc);
}

function formatIOCResult(result: any): string {
    const status = result.is_malicious ? '🚨 **MALICIOUS**' : '✅ **CLEAN**';

    return `# IOC Lookup Result

## Status: ${status}

## IOC Details
| Property | Value |
|----------|-------|
| **Type** | ${result.ioc.type} |
| **Value** | \`${result.ioc.value}\` |
| **Confidence** | ${result.ioc.confidence || 'N/A'} |

${result.threat_actor ? `## Threat Actor
| Property | Value |
|----------|-------|
| **Name** | ${result.threat_actor.name} |
| **Aliases** | ${result.threat_actor.aliases?.join(', ') || 'N/A'} |
| **Country** | ${result.threat_actor.country || 'Unknown'} |
| **Motivation** | ${result.threat_actor.motivation || 'Unknown'} |
` : ''}

${result.related_iocs && result.related_iocs.length > 0 ? `## Related IOCs
${result.related_iocs.map((ioc: any) => `- \`${ioc.value}\` (${ioc.type})`).join('\n')}
` : ''}

${result.ioc.tags && result.ioc.tags.length > 0 ? `## Tags
${result.ioc.tags.map((t: string) => `- ${t}`).join('\n')}
` : ''}

${result.ioc.references && result.ioc.references.length > 0 ? `## References
${result.ioc.references.map((r: string) => `- ${r}`).join('\n')}
` : ''}
`;
}

async function displayScanResult(result: any): Promise<void> {
    const doc = await vscode.workspace.openTextDocument({
        content: formatScanResult(result),
        language: 'markdown'
    });
    await vscode.window.showTextDocument(doc);
}

function formatScanResult(result: any): string {
    const header = `# IOC Scan Results

**Total IOCs Found:** ${result.iocs_found}

## Summary by Type
| Type | Count |
|------|-------|
${Object.entries(result.summary).map(([type, count]) => `| ${type} | ${count} |`).join('\n')}

---

`;

    const iocs = result.iocs.map((item: any, index: number) => {
        const status = item.is_malicious ? '🚨 MALICIOUS' : '⚠️ Unknown';
        return `## ${index + 1}. ${item.ioc.type.toUpperCase()}: \`${item.ioc.value}\`

**Status:** ${status}

${item.threat_actor ? `- **Threat Actor:** ${item.threat_actor.name}` : ''}
${item.ioc.confidence ? `- **Confidence:** ${item.ioc.confidence}` : ''}

---
`;
    }).join('\n');

    return header + iocs;
}

async function openScanReport(uri: vscode.Uri, result: any): Promise<void> {
    const reportContent = formatScanResult(result);
    const reportUri = vscode.Uri.parse(`untitled:${uri.path}.ioc-report.md`);

    const doc = await vscode.workspace.openTextDocument({
        content: reportContent,
        language: 'markdown'
    });
    await vscode.window.showTextDocument(doc);
}