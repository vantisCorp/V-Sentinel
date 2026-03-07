import chalk from 'chalk';
import ora from 'ora';
import Table from 'table';
import { VSentinelClient } from '../client';

export async function detectionsCommand(
    action: string,
    options: {
        id?: string;
        status?: string;
        severity?: string;
        limit?: string;
        json?: boolean;
    }
): Promise<void> {
    const client = new VSentinelClient();

    switch (action) {
        case 'list':
            await listDetections(client, options);
            break;
        case 'get':
            if (!options.id) {
                console.error(chalk.red('Detection ID is required for get action'));
                process.exit(1);
            }
            await getDetection(client, options.id, options.json);
            break;
        case 'update':
            if (!options.id) {
                console.error(chalk.red('Detection ID is required for update action'));
                process.exit(1);
            }
            if (!options.status) {
                console.error(chalk.red('Status is required for update action'));
                process.exit(1);
            }
            await updateDetection(client, options.id, options.status);
            break;
        default:
            console.error(chalk.red(`Unknown action: ${action}`));
            console.log(chalk.yellow('Available actions: list, get, update'));
    }
}

async function listDetections(client: VSentinelClient, options: any): Promise<void> {
    const spinner = ora('Fetching detections...').start();

    try {
        const result = await client.getDetections({
            limit: parseInt(options.limit || '50'),
            status: options.status,
            severity: options.severity
        });

        spinner.stop();

        if (options.json) {
            console.log(JSON.stringify(result, null, 2));
            return;
        }

        if (result.detections.length === 0) {
            console.log(chalk.yellow('No detections found.'));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║           V-SENTINEL DETECTIONS          ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.gray(`Total: ${result.total}`));

        const tableData = [
            [chalk.bold('ID'), chalk.bold('Title'), chalk.bold('Severity'), chalk.bold('Status'), chalk.bold('Created')],
        ];

        result.detections.forEach((detection) => {
            tableData.push([
                detection.id.substring(0, 8),
                detection.title.length > 30 ? detection.title.substring(0, 30) + '...' : detection.title,
                getSeverityBadge(detection.severity),
                getStatusBadge(detection.status),
                formatDate(detection.created_at)
            ]);
        });

        console.log(Table.table(tableData, {
            border: Table.getBorderCharacters('void'),
            columnDefault: { paddingLeft: 1, paddingRight: 1 },
            drawHorizontalLine: () => false
        }));

        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch detections'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function getDetection(client: VSentinelClient, id: string, json?: boolean): Promise<void> {
    const spinner = ora(`Fetching detection ${id}...`).start();

    try {
        const detection = await client.getDetection(id);
        spinner.stop();

        if (json) {
            console.log(JSON.stringify(detection, null, 2));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║           DETECTION DETAILS              ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.cyan('ID:'), detection.id);
        console.log(chalk.cyan('Title:'), detection.title);
        console.log(chalk.cyan('Description:'), detection.description);
        console.log(chalk.cyan('Severity:'), getSeverityBadge(detection.severity));
        console.log(chalk.cyan('Status:'), getStatusBadge(detection.status));
        console.log(chalk.cyan('Created:'), detection.created_at);
        console.log(chalk.cyan('Updated:'), detection.updated_at);

        if (detection.techniques?.length) {
            console.log(chalk.cyan('MITRE ATT&CK:'), detection.techniques.join(', '));
        }

        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch detection'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function updateDetection(client: VSentinelClient, id: string, status: string): Promise<void> {
    const spinner = ora(`Updating detection ${id}...`).start();

    try {
        const detection = await client.updateDetectionStatus(id, status);
        spinner.succeed(chalk.green('Detection updated successfully'));

        console.log(chalk.gray(`ID: ${detection.id}`));
        console.log(chalk.gray(`New Status: ${detection.status}`));
    } catch (error) {
        spinner.fail(chalk.red('Failed to update detection'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

function getSeverityBadge(severity: string): string {
    switch (severity) {
        case 'critical':
            return chalk.red.bold('CRITICAL');
        case 'high':
            return chalk.yellow.bold('HIGH');
        case 'medium':
            return chalk.blue.bold('MEDIUM');
        case 'low':
            return chalk.gray.bold('LOW');
        default:
            return chalk.gray(severity.toUpperCase());
    }
}

function getStatusBadge(status: string): string {
    switch (status) {
        case 'new':
            return chalk.red.bold('NEW');
        case 'in_progress':
            return chalk.yellow.bold('IN PROGRESS');
        case 'resolved':
            return chalk.green.bold('RESOLVED');
        case 'false_positive':
            return chalk.gray.bold('FALSE POSITIVE');
        default:
            return chalk.gray(status.toUpperCase());
    }
}

function formatDate(dateStr: string): string {
    const date = new Date(dateStr);
    return date.toLocaleDateString();
}