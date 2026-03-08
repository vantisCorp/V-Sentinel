import chalk from 'chalk';
import ConfigStore from 'configstore';

const config = new ConfigStore('vsentinel');

export async function configCommand(
    action: string,
    options: {
        apiKey?: string;
        apiUrl?: string;
    }
): Promise<void> {
    switch (action) {
        case 'show':
            showConfig();
            break;
        case 'set':
            setConfig(options);
            break;
        case 'reset':
            resetConfig();
            break;
        default:
            console.error(chalk.red(`Unknown action: ${action}`));
            console.log(chalk.yellow('Available actions: show, set, reset'));
    }
}

function showConfig(): void {
    console.log(chalk.blue.bold('\n╔══════════════════════════════════════════╗'));
    console.log(chalk.blue.bold('║         V-SENTINEL CONFIGURATION         ║'));
    console.log(chalk.blue.bold('╚══════════════════════════════════════════╝\n'));

    console.log(chalk.cyan('API URL:'), config.get('apiUrl') || 'Not set');
    console.log(chalk.cyan('API Key:'), config.get('apiKey') ? '••••••••' + config.get('apiKey').slice(-4) : 'Not set');
    console.log(chalk.cyan('Timeout:'), config.get('timeout') || '30 seconds');
    console.log(chalk.cyan('Verify SSL:'), config.get('verifySSL') !== false ? 'Yes' : 'No');
    
    console.log(chalk.gray('\nConfig file:'), config.path);
    console.log();
}

function setConfig(options: { apiKey?: string; apiUrl?: string }): void {
    if (options.apiKey) {
        config.set('apiKey', options.apiKey);
        console.log(chalk.green('✓ API key updated'));
    }

    if (options.apiUrl) {
        try {
            new URL(options.apiUrl);
            config.set('apiUrl', options.apiUrl);
            console.log(chalk.green('✓ API URL updated'));
        } catch {
            console.error(chalk.red('Invalid URL format'));
        }
    }

    if (!options.apiKey && !options.apiUrl) {
        console.log(chalk.yellow('No configuration options provided.'));
        console.log(chalk.gray('Usage: vsentinel config set --api-key <key> --api-url <url>'));
    }
}

function resetConfig(): void {
    config.clear();
    console.log(chalk.green('✓ Configuration reset to defaults'));
}