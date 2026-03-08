import chalk from 'chalk';
import ora from 'ora';
import Table from 'table';
import { VSentinelClient } from '../client';

export async function threatIntelCommand(
    action: string,
    options: {
        name?: string;
        id?: string;
        limit?: string;
        json?: boolean;
    }
): Promise<void> {
    const client = new VSentinelClient();

    switch (action) {
        case 'actors':
            if (options.name) {
                await searchThreatActor(client, options.name, options.json);
            } else {
                await listThreatActors(client, options);
            }
            break;
        case 'techniques':
            await getMITRETechniques(client, options.id, options.json);
            break;
        case 'iocs':
            await listIOCs(client, options);
            break;
        default:
            console.error(chalk.red(`Unknown action: ${action}`));
            console.log(chalk.yellow('Available actions: actors, techniques, iocs'));
    }
}

async function listThreatActors(client: VSentinelClient, options: any): Promise<void> {
    const spinner = ora('Fetching threat actors...').start();

    try {
        const result = await client.getThreatActors(parseInt(options.limit || '50'));
        spinner.stop();

        if (options.json) {
            console.log(JSON.stringify(result, null, 2));
            return;
        }

        if (result.actors.length === 0) {
            console.log(chalk.yellow('No threat actors found.'));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║           THREAT ACTORS                  ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.gray(`Total: ${result.total}`));

        const tableData = [
            [chalk.bold('Name'), chalk.bold('Country'), chalk.bold('Motivation'), chalk.bold('MITRE ID')],
        ];

        result.actors.forEach((actor) => {
            tableData.push([
                actor.name,
                actor.country || 'Unknown',
                actor.motivation || 'Unknown',
                actor.mitre_id || '-'
            ]);
        });

        console.log(Table.table(tableData, {
            border: Table.getBorderCharacters('void'),
            columnDefault: { paddingLeft: 1, paddingRight: 1 },
            drawHorizontalLine: () => false
        }));

        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch threat actors'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function searchThreatActor(client: VSentinelClient, name: string, json?: boolean): Promise<void> {
    const spinner = ora(`Searching for threat actor: ${name}...`).start();

    try {
        const actor = await client.searchThreatActor(name);
        spinner.stop();

        if (!actor) {
            console.log(chalk.yellow(`No threat actor found matching "${name}"`));
            return;
        }

        if (json) {
            console.log(JSON.stringify(actor, null, 2));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║           THREAT ACTOR PROFILE           ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.cyan('Name:'), chalk.white.bold(actor.name));

        if (actor.aliases?.length) {
            console.log(chalk.cyan('Aliases:'), actor.aliases.join(', '));
        }

        if (actor.country) {
            console.log(chalk.cyan('Country:'), actor.country);
        }

        if (actor.motivation) {
            console.log(chalk.cyan('Motivation:'), actor.motivation);
        }

        if (actor.mitre_id) {
            console.log(chalk.cyan('MITRE ID:'), actor.mitre_id);
        }

        if (actor.description) {
            console.log(chalk.cyan('\nDescription:'));
            console.log(chalk.gray(actor.description));
        }

        if (actor.campaigns?.length) {
            console.log(chalk.magenta('\n┌─ Campaigns ─────────────────────────────'));
            actor.campaigns.forEach(c => console.log(chalk.magenta('│'), `• ${c}`));
            console.log(chalk.magenta('└──────────────────────────────────────────'));
        }

        if (actor.malware?.length) {
            console.log(chalk.red('\n┌─ Associated Malware ────────────────────'));
            actor.malware.forEach(m => console.log(chalk.red('│'), `• ${m}`));
            console.log(chalk.red('└──────────────────────────────────────────'));
        }

        if (actor.techniques?.length) {
            console.log(chalk.yellow('\n┌─ MITRE ATT&CK Techniques ───────────────'));
            const techniques = actor.techniques.slice(0, 10);
            techniques.forEach(t => console.log(chalk.yellow('│'), `• ${t}`));
            if (actor.techniques.length > 10) {
                console.log(chalk.yellow('│'), `  ... and ${actor.techniques.length - 10} more`);
            }
            console.log(chalk.yellow('└──────────────────────────────────────────'));
        }

        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to search threat actor'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function getMITRETechniques(client: VSentinelClient, techniqueId?: string, json?: boolean): Promise<void> {
    const spinner = ora('Fetching MITRE ATT&CK techniques...').start();

    try {
        const result = await client.getMITRETechniques(techniqueId);
        spinner.stop();

        if (json) {
            console.log(JSON.stringify(result, null, 2));
            return;
        }

        if (result.techniques.length === 0) {
            console.log(chalk.yellow('No techniques found.'));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║         MITRE ATT&CK TECHNIQUES          ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        const tableData = [
            [chalk.bold('ID'), chalk.bold('Name'), chalk.bold('Tactics')],
        ];

        result.techniques.forEach((technique) => {
            tableData.push([
                technique.id,
                technique.name.length > 30 ? technique.name.substring(0, 30) + '...' : technique.name,
                technique.tactics?.join(', ').substring(0, 20) || '-'
            ]);
        });

        console.log(Table.table(tableData, {
            border: Table.getBorderCharacters('void'),
            columnDefault: { paddingLeft: 1, paddingRight: 1 },
            drawHorizontalLine: () => false
        }));

        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch MITRE techniques'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function listIOCs(client: VSentinelClient, options: any): Promise<void> {
    const spinner = ora('Fetching IOCs...').start();

    try {
        const result = await client.getIOCs(parseInt(options.limit || '100'));
        spinner.stop();

        if (options.json) {
            console.log(JSON.stringify(result, null, 2));
            return;
        }

        if (result.iocs.length === 0) {
            console.log(chalk.yellow('No IOCs found.'));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║        INDICATORS OF COMPROMISE          ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.gray(`Total: ${result.total}`));

        const tableData = [
            [chalk.bold('Type'), chalk.bold('Value'), chalk.bold('Confidence'), chalk.bold('Threat Actor')],
        ];

        result.iocs.forEach((ioc) => {
            tableData.push([
                ioc.type.toUpperCase(),
                ioc.value.length > 30 ? ioc.value.substring(0, 30) + '...' : ioc.value,
                ioc.confidence || '-',
                ioc.threat_actor || '-'
            ]);
        });

        console.log(Table.table(tableData, {
            border: Table.getBorderCharacters('void'),
            columnDefault: { paddingLeft: 1, paddingRight: 1 },
            drawHorizontalLine: () => false
        }));

        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch IOCs'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}