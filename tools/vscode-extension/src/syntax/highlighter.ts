import * as vscode from 'vscode';

export function registerSyntaxHighlighting(context: vscode.ExtensionContext): void {
    // Register IOC JSON syntax
    const iocJsonProvider = vscode.languages.registerDocumentSemanticTokensProvider(
        { language: 'ioc-json' },
        new IOCSemanticTokensProvider(),
        iocLegend
    );

    // Register YARA syntax
    const yaraProvider = vscode.languages.registerDocumentSemanticTokensProvider(
        { language: 'yara' },
        new YARASemanticTokensProvider(),
        yaraLegend
    );

    context.subscriptions.push(iocJsonProvider, yaraProvider);
}

// IOC JSON Semantic Tokens
const iocTokenTypes = ['ioc', 'threatActor', 'malware', 'confidence', 'tag', 'reference'];
const iocTokenModifiers = ['malicious', 'suspicious', 'benign'];
const iocLegend = new vscode.SemanticTokensLegend(iocTokenTypes, iocTokenModifiers);

class IOCSemanticTokensProvider implements vscode.DocumentSemanticTokensProvider {
    async provideDocumentSemanticTokens(
        document: vscode.TextDocument,
        token: vscode.CancellationToken
    ): Promise<vscode.SemanticTokens> {
        const tokensBuilder = new vscode.SemanticTokensBuilder(iocLegend);

        const text = document.getText();
        try {
            const content = JSON.parse(text);

            // Highlight IOC values
            if (content.value) {
                const regex = new RegExp(`"${escapeRegex(content.value)}"`, 'g');
                let match;
                while ((match = regex.exec(text)) !== null) {
                    const startPos = document.positionAt(match.index + 1);
                    const endPos = document.positionAt(match.index + match[0].length - 1);
                    const range = new vscode.Range(startPos, endPos);
                    tokensBuilder.push(range, 'ioc', ['malicious']);
                }
            }

            // Highlight threat actor
            if (content.threat_actor) {
                const regex = new RegExp(`"${escapeRegex(content.threat_actor)}"`, 'g');
                let match;
                while ((match = regex.exec(text)) !== null) {
                    const startPos = document.positionAt(match.index + 1);
                    const endPos = document.positionAt(match.index + match[0].length - 1);
                    const range = new vscode.Range(startPos, endPos);
                    tokensBuilder.push(range, 'threatActor');
                }
            }

            // Highlight malware families
            if (content.malware_family) {
                const regex = new RegExp(`"${escapeRegex(content.malware_family)}"`, 'g');
                let match;
                while ((match = regex.exec(text)) !== null) {
                    const startPos = document.positionAt(match.index + 1);
                    const endPos = document.positionAt(match.index + match[0].length - 1);
                    const range = new vscode.Range(startPos, endPos);
                    tokensBuilder.push(range, 'malware');
                }
            }

            // Highlight confidence level
            const confidenceMatch = text.match(/"confidence"\s*:\s*"(low|medium|high)"/);
            if (confidenceMatch) {
                const valueStart = text.indexOf(confidenceMatch[1], confidenceMatch.index);
                const startPos = document.positionAt(valueStart);
                const endPos = document.positionAt(valueStart + confidenceMatch[1].length);
                tokensBuilder.push(new vscode.Range(startPos, endPos), 'confidence');
            }

            // Highlight tags
            if (content.tags && Array.isArray(content.tags)) {
                for (const tag of content.tags) {
                    const regex = new RegExp(`"${escapeRegex(tag)}"`, 'g');
                    let match;
                    while ((match = regex.exec(text)) !== null) {
                        const startPos = document.positionAt(match.index + 1);
                        const endPos = document.positionAt(match.index + match[0].length - 1);
                        const range = new vscode.Range(startPos, endPos);
                        tokensBuilder.push(range, 'tag');
                    }
                }
            }

        } catch (e) {
            // Invalid JSON, skip semantic highlighting
        }

        return tokensBuilder.build();
    }
}

// YARA Semantic Tokens
const yaraTokenTypes = [
    'keyword', 'string', 'meta', 'condition', 'identifier',
    'hexString', 'operator', 'function'
];
const yaraTokenModifiers = [];
const yaraLegend = new vscode.SemanticTokensLegend(yaraTokenTypes, yaraTokenModifiers);

class YARASemanticTokensProvider implements vscode.DocumentSemanticTokensProvider {
    async provideDocumentSemanticTokens(
        document: vscode.TextDocument,
        token: vscode.CancellationToken
    ): Promise<vscode.SemanticTokens> {
        const tokensBuilder = new vscode.SemanticTokensBuilder(yaraLegend);
        const text = document.getText();

        // YARA keywords
        const keywords = [
            'rule', 'global', 'private', 'meta', 'strings', 'condition',
            'true', 'false', 'and', 'or', 'not', 'any', 'all', 'them',
            'entrypoint', 'filesize', 'int8', 'int16', 'int32', 'uint8',
            'uint16', 'uint32', 'int8be', 'int16be', 'int32be', 'uint8be',
            'uint16be', 'uint32be', 'for', 'of', 'import'
        ];

        for (const keyword of keywords) {
            const regex = new RegExp(`\\b${keyword}\\b`, 'g');
            let match;
            while ((match = regex.exec(text)) !== null) {
                const startPos = document.positionAt(match.index);
                const endPos = document.positionAt(match.index + match[0].length);
                tokensBuilder.push(new vscode.Range(startPos, endPos), 'keyword');
            }
        }

        // Hex strings { ... }
        const hexStringRegex = /\{[0-9a-fA-F\s\?\[\]\(\)\|]+\}/g;
        let match;
        while ((match = hexStringRegex.exec(text)) !== null) {
            const startPos = document.positionAt(match.index);
            const endPos = document.positionAt(match.index + match[0].length);
            tokensBuilder.push(new vscode.Range(startPos, endPos), 'hexString');
        }

        // String variables $name
        const stringVarRegex = /\$[a-zA-Z_][a-zA-Z0-9_]*/g;
        while ((match = stringVarRegex.exec(text)) !== null) {
            const startPos = document.positionAt(match.index);
            const endPos = document.positionAt(match.index + match[0].length);
            tokensBuilder.push(new vscode.Range(startPos, endPos), 'identifier');
        }

        // Operators
        const operators = ['==', '!=', '<', '>', '<=', '>=', 'contains', 'matches', 'icontains', 'startswith', 'endswith'];
        for (const op of operators) {
            const regex = new RegExp(escapeRegex(op), 'g');
            let opMatch;
            while ((opMatch = regex.exec(text)) !== null) {
                const startPos = document.positionAt(opMatch.index);
                const endPos = document.positionAt(opMatch.index + opMatch[0].length);
                tokensBuilder.push(new vscode.Range(startPos, endPos), 'operator');
            }
        }

        // YARA functions
        const functions = ['uint8', 'uint16', 'uint32', 'int8', 'int16', 'int32'];
        for (const func of functions) {
            const regex = new RegExp(`\\b${func}\\s*\\(`, 'g');
            let funcMatch;
            while ((funcMatch = regex.exec(text)) !== null) {
                const startPos = document.positionAt(funcMatch.index);
                const endPos = document.positionAt(funcMatch.index + func.length);
                tokensBuilder.push(new vscode.Range(startPos, endPos), 'function');
            }
        }

        return tokensBuilder.build();
    }
}

function escapeRegex(str: string): string {
    return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}