import chalk from 'chalk';
import ora from 'ora';
import YAML from 'js-yaml';
import Table from 'table';
import { VSentinelClient } from '../client';

export async function lookupCommand(value: string, options: {
    json?: boolean;
    yaml?: boolean;
    raw?: boolean;
    related?: boolean;
}): Promise<void> {
    const client = new VSentinelClient();
    const spinner = ora(`Looking up: ${value}...`).start();

    try {
        const result = await client.lookupIOC(value);
        spinner.stop();

        if (options.json) {
            console.log(JSON.stringify(result, null, 2));
            return;
        }

        if (options.yaml) {
            console.log(YAML.dump(result, { indent: 2 }));
            return;
        }

        // Pretty output
        displayResult(result, options.related);
    } catch (error) {
        spinner.fail(chalk.red('Lookup failed'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

function displayResult(result: any, showRelated?: boolean): void {
    const status = result.is_malicious 
        ? chalk.red.bold('🚨 MALICIOUS')
        : chalk.green.bold('✅ CLEAN');

    console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
    console.log(chalk.blue.bold('║         V-SENTINEL IOC LOOKUP            ║'));
    console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

    console.log(`Status: ${status}\n`);

    // IOC Details
    console.log(chalk.cyan('┌─ IOC Details ────────────────────────────'));
    console.log(chalk.cyan('│'), `Type:       ${chalk.white(result.ioc.type)}`);
    console.log(chalk.cyan('│'), `Value:      ${chalk.white(result.ioc.value)}`);
    console.log(chalk.cyan('│'), `Confidence: ${getConfidenceDisplay(result.ioc.confidence)}`);
    console.log(chalk.cyan('└───────────────────────────────────────────\n'));

    // Threat Actor
    if (result.threat_actor) {
        console.log(chalk.magenta('┌─ Threat Actor ────────────────────────────'));
        console.log(chalk.magenta('│'), `Name:       ${chalk.white(result.threat_actor.name)}`);
        if (result.threat_actor.aliases?.length) {
            console.log(chalk.magenta('│'), `Aliases:    ${chalk.white(result.threat_actor.aliases.join(', '))}`);
        }
        if (result.threat_actor.country) {
            console.log(chalk.magenta('│'), `Country:    ${chalk.white(result.threat_actor.country)}`);
        }
        if (result.threat_actor.motivation) {
            console.log(chalk.magenta('│'), `Motivation: ${chalk.white(result.threat_actor.motivation)}`);
        }
        console.log(chalk.magenta('└───────────────────────────────────────────\n'));
    }

    // Additional IOC info
    if (result.ioc.campaign) {
        console.log(chalk.yellow(`Campaign: ${result.ioc.campaign}`));
    }
    if (result.ioc.malware_family) {
        console.log(chalk.yellow(`Malware Family: ${result.ioc.malware_family}`));
    }
    if (result.ioc.tags?.length) {
        console.log(chalk.gray(`Tags: ${result.ioc.tags.map((t: string) => chalk.blue(`#${t}`)).join(' ')}`));
    }

    // Related IOCs
    if (showRelated && result.related_iocs?.length) {
        console.log(chalk.cyan('\n┌─ Related IOCs ────────────────────────────'));
        const tableData = [
            [chalk.bold('Type'), chalk.bold('Value'), chalk.bold('Confidence')],
            ...result.related_iocs.slice(0, 10).map((ioc: any) => [
                ioc.type,
                ioc.value.length > 40 ? ioc.value.substring(0, 40) + '...' : ioc.value,
                ioc.confidence
            ])
        ];
        console.log(Table.table(tableData, {
            border: Table.getBorderCharacters('void'),
            columnDefault: { paddingLeft: 1, paddingRight: 1 },
            drawHorizontalLine: () => false
        }));
        console.log(chalk.cyan('└───────────────────────────────────────────'));
    }

    console.log();
}

function getConfidenceDisplay(confidence: string): string {
    switch (confidence) {
        case 'high':
            return chalk.red.bold('HIGH');
        case 'medium':
            return chalk.yellow.bold('MEDIUM');
        case 'low':
            return chalk.gray.bold('LOW');
        default:
            return chalk.gray('UNKNOWN');
    }
}