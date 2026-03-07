import inquirer from 'inquirer';
import chalk from 'chalk';
import ConfigStore from 'configstore';
import ora from 'ora';

const config = new ConfigStore('vsentinel');

export async function initCommand(options: { force?: boolean }): Promise<void> {
    console.log(chalk.blue.bold('\n🛡️  V-Sentinel CLI Configuration\n'));

    if (config.has('apiKey') && !options.force) {
        console.log(chalk.yellow('Configuration already exists. Use --force to overwrite.'));
        console.log(chalk.gray(`Current API URL: ${config.get('apiUrl')}`));
        return;
    }

    const answers = await inquirer.prompt([
        {
            type: 'input',
            name: 'apiUrl',
            message: 'V-Sentinel API URL:',
            default: 'https://api.vantis.ai/v1',
            validate: (input: string) => {
                try {
                    new URL(input);
                    return true;
                } catch {
                    return 'Please enter a valid URL';
                }
            }
        },
        {
            type: 'password',
            name: 'apiKey',
            message: 'API Key:',
            mask: '*',
            validate: (input: string) => input.length > 0 || 'API key is required'
        },
        {
            type: 'number',
            name: 'timeout',
            message: 'Request timeout (seconds):',
            default: 30
        },
        {
            type: 'confirm',
            name: 'verifySSL',
            message: 'Verify SSL certificates?',
            default: true
        }
    ]);

    const spinner = ora('Saving configuration...').start();

    try {
        config.set('apiUrl', answers.apiUrl);
        config.set('apiKey', answers.apiKey);
        config.set('timeout', answers.timeout);
        config.set('verifySSL', answers.verifySSL);

        spinner.succeed(chalk.green('Configuration saved successfully!'));
        
        console.log(chalk.gray('\nConfiguration stored at:'), config.path);
        console.log(chalk.gray('\nYou can now use the V-Sentinel CLI commands.'));
        console.log(chalk.gray('Run `vsentinel --help` to see available commands.\n'));
    } catch (error) {
        spinner.fail(chalk.red('Failed to save configuration'));
        console.error(error);
    }
}