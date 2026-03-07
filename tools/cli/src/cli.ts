#!/usr/bin/env node

import { Command } from 'commander';
import chalk from 'chalk';
import updateNotifier from 'update-notifier';
import pkg from '../package.json';
import { initCommand } from './commands/init';
import { lookupCommand } from './commands/lookup';
import { scanCommand } from './commands/scan';
import { detectionsCommand } from './commands/detections';
import { hostsCommand } from './commands/hosts';
import { incidentsCommand } from './commands/incidents';
import { threatIntelCommand } from './commands/threat-intel';
import { configCommand } from './commands/config';
import { versionCommand } from './commands/version';

// Check for updates
const notifier = updateNotifier({ pkg });
notifier.notify();

const program = new Command();

program
    .name('vsentinel')
    .description('CLI for V-Sentinel threat intelligence and security operations')
    .version(pkg.version);

// Register commands
program
    .command('init')
    .description('Initialize V-Sentinel CLI configuration')
    .option('-f, --force', 'Overwrite existing configuration')
    .action(initCommand);

program
    .command('lookup <value>')
    .alias('l')
    .description('Look up an IOC (IP, domain, URL, hash, or email)')
    .option('-j, --json', 'Output as JSON')
    .option('-y, --yaml', 'Output as YAML')
    .option('-r, --raw', 'Raw output without formatting')
    .option('--related', 'Show related IOCs')
    .action(lookupCommand);

program
    .command('scan <file>')
    .alias('s')
    .description('Scan a file for indicators of compromise')
    .option('-f, --format <format>', 'Output format (json, yaml, table)', 'table')
    .option('-o, --output <file>', 'Save results to file')
    .option('--min-confidence <level>', 'Minimum confidence level (low, medium, high)', 'low')
    .option('--types <types>', 'IOC types to scan for (comma-separated)')
    .action(scanCommand);

program
    .command('detections')
    .alias('d')
    .description('Manage detections')
    .argument('[action]', 'Action to perform (list, get, update)', 'list')
    .option('-i, --id <id>', 'Detection ID for get/update actions')
    .option('-s, --status <status>', 'Filter by status')
    .option('--severity <severity>', 'Filter by severity')
    .option('-l, --limit <number>', 'Maximum number of results', '50')
    .option('-j, --json', 'Output as JSON')
    .action(detectionsCommand);

program
    .command('hosts')
    .alias('h')
    .description('Manage hosts')
    .argument('[action]', 'Action to perform (list, get, isolate, unisolate)', 'list')
    .option('-i, --id <id>', 'Host ID')
    .option('--platform <platform>', 'Filter by platform (windows, linux, macos)')
    .option('--state <state>', 'Filter by state (online, offline, isolated)')
    .option('-l, --limit <number>', 'Maximum number of results', '50')
    .option('-j, --json', 'Output as JSON')
    .action(hostsCommand);

program
    .command('incidents')
    .alias('i')
    .description('Manage incidents')
    .argument('[action]', 'Action to perform (list, get, create, update)', 'list')
    .option('-i, --id <id>', 'Incident ID')
    .option('-s, --status <status>', 'Filter by status')
    .option('--severity <severity>', 'Filter by severity')
    .option('-t, --title <title>', 'Incident title (for create)')
    .option('-d, --description <desc>', 'Incident description (for create)')
    .option('-l, --limit <number>', 'Maximum number of results', '50')
    .option('-j, --json', 'Output as JSON')
    .action(incidentsCommand);

program
    .command('threat-intel')
    .alias('ti')
    .description('Threat intelligence operations')
    .argument('[action]', 'Action to perform (actors, techniques, iocs)', 'actors')
    .option('-n, --name <name>', 'Threat actor name to search')
    .option('-i, --id <id>', 'Technique ID')
    .option('-l, --limit <number>', 'Maximum number of results', '50')
    .option('-j, --json', 'Output as JSON')
    .action(threatIntelCommand);

program
    .command('config')
    .description('Manage CLI configuration')
    .argument('[action]', 'Action to perform (show, set, reset)', 'show')
    .option('-k, --api-key <key>', 'Set API key')
    .option('-u, --api-url <url>', 'Set API URL')
    .action(configCommand);

program
    .command('version')
    .alias('-v')
    .description('Show version information')
    .action(versionCommand);

// Handle unknown commands
program.on('command:*', () => {
    console.error(chalk.red(`Invalid command: ${program.args.join(' ')}`));
    console.log(chalk.yellow('See --help for available commands'));
    process.exit(1);
});

// Parse arguments
program.parse(process.argv);

// Show help if no command provided
if (!process.argv.slice(2).length) {
    program.outputHelp();
}