import chalk from 'chalk';

export async function versionCommand(): Promise<void> {
    const version = require('../../package.json').version;

    console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
    console.log(chalk.blue.bold('║           V-SENTINEL CLI                 ║'));
    console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

    console.log(chalk.cyan('Version:'), version);
    console.log(chalk.cyan('Node.js:'), process.version);
    console.log(chalk.cyan('Platform:'), process.platform);
    console.log(chalk.cyan('Arch:'), process.arch);

    console.log(chalk.gray('\nRepository: https://github.com/vantis-ai/v-sentinel'));
    console.log(chalk.gray('Documentation: https://docs.vantis.ai/v-sentinel'));
    console.log();
}