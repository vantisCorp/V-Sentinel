import * as fs from 'fs';
import * as path from 'path';
import chalk from 'chalk';
import ora from 'ora';
import YAML from 'js-yaml';
import Table from 'table';
import { VSentinelClient } from '../client';

export async function scanCommand(file: string, options: {
    format?: string;
    output?: string;
    minConfidence?: string;
    types?: string;
}): Promise<void> {
    const filePath = path.resolve(file);
    
    if (!fs.existsSync(filePath)) {
        console.error(chalk.red(`File not found: ${filePath}`));
        process.exit(1);
    }

    const client = new VSentinelClient();
    const spinner = ora(`Scanning: ${path.basename(filePath)}...`).start();

    try {
        const content = fs.readFileSync(filePath, 'utf-8');
        const types = options.types?.split(',').map(t => t.trim());
        
        const result = await client.scanContent(content, types, options.minConfidence);
        spinner.stop();

        if (options.format === 'json') {
            outputJson(result, options.output);
            return;
        }

        if (options.format === 'yaml') {
            outputYaml(result, options.output);
            return;
        }

        displayResults(result, filePath, options.output);
    } catch (error) {
        spinner.fail(chalk.red('Scan failed'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

function outputJson(result: any, outputFile?: string): void {
    const json = JSON.stringify(result, null, 2);
    if (outputFile) {
        fs.writeFileSync(outputFile, json);
        console.log(chalk.green(`Results saved to: ${outputFile}`));
    } else {
        console.log(json);
    }
}

function outputYaml(result: any, outputFile?: string): void {
    const yaml = YAML.dump(result, { indent: 2 });
    if (outputFile) {
        fs.writeFileSync(outputFile, yaml);
        console.log(chalk.green(`Results saved to: ${outputFile}`));
    } else {
        console.log(yaml);
    }
}

function displayResults(result: any, filePath: string, outputFile?: string): void {
    const fileName = path.basename(filePath);
    
    console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
    console.log(chalk.blue.bold('║         V-SENTINEL FILE SCAN             ║'));
    console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

    console.log(chalk.gray(`File: ${fileName}`));
    console.log(chalk.gray(`IOCs Found: ${result.iocs_found}`));

    if (result.iocs_found === 0) {
        console.log(chalk.green('\n✅ No indicators of compromise detected.\n'));
        return;
    }

    // Summary table
    console.log(chalk.cyan('\n┌─ Summary by Type ─────────────────────────'));
    const summaryData = Object.entries(result.summary).map(([type, count]) => [
        type.toUpperCase(),
        String(count)
    ]);
    
    console.log(Table.table([
        [chalk.bold('Type'), chalk.bold('Count')],
        ...summaryData
    ], {
        border: Table.getBorderCharacters('void'),
        columnDefault: { paddingLeft: 1, paddingRight: 1 },
        drawHorizontalLine: () => false
    }));
    console.log(chalk.cyan('└───────────────────────────────────────────'));

    // Detailed IOCs
    console.log(chalk.cyan('\n┌─ Detected IOCs ───────────────────────────'));
    
    const iocTable = [
        [chalk.bold('#'), chalk.bold('Status'), chalk.bold('Type'), chalk.bold('Value'), chalk.bold('Confidence')],
    ];

    result.iocs.forEach((item: any, index: number) => {
        const status = item.is_malicious ? chalk.red('🚨') : chalk.gray('⚠️');
        const value = item.ioc.value.length > 35 
            ? item.ioc.value.substring(0, 35) + '...'
            : item.ioc.value;
        
        iocTable.push([
            String(index + 1),
            status,
            item.ioc.type,
            value,
            item.ioc.confidence || '-'
        ]);
    });

    console.log(Table.table(iocTable, {
        border: Table.getBorderCharacters('void'),
        columnDefault: { paddingLeft: 1, paddingRight: 1 },
        drawHorizontalLine: () => false
    }));
    console.log(chalk.cyan('└───────────────────────────────────────────'));

    // Threat actors found
    const actors = result.iocs
        .filter((i: any) => i.threat_actor)
        .map((i: any) => i.threat_actor.name);
    
    if (actors.length > 0) {
        console.log(chalk.magenta('\n┌─ Associated Threat Actors ────────────────'));
        [...new Set(actors)].forEach(actor => {
            console.log(chalk.magenta('│'), `• ${actor}`);
        });
        console.log(chalk.magenta('└───────────────────────────────────────────'));
    }

    // Save to file
    if (outputFile) {
        const report = generateReport(result, fileName);
        fs.writeFileSync(outputFile, report);
        console.log(chalk.green(`\n📄 Report saved to: ${outputFile}\n`));
    }

    console.log();
}

function generateReport(result: any, fileName: string): string {
    const lines = [
        `# V-Sentinel IOC Scan Report`,
        ``,
        `**File:** ${fileName}`,
        `**Scan Date:** ${new Date().toISOString()}`,
        `**Total IOCs Found:** ${result.iocs_found}`,
        ``,
        `## Summary`,
        ``,
        `| Type | Count |`,
        `|------|-------|`,
    ];

    for (const [type, count] of Object.entries(result.summary)) {
        lines.push(`| ${type} | ${count} |`);
    }

    lines.push(``, `## Detected IOCs`, ``);

    result.iocs.forEach((item: any, index: number) => {
        lines.push(`### ${index + 1}. ${item.ioc.type.toUpperCase()}`);
        lines.push(`- **Value:** \`${item.ioc.value}\``);
        lines.push(`- **Malicious:** ${item.is_malicious ? 'Yes' : 'Unknown'}`);
        if (item.ioc.confidence) {
            lines.push(`- **Confidence:** ${item.ioc.confidence}`);
        }
        if (item.threat_actor) {
            lines.push(`- **Threat Actor:** ${item.threat_actor.name}`);
        }
        lines.push(``);
    });

    return lines.join('\n');
}