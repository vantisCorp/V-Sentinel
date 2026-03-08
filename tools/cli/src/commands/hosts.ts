import chalk from 'chalk';
import ora from 'ora';
import Table from 'table';
import inquirer from 'inquirer';
import { VSentinelClient } from '../client';

export async function hostsCommand(
    action: string,
    options: {
        id?: string;
        platform?: string;
        state?: string;
        limit?: string;
        json?: boolean;
    }
): Promise<void> {
    const client = new VSentinelClient();

    switch (action) {
        case 'list':
            await listHosts(client, options);
            break;
        case 'get':
            if (!options.id) {
                console.error(chalk.red('Host ID is required for get action'));
                process.exit(1);
            }
            await getHost(client, options.id, options.json);
            break;
        case 'isolate':
            if (!options.id) {
                console.error(chalk.red('Host ID is required for isolate action'));
                process.exit(1);
            }
            await isolateHost(client, options.id);
            break;
        case 'unisolate':
            if (!options.id) {
                console.error(chalk.red('Host ID is required for unisolate action'));
                process.exit(1);
            }
            await unisolateHost(client, options.id);
            break;
        default:
            console.error(chalk.red(`Unknown action: ${action}`));
            console.log(chalk.yellow('Available actions: list, get, isolate, unisolate'));
    }
}

async function listHosts(client: VSentinelClient, options: any): Promise<void> {
    const spinner = ora('Fetching hosts...').start();

    try {
        const result = await client.getHosts({
            limit: parseInt(options.limit || '50'),
            platform: options.platform,
            state: options.state
        });

        spinner.stop();

        if (options.json) {
            console.log(JSON.stringify(result, null, 2));
            return;
        }

        if (result.hosts.length === 0) {
            console.log(chalk.yellow('No hosts found.'));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║            V-SENTINEL HOSTS              ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.gray(`Total: ${result.total}`));

        const tableData = [
            [chalk.bold('ID'), chalk.bold('Hostname'), chalk.bold('Platform'), chalk.bold('State'), chalk.bold('Isolated')],
        ];

        result.hosts.forEach((host) => {
            tableData.push([
                host.id.substring(0, 8),
                host.hostname,
                getPlatformBadge(host.platform),
                getStateBadge(host.state),
                host.is_isolated ? chalk.red('Yes') : chalk.green('No')
            ]);
        });

        console.log(Table.table(tableData, {
            border: Table.getBorderCharacters('void'),
            columnDefault: { paddingLeft: 1, paddingRight: 1 },
            drawHorizontalLine: () => false
        }));

        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch hosts'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function getHost(client: VSentinelClient, id: string, json?: boolean): Promise<void> {
    const spinner = ora(`Fetching host ${id}...`).start();

    try {
        const host = await client.getHost(id);
        spinner.stop();

        if (json) {
            console.log(JSON.stringify(host, null, 2));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║              HOST DETAILS                ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.cyan('ID:'), host.id);
        console.log(chalk.cyan('Hostname:'), host.hostname);
        console.log(chalk.cyan('Platform:'), getPlatformBadge(host.platform));
        console.log(chalk.cyan('State:'), getStateBadge(host.state));
        console.log(chalk.cyan('Isolated:'), host.is_isolated ? chalk.red('Yes') : chalk.green('No'));

        if (host.ip_addresses?.length) {
            console.log(chalk.cyan('IP Addresses:'), host.ip_addresses.join(', '));
        }

        if (host.tags?.length) {
            console.log(chalk.cyan('Tags:'), host.tags.map(t => chalk.blue(`#${t}`)).join(' '));
        }

        console.log(chalk.cyan('Last Seen:'), host.last_seen);
        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch host'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function isolateHost(client: VSentinelClient, id: string): Promise<void> {
    console.log(chalk.yellow('\n⚠️  Network isolation will block all network traffic for this host.\n'));

    const answers = await inquirer.prompt([
        {
            type: 'input',
            name: 'reason',
            message: 'Reason for isolation:',
            validate: (input: string) => input.length > 0 || 'Reason is required'
        },
        {
            type: 'confirm',
            name: 'confirm',
            message: 'Are you sure you want to isolate this host?',
            default: false
        }
    ]);

    if (!answers.confirm) {
        console.log(chalk.gray('Operation cancelled.'));
        return;
    }

    const spinner = ora(`Isolating host ${id}...`).start();

    try {
        const host = await client.isolateHost(id, answers.reason);
        spinner.succeed(chalk.green('Host isolated successfully'));

        console.log(chalk.gray(`Host: ${host.hostname}`));
        console.log(chalk.gray(`State: ${host.state}`));
    } catch (error) {
        spinner.fail(chalk.red('Failed to isolate host'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function unisolateHost(client: VSentinelClient, id: string): Promise<void> {
    const answers = await inquirer.prompt([
        {
            type: 'confirm',
            name: 'confirm',
            message: 'Are you sure you want to remove network isolation?',
            default: false
        }
    ]);

    if (!answers.confirm) {
        console.log(chalk.gray('Operation cancelled.'));
        return;
    }

    const spinner = ora(`Removing isolation for host ${id}...`).start();

    try {
        const host = await client.unisolateHost(id);
        spinner.succeed(chalk.green('Host unisolated successfully'));

        console.log(chalk.gray(`Host: ${host.hostname}`));
        console.log(chalk.gray(`State: ${host.state}`));
    } catch (error) {
        spinner.fail(chalk.red('Failed to unisolate host'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

function getPlatformBadge(platform: string): string {
    switch (platform) {
        case 'windows':
            return chalk.blue.bold('Windows');
        case 'linux':
            return chalk.yellow.bold('Linux');
        case 'macos':
            return chalk.gray.bold('macOS');
        default:
            return chalk.gray(platform);
    }
}

function getStateBadge(state: string): string {
    switch (state) {
        case 'online':
            return chalk.green.bold('● ONLINE');
        case 'offline':
            return chalk.red.bold('○ OFFLINE');
        case 'isolated':
            return chalk.yellow.bold('⬤ ISOLATED');
        default:
            return chalk.gray(state.toUpperCase());
    }
}