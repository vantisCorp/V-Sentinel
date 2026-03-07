import chalk from 'chalk';
import ora from 'ora';
import Table from 'table';
import inquirer from 'inquirer';
import { VSentinelClient } from '../client';

export async function incidentsCommand(
    action: string,
    options: {
        id?: string;
        status?: string;
        severity?: string;
        title?: string;
        description?: string;
        limit?: string;
        json?: boolean;
    }
): Promise<void> {
    const client = new VSentinelClient();

    switch (action) {
        case 'list':
            await listIncidents(client, options);
            break;
        case 'get':
            if (!options.id) {
                console.error(chalk.red('Incident ID is required for get action'));
                process.exit(1);
            }
            await getIncident(client, options.id, options.json);
            break;
        case 'create':
            await createIncident(client, options);
            break;
        case 'update':
            if (!options.id) {
                console.error(chalk.red('Incident ID is required for update action'));
                process.exit(1);
            }
            await updateIncidentStatus(client, options.id, options.status);
            break;
        default:
            console.error(chalk.red(`Unknown action: ${action}`));
            console.log(chalk.yellow('Available actions: list, get, create, update'));
    }
}

async function listIncidents(client: VSentinelClient, options: any): Promise<void> {
    const spinner = ora('Fetching incidents...').start();

    try {
        const result = await client.getIncidents({
            limit: parseInt(options.limit || '50'),
            status: options.status,
            severity: options.severity
        });

        spinner.stop();

        if (options.json) {
            console.log(JSON.stringify(result, null, 2));
            return;
        }

        if (result.incidents.length === 0) {
            console.log(chalk.yellow('No incidents found.'));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║           V-SENTINEL INCIDENTS           ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.gray(`Total: ${result.total}`));

        const tableData = [
            [chalk.bold('ID'), chalk.bold('Title'), chalk.bold('Severity'), chalk.bold('Status'), chalk.bold('Created')],
        ];

        result.incidents.forEach((incident) => {
            tableData.push([
                incident.id.substring(0, 8),
                incident.title.length > 25 ? incident.title.substring(0, 25) + '...' : incident.title,
                getSeverityBadge(incident.severity),
                getStatusBadge(incident.status),
                formatDate(incident.created_at)
            ]);
        });

        console.log(Table.table(tableData, {
            border: Table.getBorderCharacters('void'),
            columnDefault: { paddingLeft: 1, paddingRight: 1 },
            drawHorizontalLine: () => false
        }));

        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch incidents'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function getIncident(client: VSentinelClient, id: string, json?: boolean): Promise<void> {
    const spinner = ora(`Fetching incident ${id}...`).start();

    try {
        const incident = await client.getIncident(id);
        spinner.stop();

        if (json) {
            console.log(JSON.stringify(incident, null, 2));
            return;
        }

        console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
        console.log(chalk.blue.bold('║           INCIDENT DETAILS               ║'));
        console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

        console.log(chalk.cyan('ID:'), incident.id);
        console.log(chalk.cyan('Title:'), incident.title);
        console.log(chalk.cyan('Severity:'), getSeverityBadge(incident.severity));
        console.log(chalk.cyan('Status:'), getStatusBadge(incident.status));

        if (incident.phase) {
            console.log(chalk.cyan('Phase:'), incident.phase);
        }

        if (incident.description) {
            console.log(chalk.cyan('Description:'));
            console.log(chalk.gray(incident.description));
        }

        if (incident.hosts?.length) {
            console.log(chalk.cyan('Affected Hosts:'), incident.hosts.join(', '));
        }

        if (incident.assignee) {
            console.log(chalk.cyan('Assignee:'), incident.assignee);
        }

        console.log(chalk.cyan('Created:'), incident.created_at);
        console.log(chalk.cyan('Updated:'), incident.updated_at);
        console.log();
    } catch (error) {
        spinner.fail(chalk.red('Failed to fetch incident'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function createIncident(client: VSentinelClient, options: any): Promise<void> {
    let { title, description, severity } = options;

    if (!title || !description || !severity) {
        const answers = await inquirer.prompt([
            {
                type: 'input',
                name: 'title',
                message: 'Incident title:',
                default: title,
                validate: (input: string) => input.length > 0 || 'Title is required'
            },
            {
                type: 'editor',
                name: 'description',
                message: 'Incident description:',
                default: description
            },
            {
                type: 'list',
                name: 'severity',
                message: 'Severity:',
                choices: ['critical', 'high', 'medium', 'low'],
                default: severity
            }
        ]);

        title = answers.title;
        description = answers.description;
        severity = answers.severity;
    }

    const spinner = ora('Creating incident...').start();

    try {
        const incident = await client.createIncident({
            title,
            description,
            severity
        });

        spinner.succeed(chalk.green('Incident created successfully'));

        console.log(chalk.gray(`ID: ${incident.id}`));
        console.log(chalk.gray(`Title: ${incident.title}`));
        console.log(chalk.gray(`Severity: ${incident.severity}`));
    } catch (error) {
        spinner.fail(chalk.red('Failed to create incident'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

async function updateIncidentStatus(client: VSentinelClient, id: string, status?: string): Promise<void> {
    if (!status) {
        const answer = await inquirer.prompt([
            {
                type: 'list',
                name: 'status',
                message: 'New status:',
                choices: ['new', 'in_progress', 'on_hold', 'resolved', 'closed', 'false_positive']
            }
        ]);
        status = answer.status;
    }

    const spinner = ora(`Updating incident ${id}...`).start();

    try {
        const incident = await client.updateIncidentStatus(id, status!);
        spinner.succeed(chalk.green('Incident updated successfully'));

        console.log(chalk.gray(`ID: ${incident.id}`));
        console.log(chalk.gray(`New Status: ${incident.status}`));
    } catch (error) {
        spinner.fail(chalk.red('Failed to update incident'));
        console.error(chalk.red(error instanceof Error ? error.message : String(error)));
        process.exit(1);
    }
}

function getSeverityBadge(severity: string): string {
    switch (severity) {
        case 'critical':
            return chalk.red.bold('🔴 CRITICAL');
        case 'high':
            return chalk.yellow.bold('🟠 HIGH');
        case 'medium':
            return chalk.blue.bold('🟡 MEDIUM');
        case 'low':
            return chalk.gray.bold('🟢 LOW');
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
        case 'on_hold':
            return chalk.gray.bold('ON HOLD');
        case 'resolved':
            return chalk.green.bold('RESOLVED');
        case 'closed':
            return chalk.blue.bold('CLOSED');
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